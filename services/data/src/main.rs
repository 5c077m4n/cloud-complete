mod lib;

use std::{env, str};

use futures::stream::StreamExt;
use lapin::{
	options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
	types::FieldTable,
	Connection, ConnectionProperties,
};
use log::{debug, error, info};
use mongodb::{options::ClientOptions, Client};

use lib::{ErrorType, Order, Ticket};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), ErrorType> {
	env_logger::init();
	info!("Starting up...");

	let mongodb_url = {
		let mongodb_host = env::var("MONGODB_HOST").unwrap_or_else(|_| "0.0.0.0".into());
		let mongodb_port: usize = env::var("MONGODB_PORT")
			.unwrap_or_else(|_| "27017".into())
			.parse()?;
		format!("mongodb://{}:{}", &mongodb_host, &mongodb_port)
	};

	let mut client_options = ClientOptions::parse(&mongodb_url).await?;
	client_options.app_name = Some("Cloud Complete Data".into());
	let client = Client::with_options(client_options)?;
	info!(
		"Successfully connected to the MongoDB instance @ {}",
		&mongodb_url
	);

	let db = client.database("Cloud Complete Data");
	let _ticket_collection = db.collection::<Ticket>("ticket");
	let _order_collection = db.collection::<Order>("order");

	let rabbitmq_url = {
		let rabbitmq_host = env::var("RABBITMQ_HOST").unwrap_or_else(|_| "0.0.0.0".into());
		let rabbitmq_port: usize = env::var("RABBITMQ_PORT")
			.unwrap_or_else(|_| "5672".into())
			.parse()?;
		format!("amqp://{}:{}/%2f", &rabbitmq_host, &rabbitmq_port)
	};

	let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
	info!(
		"Successfully connected to the RabbitMQ instance @ {}",
		&rabbitmq_url
	);
	let channel = conn.create_channel().await?;

	let _queue = channel
		.queue_declare(
			"data_queue",
			QueueDeclareOptions::default(),
			FieldTable::default(),
		)
		.await?;
	let mut consumer = channel
		.basic_consume(
			"data_queue",
			"data_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(delivery) = consumer.next().await {
		let (_channel, delivery) = delivery?;
		delivery.ack(BasicAckOptions::default()).await?;

		let message = str::from_utf8(&delivery.data)?;
		let message: Value = match serde_json::from_str(message) {
			Ok(msg) => msg,
			Err(e) => {
				error!("{}", e);
				continue;
			}
		};
		debug!("{}", message);
	}

	Ok(())
}
