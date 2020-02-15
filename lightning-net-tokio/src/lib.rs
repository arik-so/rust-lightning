use secp256k1::key::PublicKey;

use tokio::net::TcpStream;
use tokio::{io, time};
use tokio::sync::{mpsc, oneshot};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};

use lightning::ln::peer_handler;
use lightning::ln::peer_handler::SocketDescriptor as LnSocketTrait;
use lightning::ln::msgs::ChannelMessageHandler;

use std::task;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use std::hash::Hash;

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// A connection to a remote peer. Can be constructed either as a remote connection using
/// Connection::setup_outbound
pub struct Connection {
	writer: Option<io::WriteHalf<TcpStream>>,
	event_notify: mpsc::Sender<()>,
	// Because our PeerManager is templated by user-provided types, and we can't (as far as I can
	// tell) have a const RawWakerVTable built out of templated functions, we need some indirection
	// between being woken up with write-ready and calling PeerManager::write_event. This provides
	// that indirection, with a Sender which gets handed to the PeerManager Arc on the
	// schedule_read stack.
	//
	// An alternative (likely more effecient) approach would involve creating a RawWakerVTable at
	// runtime with functions templated by the Arc<PeerManager> type, calling write_event directly
	// from tokio's write wake, however doing so would require more unsafe voodo than I really feel
	// like writing.
	write_avail: mpsc::Sender<()>,
	// When we are told by rust-lightning to pause read (because we have writes backing up), we do
	// so by setting read_paused. If the read thread thereafter reads some data, it will place a
	// Sender here and then block on it.
	read_blocker: Option<oneshot::Sender<()>>,
	read_paused: bool,
	// If we get disconnected via SocketDescriptor::disconnect_socket(), we don't call
	// disconnect_event(), but if we get an Err return value out of PeerManager, in general, we do.
	// We track here whether we'll need to call disconnect_event() after the socket closes.
	need_disconnect_event: bool,
	disconnect: bool,
	id: u64,
}
impl Connection {
	async fn schedule_read<CMH: ChannelMessageHandler + 'static>(peer_manager: Arc<peer_handler::PeerManager<SocketDescriptor, Arc<CMH>>>, us: Arc<Mutex<Self>>, mut reader: io::ReadHalf<TcpStream>, mut write_event: mpsc::Receiver<()>) {
		let peer_manager_ref = peer_manager.clone();
		let mut buf = [0; 8192];
		loop {
			macro_rules! shutdown_socket {
				($err: expr) => { {
					println!("Disconnecting peer due to {}!", $err);
					break;
				} }
			}

			// Whenever we want to block, we have to at least select with the write_event Receiver,
			// which is used by the SocketDescriptor to wake us up if we need to shut down the
			// socket or if we need to generate a write_event.
			macro_rules! select_write_ev {
				($v: expr) => { {
					assert!($v.is_some()); // We can't have dropped the sending end, its in the us Arc!
					if us.lock().unwrap().disconnect {
						shutdown_socket!("disconnect_socket() call from RL");
					}
					if let Err(e) = peer_manager.write_event(&mut SocketDescriptor::new(us.clone())) {
						us.lock().unwrap().need_disconnect_event = false;
						shutdown_socket!(e);
					}
				} }
			}

			tokio::select! {
				v = write_event.recv() => select_write_ev!(v),
				read = reader.read(&mut buf) => match read {
					Ok(0) => {
						println!("Connection closed");
						break;
					},
					Ok(len) => {
						if let Some(blocker) = {
							let mut lock = us.lock().unwrap();
							if lock.disconnect {
								shutdown_socket!("disconnect_socket() call from RL");
							}
							if lock.read_paused {
								let (sender, blocker) = oneshot::channel();
								lock.read_blocker = Some(sender);
								Some(blocker)
							} else { None }
						} {
							tokio::select! {
								res = blocker => {
									res.unwrap(); // We should never drop the sender without sending () into it!
									if us.lock().unwrap().disconnect {
										shutdown_socket!("disconnect_socket() call from RL");
									}
								},
								v = write_event.recv() => select_write_ev!(v),
							}
						}
						match peer_manager.read_event(&mut SocketDescriptor::new(Arc::clone(&us)), &buf[0..len]) {
							Ok(pause_read) => {
								if pause_read {
									let mut lock = us.lock().unwrap();
									lock.read_paused = true;
								}

								match us.lock().unwrap().event_notify.try_send(()) {
									Ok(_) => {},
									Err(mpsc::error::TrySendError::Full(_)) => {
										// Ignore full errors as we just need them to poll after this point, so if the user
										// hasn't received the last send yet, it doesn't matter.
									},
									_ => panic!()
								}
							},
							Err(e) => {
								us.lock().unwrap().need_disconnect_event = false;
								shutdown_socket!(e)
							},
						}
					},
					Err(e) => {
						println!("Connection closed: {}", e);
						break;
					},
				},
			}
		}
		let writer_option = us.lock().unwrap().writer.take();
		if let Some(mut writer) = writer_option {
			// If the socket is already closed, shutdown() will fail, so just ignore it.
			let _ = writer.shutdown().await;
		}
		if us.lock().unwrap().need_disconnect_event {
			peer_manager_ref.disconnect_event(&SocketDescriptor::new(Arc::clone(&us)));
			match us.lock().unwrap().event_notify.try_send(()) {
				Ok(_) => {},
				Err(mpsc::error::TrySendError::Full(_)) => {
					// Ignore full errors as we just need them to poll after this point, so if the user
					// hasn't received the last send yet, it doesn't matter.
				},
				_ => panic!()
			}
		}
	}

	fn new(event_notify: mpsc::Sender<()>, stream: TcpStream) -> (io::ReadHalf<TcpStream>, mpsc::Receiver<()>, Arc<Mutex<Self>>) {
		// We only ever need a channel of depth 1 here: if we returned a non-full write to the
		// PeerManager, we will eventually get notified that there is room in the socket to write
		// new bytes, which will generate an event. That event will be popped off the queue before
		// we call write_event, ensuring that we have room to push a new () if, during the
		// write_event() call, send_data() returns a non-full write.
		let (write_avail, receiver) = mpsc::channel(1);
		let (reader, writer) = io::split(stream);

		(reader, receiver,
		Arc::new(Mutex::new(Self {
			writer: Some(writer), event_notify, write_avail,
			read_blocker: None, read_paused: false, need_disconnect_event: true, disconnect: false,
			id: ID_COUNTER.fetch_add(1, Ordering::AcqRel)
		})))
	}

	/// Process incoming messages and feed outgoing messages on the provided socket generated by
	/// accepting an incoming connection (by scheduling futures with tokio::spawn).
	///
	/// You should poll the Receive end of event_notify and call get_and_clear_pending_events() on
	/// ChannelManager and ChannelMonitor objects.
	pub async fn setup_inbound<CMH: ChannelMessageHandler + 'static>(peer_manager: Arc<peer_handler::PeerManager<SocketDescriptor, Arc<CMH>>>, event_notify: mpsc::Sender<()>, stream: TcpStream) {
		let (reader, receiver, us) = Self::new(event_notify, stream);

		if let Ok(_) = peer_manager.new_inbound_connection(SocketDescriptor::new(us.clone())) {
			if let Err(e) = tokio::spawn(Self::schedule_read(peer_manager, us, reader, receiver)).await {
				assert!(e.is_cancelled());
			}
		}
	}

	/// Process incoming messages and feed outgoing messages on the provided socket generated by
	/// making an outbound connection which is expected to be accepted by a peer with the given
	/// public key (by scheduling futures with tokio::spawn).
	///
	/// You should poll the Receive end of event_notify and call get_and_clear_pending_events() on
	/// ChannelManager and ChannelMonitor objects.
	pub async fn setup_outbound<CMH: ChannelMessageHandler + 'static>(peer_manager: Arc<peer_handler::PeerManager<SocketDescriptor, Arc<CMH>>>, event_notify: mpsc::Sender<()>, their_node_id: PublicKey, stream: TcpStream) {
		let (reader, receiver, us) = Self::new(event_notify, stream);

		if let Ok(initial_send) = peer_manager.new_outbound_connection(their_node_id, SocketDescriptor::new(us.clone())) {
			if SocketDescriptor::new(us.clone()).send_data(&initial_send, true) == initial_send.len() {
				if let Err(e) = tokio::spawn(Self::schedule_read(peer_manager, us, reader, receiver)).await {
					assert!(e.is_cancelled());
				}
			} else {
				// Note that we will skip disconnect_event here, in accordance with the PeerManager
				// requirements, as disconnect_event is called by the schedule_read Future.
				println!("Failed to write first full message to socket!");
			}
		}
	}

	/// Process incoming messages and feed outgoing messages on a new connection made to the given
	/// socket address which is expected to be accepted by a peer with the given public key (by
	/// scheduling futures with tokio::spawn).
	///
	/// You should poll the Receive end of event_notify and call get_and_clear_pending_events() on
	/// ChannelManager and ChannelMonitor objects.
	pub async fn connect_outbound<CMH: ChannelMessageHandler + 'static>(peer_manager: Arc<peer_handler::PeerManager<SocketDescriptor, Arc<CMH>>>, event_notify: mpsc::Sender<()>, their_node_id: PublicKey, addr: SocketAddr) {
		let connect_timeout = time::delay_for(Duration::from_secs(10));
		let connect_fut = TcpStream::connect(&addr);
		tokio::select! {
			_ = connect_timeout => { },
			res = connect_fut => {
				if let Ok(stream) = res {
					Connection::setup_outbound(peer_manager, event_notify, their_node_id, stream).await;
				}
			},
		};
	}
}

