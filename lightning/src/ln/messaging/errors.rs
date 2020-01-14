use std::fmt::{Formatter, Error};

#[derive(Debug)]
pub (crate) struct UnexpectedTypeError {}

impl std::error::Error for UnexpectedTypeError {}

impl std::fmt::Display for UnexpectedTypeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		f.write_str("Unexpected type error")?;
		Ok(())
	}
}