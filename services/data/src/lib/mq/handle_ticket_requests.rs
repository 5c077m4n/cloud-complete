use futures::{StreamExt, TryStreamExt};
use lapin::{
	options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
	types::FieldTable,
	BasicProperties,
};
use log::{debug, error};
use mongodb::bson::{doc, Document};

use crate::lib::{ErrorType, MQMessage, Ticket};

const TICKET_REQUEST_QUEUE: &str = "ticket_request_queue";
const TICKET_RESPONSE_QUEUE: &str = "ticket_response_queue";

pub async fn handle_ticket_requests(
	rmq_conn: &lapin::Connection,
	db_conn: &mongodb::Database,
) -> Result<(), ErrorType> {
	let ticket_collection = db_conn.collection::<Ticket>("ticket");

	let rx = rmq_conn.create_channel().await?;
	let tx = rmq_conn.create_channel().await?;

	rx.queue_declare(
		TICKET_REQUEST_QUEUE,
		QueueDeclareOptions::default(),
		FieldTable::default(),
	)
	.await?;
	tx.queue_declare(
		TICKET_RESPONSE_QUEUE,
		QueueDeclareOptions::default(),
		FieldTable::default(),
	)
	.await?;
	let mut ticket_consumer = rx
		.basic_consume(
			TICKET_REQUEST_QUEUE,
			"ticket_request_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(Ok((_rx_channel, delivery))) = ticket_consumer.next().await {
		delivery.ack(BasicAckOptions::default()).await?;

		if let Some(reply_to) = delivery.properties.reply_to() {
			let reply_to = reply_to.as_str();
			tx.queue_declare(
				reply_to,
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await?;

			if let Ok(message) = serde_json::from_slice::<MQMessage>(&delivery.data) {
				match message.pattern.as_str() {
					"get_tickets" => {
						let id_list: Vec<String> = serde_json::from_value(message.data)?;
						let filter: Option<Document> = if id_list.is_empty() {
							None
						} else if id_list.len() == 1 {
							Some(doc! { "_id": &id_list[0] })
						} else {
							Some(doc! { "_id": { "$in": id_list } })
						};

						debug!("Getting tickets with filter: {:?}", &filter);

						let cursor = ticket_collection.find(filter, None).await?;
						let tickets: Vec<Ticket> = cursor.try_collect().await?;
						debug!("Successfully fetched tickets: {:?}", &tickets);

						let response = MQMessage {
							id: message.id,
							pattern: "get_tickets_response".into(),
							data: &tickets,
						};
						debug!("{:?}", &response);

						let _confirm = tx
							.basic_publish(
								"",
								reply_to,
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
		} else {
			error!("No `reply_to` queue specified: {:?}", &delivery);
		}
	}

	Ok(())
}
