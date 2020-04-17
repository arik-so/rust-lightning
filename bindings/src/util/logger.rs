use lightning::util::logger::Record;

pub struct Logger {

}

impl lightning::util::logger::Logger for Logger {
	fn log<'a>(&self, record: &Record<'a>) {
		unimplemented!()
	}
}