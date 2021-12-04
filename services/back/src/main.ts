import { Logger } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { NestFactory } from '@nestjs/core';
import { MicroserviceOptions } from '@nestjs/microservices';

import { AppModule } from './app.module';

async function bootstrap() {
	const logger = new Logger('NestApplication');
	const app = await NestFactory.create(AppModule, { logger });

	const configService = app.get(ConfigService);
	app.connectMicroservice<MicroserviceOptions>(configService.get('ticketResponseQueue'));
	app.connectMicroservice<MicroserviceOptions>(configService.get('orderResponseQueue'));

	const port = 3333;
	await app.startAllMicroservices();
	await app.listen(port, () => logger.log('Now listening on port ' + port));
}

bootstrap().catch((e) => {
	console.error(e);
	process.exit(1);
});
