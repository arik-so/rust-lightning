pub use bitcoin::io::*;

/// The `Seek` trait provides a cursor which can be moved within a stream of
/// bytes.
///
/// The stream typically has a fixed size, allowing seeking relative to either
/// end or the current offset.
pub trait Seek {
	/// Seek to an offset, in bytes, in a stream.
	///
	/// A seek beyond the end of a stream is allowed, but behavior is defined
	/// by the implementation.
	///
	/// If the seek operation completed successfully,
	/// this method returns the new position from the start of the stream.
	/// That position can be used later with [`SeekFrom::Start`].
	///
	/// # Errors
	///
	/// Seeking to a negative offset is considered an error.
	fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

/// Enumeration of possible methods to seek within an I/O object.
///
/// It is used by the [`Seek`] trait.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum SeekFrom {
	/// Sets the offset to the provided number of bytes.
	Start(u64),

	/// Sets the offset to the size of this object plus the specified number of
	/// bytes.
	///
	/// It is possible to seek beyond the end of an object, but it's an error to
	/// seek before byte 0.
	End(i64),

	/// Sets the offset to the current position plus the specified number of
	/// bytes.
	///
	/// It is possible to seek beyond the end of an object, but it's an error to
	/// seek before byte 0.
	Current(i64),
}

/// Adaptor to chain together two readers.
pub struct Chain<T, U> {
	first: T,
	second: U,
	done_first: bool,
}

impl<T: Read, U: Read> Read for Chain<T, U> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		if !self.done_first {
			match self.first.read(buf)? {
				0 if !buf.is_empty() => self.done_first = true,
				n => return Ok(n),
			}
		}
		self.second.read(buf)
	}
}

/// Emulation of std::io::Cursor
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cursor<T> {
	inner: T,
	pos: u64,
}

impl<T> Cursor<T> {
	/// Creates a new cursor wrapping the provided underlying in-memory buffer.
	///
	/// Cursor initial position is `0` even if underlying buffer (e.g., [`Vec`])
	/// is not empty. So writing to cursor starts with overwriting [`Vec`]
	/// content, not with appending to it.
	pub fn new(inner: T) -> Cursor<T> {
		Cursor { pos: 0, inner }
	}

	/// Consumes this cursor, returning the underlying value.
	pub fn into_inner(self) -> T {
		self.inner
	}

	/// Gets a reference to the underlying value in this cursor.
	pub fn get_ref(&self) -> &T {
		&self.inner
	}

	/// Gets a mutable reference to the underlying value in this cursor.
	///
	/// Care should be taken to avoid modifying the internal I/O state of the
	/// underlying value as it may corrupt this cursor's position.
	pub fn get_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	/// Returns the current position of this cursor.
	pub fn position(&self) -> u64 {
		self.pos
	}

	/// Sets the position of this cursor.
	pub fn set_position(&mut self, pos: u64) {
		self.pos = pos;
	}
}

impl<T> Seek for Cursor<T>
where
	T: AsRef<[u8]>,
{
	fn seek(&mut self, style: SeekFrom) -> Result<u64> {
		let (base_pos, offset) = match style {
			SeekFrom::Start(n) => {
				self.pos = n;
				return Ok(n);
			},
			SeekFrom::End(n) => (self.inner.as_ref().len() as u64, n),
			SeekFrom::Current(n) => (self.pos, n),
		};
		let new_pos = if offset >= 0 {
			base_pos.checked_add(offset as u64)
		} else {
			base_pos.checked_sub((offset.wrapping_neg()) as u64)
		};
		match new_pos {
			Some(n) => {
				self.pos = n;
				Ok(self.pos)
			},
			None => Err(Error::new(
				ErrorKind::InvalidInput,
				"invalid seek to a negative or overflowing position",
			)),
		}
	}
}

impl<T> Read for Cursor<T>
where
	T: AsRef<[u8]>,
{
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		let n = Read::read(&mut self.fill_buf()?, buf)?;
		self.pos += n as u64;
		Ok(n)
	}

	fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
		let n = buf.len();
		Read::read_exact(&mut self.fill_buf()?, buf)?;
		self.pos += n as u64;
		Ok(())
	}
}

impl<T> BufRead for Cursor<T>
where
	T: AsRef<[u8]>,
{
	fn fill_buf(&mut self) -> Result<&[u8]> {
		let amt = core::cmp::min(self.pos, self.inner.as_ref().len() as u64);
		Ok(&self.inner.as_ref()[(amt as usize)..])
	}
	fn consume(&mut self, amt: usize) {
		self.pos += amt as u64;
	}
}
