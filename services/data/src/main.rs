#![forbid(unsafe_code)]

mod lib;

use std::{env, str};

use futures::{stream::StreamExt, TryStreamExt};
use lapin::{
	options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
	types::FieldTable,
	BasicProperties, Connection, ConnectionProperties,
};
use log::{debug, error};
use mongodb::{bson::doc, options::ClientOptions, Client};

use lib::{ErrorType, MQMessage, Order, Ticket};

#[tokio::main]
async fn main() -> Result<(), ErrorType> {
	env_logger::init();
	debug!("Starting up...");

	let mongodb_url = {
		let mongodb_host = env::var("MONGODB_HOST").unwrap_or_else(|_| "0.0.0.0".into());
		let mongodb_port: usize = env::var("MONGODB_PORT")
			.unwrap_or_else(|_| "27017".into())
			.parse()?;
		format!("mongodb://{}:{}", &mongodb_host, &mongodb_port)
	};

	let mut client_options = ClientOptions::parse(&mongodb_url).await?;
	client_options.app_name = Some("cloud_complete_data".into());
	let client = Client::with_options(client_options)?;
	debug!(
		"Successfully connected to the MongoDB instance @ {}",
		&mongodb_url
	);

	let db = client.database("cloud_complete_data");
	let _ticket_collection = db.collection::<Ticket>("ticket");
	let _order_collection = db.collection::<Order>("order");

	let rabbitmq_url = {
		let rabbitmq_host = env::var("RABBITMQ_HOST").unwrap_or_else(|_| "0.0.0.0".into());
		let rabbitmq_port: usize = env::var("RABBITMQ_PORT")
			.unwrap_or_else(|_| "5672".into())
			.parse()?;
		format!("amqp://{}:{}/%2f", &rabbitmq_host, &rabbitmq_port)
	};

	let connection = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
	debug!(
		"Successfully connected to the RabbitMQ instance @ {}",
		&rabbitmq_url
	);
	let rx = connection.create_channel().await?;
	let tx = connection.create_channel().await?;
	debug!("{:?}", connection.status().state());

	let _tx_queue = tx
		.queue_declare(
			"data_queue",
			QueueDeclareOptions::default(),
			FieldTable::default(),
		)
		.await?;
	let mut consumer = rx
		.basic_consume(
			"data_queue",
			"data_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(Ok((_rx_channel, delivery))) = consumer.next().await {
		delivery.ack(BasicAckOptions::default()).await?;

		match serde_json::from_slice::<MQMessage<&str>>(&delivery.data) {
			Ok(message) => {
				match message.pattern {
					"get_tickets" => {
						match serde_json::from_str::<Vec<&str>>(&message.data) {
							Ok(id_list) => {
								let filter = if id_list.is_empty() {
									None
								} else {
									Some(doc! { "_id": doc! { "$in": id_list } })
								};

								let mut cursor = _ticket_collection.find(filter, None).await?;
								let mut tickets: Vec<Ticket> = Vec::new();
								while let Some(ticket) = cursor.try_next().await? {
									tickets.push(ticket);
								}

								let _confirm = tx
									.basic_publish(
										"",
										"data_queue",
										BasicPublishOptions::default(),
										serde_json::to_vec(&tickets)?,
										BasicProperties::default(),
									)
									.await?
									.await?;
							}
							Err(e) => {
								error!("{} - {}", &e, message.data);
								continue;
							}
						};
					}
					other => error!(r#"The pattern "{}" is not supported"#, other),
				};
			}
			Err(e) => {
				error!("{} - {:?}", &e, str::from_utf8(&delivery.data));
				continue;
			}
		};
	}

	Ok(())
}
