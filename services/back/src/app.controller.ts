import { Controller, Logger } from '@nestjs/common';

import { OrderService } from './services/order/order.service';
import { TicketService } from './services/ticket/ticket.service';

@Controller()
export class AppController {
	private readonly logger = new Logger(AppController.name);

	constructor(private readonly orderService: OrderService, private readonly ticketService: TicketService) {
		this.orderService.getBatch().subscribe();
		this.ticketService.getBatch().subscribe();
	}
}
