extern crate lightning;

pub mod buffer;
pub mod chain;
pub mod channels;
pub mod error;
pub mod node;
pub mod peers;
mod util;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
