use std::{error, fmt};

#[derive(Debug)]
pub enum ErrorType {
	Io(std::io::Error),
	RabbitMQ(lapin::Error),
	MongoDB(mongodb::error::Error),
	ParseInt(std::num::ParseIntError),
	ParseStr(std::str::Utf8Error),
	ParseJson(serde_json::Error),
}

impl error::Error for ErrorType {}

impl From<std::io::Error> for ErrorType {
	fn from(err: std::io::Error) -> Self {
		ErrorType::Io(err)
	}
}
impl From<lapin::Error> for ErrorType {
	fn from(err: lapin::Error) -> Self {
		ErrorType::RabbitMQ(err)
	}
}
impl From<mongodb::error::Error> for ErrorType {
	fn from(err: mongodb::error::Error) -> Self {
		ErrorType::MongoDB(err)
	}
}
impl From<std::num::ParseIntError> for ErrorType {
	fn from(parse_int_error: std::num::ParseIntError) -> Self {
		Self::ParseInt(parse_int_error)
	}
}
impl From<std::str::Utf8Error> for ErrorType {
	fn from(parse_str_error: std::str::Utf8Error) -> Self {
		Self::ParseStr(parse_str_error)
	}
}
impl From<serde_json::Error> for ErrorType {
	fn from(parse_json_error: serde_json::Error) -> Self {
		Self::ParseJson(parse_json_error)
	}
}

impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(io_error) => write!(f, "{}", io_error),
			Self::RabbitMQ(rabbitmq_error) => write!(f, "{}", rabbitmq_error),
			Self::MongoDB(mongo_error) => write!(f, "{}", mongo_error),
			Self::ParseInt(parse_int_error) => write!(f, "{}", parse_int_error),
			Self::ParseStr(parse_str_error) => write!(f, "{}", parse_str_error),
			Self::ParseJson(parse_json_error) => write!(f, "{}", parse_json_error),
		}
	}
}
