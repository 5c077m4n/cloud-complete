import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';

import { rmqConfig } from './config';
import { AppController } from './app.controller';
import { OrderModule } from './services/order/order.module';
import { TicketModule } from './services/ticket/ticket.module';

@Module({
	imports: [ConfigModule.forRoot({ isGlobal: true, load: [rmqConfig] }), TicketModule, OrderModule],
	controllers: [AppController],
	providers: [],
})
export class AppModule {}
