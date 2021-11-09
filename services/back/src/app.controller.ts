import { Controller, Logger } from '@nestjs/common';

import { AppService } from './app.service';

@Controller()
export class AppController {
	private readonly logger = new Logger(AppController.name);

	constructor(private readonly appService: AppService) {
		this.appService.getTicket('123').subscribe();
		this.appService.getTickets().subscribe();
	}
}
