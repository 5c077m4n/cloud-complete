use futures::{StreamExt, TryStreamExt};
use lapin::{
	options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
	types::FieldTable,
	BasicProperties,
};
use log::{debug, error};
use mongodb::bson::doc;

use crate::lib::{ErrorType, MQMessage, Ticket};

const TICKET_REQUEST_QUEUE: &str = "ticket_request_queue";
const TICKET_RESPONSE_QUEUE: &str = "ticket_response_queue";

pub async fn handle_ticket_requests(
	rmq_conn: &lapin::Connection,
	db_conn: &mongodb::Database,
) -> Result<(), ErrorType> {
	let ticket_collection = db_conn.collection::<Ticket>("ticket");

	let ticket_rx = rmq_conn.create_channel().await?;
	let ticket_tx = rmq_conn.create_channel().await?;
	debug!("{:?}", rmq_conn.status().state());

	let _ = ticket_rx
		.queue_declare(
			TICKET_REQUEST_QUEUE,
			QueueDeclareOptions::default(),
			FieldTable::default(),
		)
		.await?;
	let _ = ticket_tx
		.queue_declare(
			TICKET_RESPONSE_QUEUE,
			QueueDeclareOptions::default(),
			FieldTable::default(),
		)
		.await?;
	let mut ticket_consumer = ticket_rx
		.basic_consume(
			TICKET_REQUEST_QUEUE,
			"ticket_request_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(Ok((_rx_channel, delivery))) = ticket_consumer.next().await {
		delivery.ack(BasicAckOptions::default()).await?;

		if let Ok(message) = serde_json::from_slice::<MQMessage<Vec<&str>>>(&delivery.data) {
			let id_list = &message.data;
			let filter = if id_list.is_empty() {
				None
			} else if id_list.len() == 1 {
				Some(doc! { "_id": id_list[0] })
			} else {
				Some(doc! { "_id": { "$in": id_list } })
			};

			match message.pattern {
				"get_tickets" => {
					debug!("Getting tickets with filter: {:?}", &filter);

					let cursor = ticket_collection.find(filter, None).await?;
					let tickets: Vec<Ticket> = cursor.try_collect().await?;
					debug!("Successfully fetched tickets: {:?}", &tickets);

					let response = MQMessage {
						id: message.id,
						pattern: "get_tickets_response",
						data: &tickets,
					};

					let _confirm = ticket_tx
						.basic_publish(
							"",
							TICKET_RESPONSE_QUEUE,
							BasicPublishOptions::default(),
							serde_json::to_vec(&response)?,
							BasicProperties::default(),
						)
						.await?
						.await?;
				}
				other => error!(r#"The pattern "{}" is not supported"#, other),
			};
		}
	}

	Ok(())
}
