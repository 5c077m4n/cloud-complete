import { Module } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { ClientProxyFactory } from '@nestjs/microservices';

import { OrderController } from './order.controller';
import { OrderService } from './order.service';

@Module({
	imports: [],
	controllers: [OrderController],
	providers: [
		{
			provide: 'ORDER_SERVICE',
			inject: [ConfigService],
			useFactory: (configService: ConfigService) => {
				const orderQueue = configService.get('orderRequestQueue');
				return ClientProxyFactory.create({ ...orderQueue, name: 'ORDER_SERVICE' });
			},
		},
		OrderService,
	],
	exports: [OrderService],
})
export class OrderModule {}
