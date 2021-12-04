use futures::{StreamExt, TryStreamExt};
use lapin::{
	options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
	types::FieldTable,
	BasicProperties,
};
use log::{debug, error};
use mongodb::bson::{doc, Document};

use crate::lib::{ErrorType, MQMessage, Order};

const ORDER_REQUEST_QUEUE: &str = "order_request_queue";
const _ORDER_RESPONSE_QUEUE: &str = "order_response_queue";

pub async fn handle_order_requests(
	rmq_conn: &lapin::Connection,
	db_conn: &mongodb::Database,
) -> Result<(), ErrorType> {
	let order_collection = db_conn.collection::<Order>("order");

	let rx = rmq_conn.create_channel().await?;
	let tx = rmq_conn.create_channel().await?;

	let mut order_consumer = rx
		.basic_consume(
			ORDER_REQUEST_QUEUE,
			"order_request_consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await?;

	while let Some(Ok((_rx_channel, delivery))) = order_consumer.next().await {
		if let Some(reply_to) = delivery.properties.reply_to() {
			let reply_to = reply_to.as_str();

			if let Ok(message) = serde_json::from_slice::<MQMessage>(&delivery.data) {
				match message.pattern.as_str() {
					"get_orders" => {
						let id_list: Vec<String> = serde_json::from_value(message.data)?;
						let filter: Option<Document> = if id_list.is_empty() {
							None
						} else if id_list.len() == 1 {
							Some(doc! { "_id": &id_list[0] })
						} else {
							Some(doc! { "_id": { "$in": id_list } })
						};

						debug!("Getting orders with filter: {:?}", &filter);

						let cursor = order_collection.find(filter, None).await?;
						let orders: Vec<Order> = cursor.try_collect().await?;
						debug!("Successfully fetched orders: {:?}", &orders);

						let response = MQMessage {
							id: message.id,
							pattern: "get_orders_response".into(),
							data: &orders,
						};
						debug!("{:?}", &response);

						if let Some(correlation_id) = delivery.properties.correlation_id() {
							let correlation_id = correlation_id.clone();

							let _confirm = tx
								.basic_publish(
									"",
									reply_to,
									BasicPublishOptions::default(),
									serde_json::to_vec(&response)?,
									BasicProperties::default().with_correlation_id(correlation_id),
								)
								.await?
								.await?;

							delivery.ack(BasicAckOptions::default()).await?;
						} else {
							error!("There was no correlation id");
						}
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