const SOCK_WAKER_VTABLE: task::RawWakerVTable =
	task::RawWakerVTable::new(clone_socket_waker, wake_socket_waker, wake_socket_waker_by_ref, drop_socket_waker);

fn clone_socket_waker(orig_ptr: *const ()) -> task::RawWaker {
	descriptor_to_waker(orig_ptr as *const SocketDescriptor)
}
fn wake_socket_waker(orig_ptr: *const ()) {
	wake_socket_waker_by_ref(orig_ptr);
	drop_socket_waker(orig_ptr);
}
fn wake_socket_waker_by_ref(orig_ptr: *const ()) {
	let descriptor = orig_ptr as *const SocketDescriptor;
	// An error should be fine. Most likely we got two send_datas in a row, both of which failed to
	// fully write, but we only need to provide a write_event() once. Otherwise, the sending thread
	// may have already gone away due to a socket close, in which case there's nothing to wake up
	// anyway.
	let mut us = unsafe { (*descriptor).conn.lock() }.unwrap();
	let _ = us.write_avail.try_send(());
}
fn drop_socket_waker(orig_ptr: *const ()) {
	let _orig_box = unsafe { Box::from_raw(orig_ptr as *mut SocketDescriptor) };
	// _orig_box is now dropped
}
fn descriptor_to_waker(descriptor: *const SocketDescriptor) -> task::RawWaker {
	let new_box = Box::leak(Box::new(unsafe { (*descriptor).clone() }));
	let new_ptr = new_box as *const SocketDescriptor;
	task::RawWaker::new(new_ptr as *const (), &SOCK_WAKER_VTABLE)
}

