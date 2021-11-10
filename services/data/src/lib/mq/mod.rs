use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MQMessage<'a, T = Value> {
	pub id: &'a str,
	pub pattern: &'a str,
	pub data: T,
}
