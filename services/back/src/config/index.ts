import { Transport, RmqOptions } from '@nestjs/microservices';

export enum RmqQueues {
	TICKET_REQUEST = 'ticket_request_queue',
	TICKET_RESPONSE = 'ticket_response_queue',
	ORDER_REQUEST = 'order_request_queue',
	ORDER_RESPONSE = 'order_response_queue',
}

export function rmqConfig(): Record<string, RmqOptions> {
	const {
		RABBITMQ_USERNAME = 'guest',
		RABBITMQ_PASSWORD = 'guest',
		RABBITMQ_HOST = '0.0.0.0',
		RABBITMQ_PORT = 5672,
	} = process.env;

	return {
		ticketRequestQueue: {
			transport: Transport.RMQ,
			options: {
				urls: [`amqp://${RABBITMQ_USERNAME}:${RABBITMQ_PASSWORD}@${RABBITMQ_HOST}:${RABBITMQ_PORT}`],
				queue: RmqQueues.TICKET_REQUEST,
				queueOptions: { durable: false, noAck: true },
				replyQueue: RmqQueues.TICKET_RESPONSE,
			},
		},
		orderRequestQueue: {
			transport: Transport.RMQ,
			options: {
				urls: [`amqp://${RABBITMQ_USERNAME}:${RABBITMQ_PASSWORD}@${RABBITMQ_HOST}:${RABBITMQ_PORT}`],
				queue: RmqQueues.ORDER_REQUEST,
				queueOptions: { durable: false, noAck: true },
				replyQueue: RmqQueues.ORDER_RESPONSE,
			},
		},
		ticketResponseQueue: {
			transport: Transport.RMQ,
			options: {
				urls: [`amqp://${RABBITMQ_USERNAME}:${RABBITMQ_PASSWORD}@${RABBITMQ_HOST}:${RABBITMQ_PORT}`],
				queue: RmqQueues.TICKET_RESPONSE,
				queueOptions: { durable: false, noAck: true },
			},
		},
		orderResponseQueue: {
			transport: Transport.RMQ,
			options: {
				urls: [`amqp://${RABBITMQ_USERNAME}:${RABBITMQ_PASSWORD}@${RABBITMQ_HOST}:${RABBITMQ_PORT}`],
				queue: RmqQueues.ORDER_RESPONSE,
				queueOptions: { durable: false, noAck: true },
				replyQueue: RmqQueues.ORDER_RESPONSE,
			},
		},
	};
}
