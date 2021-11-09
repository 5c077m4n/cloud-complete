import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { ClientProxyFactory } from '@nestjs/microservices';

import { AppController } from './app.controller';
import { AppService } from './app.service';
import { rabbitConfig } from './config';
import { TicketService } from './services/ticket/ticket.service';
import { OrderService } from './services/order/order.service';

@Module({
	imports: [ConfigModule.forRoot({ isGlobal: true, load: [rabbitConfig] })],
	controllers: [AppController],
	providers: [
		{
			provide: 'DATA_SERVICE',
			inject: [ConfigService],
			useFactory: (configService: ConfigService) => {
				const rabbitData = configService.get('rabbitmqData');
				return ClientProxyFactory.create(rabbitData);
			},
		},
		AppService,
		TicketService,
		OrderService,
	],
})
export class AppModule {}
