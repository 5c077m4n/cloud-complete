import { Inject, Injectable, Logger, OnApplicationBootstrap } from '@nestjs/common';
import { ClientProxy } from '@nestjs/microservices';
import { Observable, catchError, throwError } from 'rxjs';

import { Order } from '../../types';

@Injectable()
export class OrderService implements OnApplicationBootstrap {
	private readonly logger = new Logger(OrderService.name);
	constructor(@Inject('ORDER_SERVICE') private readonly dataService: ClientProxy) {}

	async onApplicationBootstrap() {
		await this.dataService.connect();
	}

	get(idList: string[] = []): Observable<Order[]> {
		this.logger.debug('Fetching orders # ' + idList.join(','));
		return this.dataService.send('get_orders', idList).pipe(
			catchError((error) => {
				this.logger.error(error);
				return throwError(() => error);
			}),
		);
	}
}
