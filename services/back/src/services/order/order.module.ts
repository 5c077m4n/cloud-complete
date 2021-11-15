import { Module } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { ClientProxyFactory, ClientsModule } from '@nestjs/microservices';

import { OrderController } from './order.controller';
import { OrderService } from './order.service';

@Module({
	imports: [
		ClientsModule.registerAsync([
			{
				name: 'ORDER_RESPONSE',
				inject: [ConfigService],
				useFactory: (configService: ConfigService) => {
					const orderResponseQueue = configService.get('orderResponseQueue');
					return { ...orderResponseQueue, name: 'ORDER_RESPONSE' };
				},
			},
		]),
	],
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
