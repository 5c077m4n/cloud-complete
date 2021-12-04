import { Controller, Logger } from '@nestjs/common';
import { MessagePattern, RmqContext, Ctx, Payload } from '@nestjs/microservices';

import { Order } from '../../types';

@Controller()
export class OrderController {
	private readonly logger = new Logger(OrderController.name);

	@MessagePattern('get_orders_response')
	getTickets(@Payload() data: Order[], @Ctx() context: RmqContext) {
		const channel = context.getChannelRef();
		const originalMsg = context.getMessage();

		this.logger.debug({ data, props: originalMsg.properties });

		channel.ack(originalMsg);
	}
}