pub struct SocketDescriptor {
	conn: Arc<Mutex<Connection>>,
	id: u64,
}
impl SocketDescriptor {
	fn new(conn: Arc<Mutex<Connection>>) -> Self {
		let id = conn.lock().unwrap().id;
		Self { conn, id }
	}
}
impl peer_handler::SocketDescriptor for SocketDescriptor {
	fn send_data(&mut self, data: &[u8], resume_read: bool) -> usize {
		let mut us = self.conn.lock().unwrap();
		if us.writer.is_none() {
			// The writer gets take()n when its time to shut down, so just fast-return 0 here.
			return 0;
		}

		if resume_read {
			if let Some(sender) = us.read_blocker.take() {
				sender.send(()).unwrap();
			}
			us.read_paused = false;
		}
		if data.is_empty() { return 0; }
		let waker = unsafe { task::Waker::from_raw(descriptor_to_waker(self)) };
		let mut ctx = task::Context::from_waker(&waker);
		let mut written_len = 0;
		loop {
			match std::pin::Pin::new(us.writer.as_mut().unwrap()).poll_write(&mut ctx, &data[written_len..]) {
				task::Poll::Ready(Ok(res)) => {
					// The tokio docs *seem* to indicate this can't happen, and I certainly don't
					// know how to handle it if it does (cause it should be a Poll::Pending
					// instead):
					assert_ne!(res, 0);
					written_len += res;
					if written_len == data.len() { return written_len; }
				},
				task::Poll::Ready(Err(e)) => {
					// The tokio docs *seem* to indicate this can't happen, and I certainly don't
					// know how to handle it if it does (cause it should be a Poll::Pending
					// instead):
					assert_ne!(e.kind(), io::ErrorKind::WouldBlock);
					// Probably we've already been closed, just return what we have and let the
					// read thread handle closing logic.
					return written_len;
				},
				task::Poll::Pending => {
					// We're queued up for a write event now, but we need to make sure we also
					// pause read given we're now waiting on the remote end to ACK (and in
					// accordance with the send_data() docs).
					us.read_paused = true;
					return written_len;
				},
			}
		}
	}

	fn disconnect_socket(&mut self) {
		let mut us = self.conn.lock().unwrap();
		us.need_disconnect_event = false;
		us.disconnect = true;
		us.read_paused = true;
		// Wake up the sending thread, assuming its still alive
		let _ = us.write_avail.try_send(());
		// TODO: There's a race where we don't meet the requirements of disconnect_socket if the
		// read task is about to call a PeerManager function (eg read_event or write_event).
		// Ideally we need to release the us lock and block until we have confirmation from the
		// read task that it has broken out of its main loop.
	}
}
impl Clone for SocketDescriptor {
	fn clone(&self) -> Self {
		Self {
			conn: Arc::clone(&self.conn),
			id: self.id,
		}
	}
}
impl Eq for SocketDescriptor {}
impl PartialEq for SocketDescriptor {
	fn eq(&self, o: &Self) -> bool {
		self.id == o.id
	}
}
impl Hash for SocketDescriptor {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

