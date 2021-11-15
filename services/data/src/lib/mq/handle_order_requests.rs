use futures::{StreamExt, TryStreamExt};
use lapin::{
	options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
	types::FieldTable,
	BasicProperties,
};
use log::{debug, error};
use mongodb::bson::doc;

use crate::lib::{ErrorType, MQMessage, Order};

const ORDER_REQUEST_QUEUE: &str = "order_request_queue";

pub async fn handle_order_requests(
	rmq_conn: &lapin::Connection,
	db_conn: &mongodb::Database,
) -> Result<(), ErrorType> {
	let order_collection = db_conn.collection::<Order>("order");

	let rx = rmq_conn.create_channel().await?;
	let tx = rmq_conn.create_channel().await?;
	debug!("{:?}", rmq_conn.status().state());

	rx.queue_declare(
		ORDER_REQUEST_QUEUE,
		QueueDeclareOptions::default(),
		FieldTable::default(),
	)
	.await?;
	let mut order_consumer = rx
		.basic_consume(
			ORDER_REQUEST_QUEUE,
			"order_request_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(Ok((_rx_channel, delivery))) = order_consumer.next().await {
		delivery.ack(BasicAckOptions::default()).await?;

		if let Some(reply_to) = delivery.properties.reply_to() {
			let reply_to = reply_to.as_str();
			tx.queue_declare(
				reply_to,
				QueueDeclareOptions::default(),
				FieldTable::default(),
			)
			.await?;

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
					"get_orders" => {
						debug!("Getting orders with filter: {:?}", &filter);

						let cursor = order_collection.find(filter, None).await?;
						let orders: Vec<Order> = cursor.try_collect().await?;
						debug!("Successfully fetched orders: {:?}", &orders);

						let response = MQMessage {
							id: message.id,
							pattern: "get_orders_response",
							data: &orders,
						};

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
					other => error!(
						r#"[Order fetcher] The pattern "{}" is not supported"#,
						other
					),
				};
			}
		} else {
			error!("No `reply_to` queue specified");
		}
	}

	Ok(())
}
