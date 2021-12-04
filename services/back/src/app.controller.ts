import { Controller, Logger, OnModuleInit } from '@nestjs/common';

import { OrderService } from './services/order/order.service';
import { TicketService } from './services/ticket/ticket.service';

@Controller()
export class AppController implements OnModuleInit {
	private readonly logger = new Logger(AppController.name);
	private counter = 0;

	constructor(private readonly orderService: OrderService, private readonly ticketService: TicketService) {}

	onModuleInit() {
		const log = this.logger.log.bind(this);

		setInterval(() => {
			this.orderService.get([this.counter.toString()]).subscribe(log);
			this.ticketService.get([this.counter.toString()]).subscribe(log);

			this.counter++;
		}, 5_000);
	}
}
