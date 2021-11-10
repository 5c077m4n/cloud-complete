use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
	pub _id: Option<String>,
	pub title: String,
	pub ticket_ref: Vec<String>,
	pub date: String,
}
