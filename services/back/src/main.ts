import { Logger } from '@nestjs/common';
import { NestFactory } from '@nestjs/core';

import { AppModule } from './app.module';

async function bootstrap() {
	const logger = new Logger('NestApplication');
	const app = await NestFactory.create(AppModule, { logger });

	const port = 3000;
	await app.listen(port, () => logger.log('Now listening on port ' + port));
}

bootstrap().catch((e) => {
	console.error(e);
	process.exit(1);
});
