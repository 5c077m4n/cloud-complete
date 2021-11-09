use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TicketStatus {
    Free,
    Ordered,
    Canceled,
}
impl TicketStatus {
    #[allow(dead_code)]
    pub fn from(str: &str) -> Self {
        match str {
            "free" => Self::Free,
            "ordered" => Self::Ordered,
            "canceled" => Self::Canceled,
            _ => panic!(),
        }
    }
    #[allow(dead_code)]
    pub fn to_str(&self) -> String {
        match self {
            Self::Free => "free".into(),
            Self::Ordered => "ordered".into(),
            Self::Canceled => "canceled".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    _id: Option<String>,
	title: String,
	price: String,
    date: String,
    status: TicketStatus,
}
