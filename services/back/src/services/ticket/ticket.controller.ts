import { Controller, Logger } from '@nestjs/common';
import { MessagePattern, RmqContext, Ctx, Payload } from '@nestjs/microservices';

import { TicketService } from './ticket.service';
import { Ticket } from '../..//types';

@Controller()
export class TicketController {
	private readonly logger = new Logger(TicketController.name);
	constructor(private readonly ticketSerivce: TicketService) {}

	@MessagePattern('get_tickets_response')
	getTickets(@Payload() data: Ticket[], @Ctx() context: RmqContext) {
		const channel = context.getChannelRef();
		const originalMsg = context.getMessage();

		this.logger.debug({ data, originalMsg });

		channel.ack(originalMsg);
	}
}
