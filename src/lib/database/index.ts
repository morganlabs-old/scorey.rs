export * from './delete';
export * from './get';
export * from './get_many';
export * from './new';
export * from './update';

export type Team = {
	id: number;
	name: string;
	individual: boolean;
	points: number;
};

export type Participant = {
	id: number;
	team_id: number;
	first_name: string;
	last_name: string;
	team_name?: string;
	team_individual?: boolean;
	team_points?: number;
};

export type Event = {
	id: number;
	name: string;
	event_type: string;
};

export type NewTeam = {
	name: string;
	individual: boolean;
};

export type NewParticipant = {
	first_name: string;
	last_name: string;
};

export type NewEvent = {
	name: string;
	event_type: string;
};
