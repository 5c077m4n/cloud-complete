use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
	_id: Option<String>,
	title: String,
	ticket_ref: String,
	date: String,
}
