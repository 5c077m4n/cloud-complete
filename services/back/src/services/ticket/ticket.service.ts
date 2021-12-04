import { Inject, Injectable, Logger, OnApplicationBootstrap } from '@nestjs/common';
import { ClientProxy } from '@nestjs/microservices';
import { catchError, Observable, throwError } from 'rxjs';

import { Ticket } from '../../types';

@Injectable()
export class TicketService implements OnApplicationBootstrap {
	private readonly logger = new Logger(TicketService.name);
	constructor(@Inject('TICKET_SERVICE') private readonly dataService: ClientProxy) {}

	async onApplicationBootstrap() {
		await this.dataService.connect();
	}

	get(idList: string[] = []): Observable<Ticket[]> {
		this.logger.debug('Fetching tickets # ' + idList.join(','));
		return this.dataService.send('get_tickets', idList).pipe(
			catchError((error) => {
				this.logger.error(error);
				return throwError(() => error);
			}),
		);
	}
}
