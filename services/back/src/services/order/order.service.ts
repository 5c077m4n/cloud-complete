import { Inject, Injectable, Logger } from '@nestjs/common';
import { ClientProxy } from '@nestjs/microservices';
import { Observable } from 'rxjs';

import { Order } from '../../types';

@Injectable()
export class OrderService {
	private readonly logger = new Logger(OrderService.name);
	constructor(@Inject('DATA_SERVICE') private readonly dataService: ClientProxy) {}

	getOne(id: string): Observable<Order> {
		this.logger.debug('Fetching one order #' + id);
		return this.dataService.send('get_order', id);
	}
	getBatch(idList: string[] = []): Observable<Order[]> {
		this.logger.debug('Fetching orders #' + idList.join(','));
		return this.dataService.send('get_orders', idList);
	}
}
