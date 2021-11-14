import { Controller, Logger } from '@nestjs/common';
import { MessagePattern, RmqContext, Ctx, Payload } from '@nestjs/microservices';

import { Order } from '../../types';
import { OrderService } from './order.service';

@Controller()
export class OrderController {
	private readonly logger = new Logger(OrderController.name);
	constructor(private readonly orderSerivce: OrderService) {}

	@MessagePattern('get_order_response')
	getTickets(@Payload() data: Order[], @Ctx() context: RmqContext) {
		const channel = context.getChannelRef();
		const originalMsg = context.getMessage();

		this.logger.debug({ data, originalMsg });

		channel.ack(originalMsg);
	}
}
