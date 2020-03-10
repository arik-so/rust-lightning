#[macro_use]
extern crate lazy_static;
extern crate lightning;

pub mod buffer;
pub mod error;
pub mod peers;
mod util;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
