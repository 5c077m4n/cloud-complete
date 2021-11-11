import { Controller, Logger } from '@nestjs/common';

import { OrderService } from './services/order/order.service';
import { TicketService } from './services/ticket/ticket.service';

@Controller()
export class AppController {
	private readonly logger = new Logger(AppController.name);

	constructor(private readonly orderService: OrderService, private readonly ticketService: TicketService) {
		setInterval(() => {
			this.ticketService.get(['324']).subscribe(this.logger.log.bind(this));
			this.ticketService.get(['324', '345', '876']).subscribe(this.logger.log.bind(this));
		}, 10_000);
	}
}
