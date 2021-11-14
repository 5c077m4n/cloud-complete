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
const ORDER_RESPONSE_QUEUE: &str = "order_response_queue";

pub async fn handle_order_requests(
	rmq_conn: &lapin::Connection,
	db_conn: &mongodb::Database,
) -> Result<(), ErrorType> {
	let order_collection = db_conn.collection::<Order>("order");

	let order_rx = rmq_conn.create_channel().await?;
	let order_tx = rmq_conn.create_channel().await?;
	debug!("{:?}", rmq_conn.status().state());

	let _ = order_rx
		.queue_declare(
			ORDER_REQUEST_QUEUE,
			QueueDeclareOptions::default(),
			FieldTable::default(),
		)
		.await?;
	let _ = order_tx
		.queue_declare(
			ORDER_RESPONSE_QUEUE,
			QueueDeclareOptions::default(),
			FieldTable::default(),
		)
		.await?;
	let mut order_consumer = order_rx
		.basic_consume(
			ORDER_REQUEST_QUEUE,
			"order_request_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(Ok((_rx_channel, delivery))) = order_consumer.next().await {
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

					let _confirm = order_tx
						.basic_publish(
							"",
							ORDER_RESPONSE_QUEUE,
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
