import { Module } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { ClientProxyFactory, ClientsModule } from '@nestjs/microservices';

import { TicketService } from './ticket.service';
import { TicketController } from './ticket.controller';

@Module({
	imports: [
		ClientsModule.registerAsync([
			{
				name: 'TICKET_RESPONSE',
				inject: [ConfigService],
				useFactory: (configService: ConfigService) => {
					const ticketResponseQueue = configService.get('ticketResponseQueue');
					return { ...ticketResponseQueue, name: 'TICKET_RESPONSE' };
				},
			},
		]),
	],
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
