import { Module } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { ClientProxyFactory } from '@nestjs/microservices';

import { TicketService } from './ticket.service';
import { TicketController } from './ticket.controller';

@Module({
	controllers: [TicketController],
	providers: [
		{
			provide: 'TICKET_SERVICE',
			inject: [ConfigService],
			useFactory: (configService: ConfigService) => {
				const ticketQueue = configService.get('ticketRequestQueue');
				return ClientProxyFactory.create({ ...ticketQueue, name: 'TICKET_SERVICE' });
			},
		},
		TicketService,
	],
	exports: [TicketService],
})
export class TicketModule {}
