export enum TicketStatus {
	FREE = 'free',
	ORDERED = 'ordered',
	CACNELED = 'canceled',
}
export interface Ticket {
	_id: string;
	title: string;
	price: string;
	date: Date;
	status: TicketStatus;
}
export interface Order {
	_id: string;
	title: string;
	ticketRef: string[];
	date: Date;
}
