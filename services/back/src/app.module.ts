import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';

import { AppController } from './app.controller';
import { rmqConfig } from './config';
import { OrderModule } from './services/order/order.module';
import { TicketModule } from './services/ticket/ticket.module';

@Module({
	imports: [ConfigModule.forRoot({ isGlobal: true, load: [rmqConfig] }), TicketModule, OrderModule],
	controllers: [AppController],
	providers: [],
})
export class AppModule {}
