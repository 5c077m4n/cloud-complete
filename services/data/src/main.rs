#![forbid(unsafe_code)]

mod lib;

use std::env;

use log::debug;
use mongodb::options::ClientOptions;

use lib::{handle_order_requests, handle_ticket_requests, ErrorType};

#[tokio::main]
async fn main() -> Result<(), ErrorType> {
	env_logger::init();
	debug!("Starting up...");

	let mongodb_url = {
		let host = env::var("MONGODB_HOST").unwrap_or_else(|_| "0.0.0.0".into());
		let port: usize = env::var("MONGODB_PORT")
			.unwrap_or_else(|_| "27017".into())
			.parse()?;
		format!("mongodb://{}:{}", &host, &port)
	};

	let mut client_options = ClientOptions::parse(&mongodb_url).await?;
	client_options.app_name = Some("cloud_complete_data".into());
	let mongodb_client = mongodb::Client::with_options(client_options)?;
	let db = mongodb_client.database("cloud_complete_data");
	debug!(
		"Successfully connected to the MongoDB instance @ {}",
		&mongodb_url
	);

	let rabbitmq_url = {
		let host = env::var("RABBITMQ_HOST").unwrap_or_else(|_| "0.0.0.0".into());
		let port: usize = env::var("RABBITMQ_PORT")
			.unwrap_or_else(|_| "5672".into())
			.parse()?;
		let username = env::var("RABBITMQ_USERNAME").unwrap_or_else(|_| "guest".into());
		let password = env::var("RABBITMQ_PASSWORD").unwrap_or_else(|_| "guest".into());
		format!("amqp://{}:{}@{}:{}/%2f", &username, &password, &host, &port)
	};

	let rmq_conn =
		lapin::Connection::connect(&rabbitmq_url, lapin::ConnectionProperties::default()).await?;
	debug!(
		"Successfully connected to the RabbitMQ instance @ {}",
		&rabbitmq_url
	);

	let (ticket_res, order_res) = tokio::join!(
		handle_ticket_requests(&rmq_conn, &db),
		handle_order_requests(&rmq_conn, &db)
	);
	ticket_res?;
	order_res?;

	unreachable!();
}
