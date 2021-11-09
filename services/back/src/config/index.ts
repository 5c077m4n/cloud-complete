import { Transport } from '@nestjs/microservices';

export function rabbitConfig() {
	const { RABBITMQ_HOST = '0.0.0.0', RABBITMQ_PORT = 5672 } = process.env;

	return {
		rabbitmqData: {
			name: 'DATA_SERVICE',
			transport: Transport.RMQ,
			options: {
				urls: [`amqp://${RABBITMQ_HOST}:${RABBITMQ_PORT}`],
				queue: 'data_queue',
				queueOptions: { durable: false },
			},
		},
	};
}
