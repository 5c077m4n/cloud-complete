import { Inject, Injectable, Logger } from '@nestjs/common';
import { ClientProxy } from '@nestjs/microservices';
import { Observable } from 'rxjs';

import { Ticket } from '../../types';

@Injectable()
export class TicketService {
	private readonly logger = new Logger(TicketService.name);
	constructor(@Inject('DATA_SERVICE') private readonly dataService: ClientProxy) {}

	getOne(id: string): Observable<Ticket> {
		this.logger.debug('Fetching one ticket #' + id);
		return this.dataService.send('get_ticket', id);
	}
	getBatch(idList: string[] = []): Observable<Ticket[]> {
		this.logger.debug('Fetching tickets #' + idList.join(','));
		return this.dataService.send('get_tickets', idList);
	}
}
