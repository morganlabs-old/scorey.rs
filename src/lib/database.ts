import { invoke } from '@tauri-apps/api/tauri';

export async function get_teams() {
	const teams = await invoke<Team[]>('get_teams');
	return teams;
}

export async function get_team(team_id: number) {
	const teams = await invoke<Team>('get_team', { team_id });
	return teams;
}

export async function get_events() {
	const events = await invoke<Event[]>('get_events');
	return events;
}

export async function get_participants() {
	const participants = await invoke<Participant[]>('get_participants');
	console.log(participants);
	return participants;
}

export async function new_team(team: NewTeam) {
	const { name, individual } = team;

	try {
		const db_team = await invoke<Team>('new_team', { name, individual });
		return db_team;
	} catch (e) {
		const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
		console.error(e);
		return alert(`Failed to add team.\n${msg}`);
	}
}

export async function new_participant(
	participant: NewParticipant,
	team_id: number
): Promise<Participant> {
	const { first_name, last_name } = participant;

	const db_participant = (await invoke('new_participant', {
		first_name,
		last_name,
		team_id
	}).catch((e) => alert(e))) as Participant;

	const team = await get_team(team_id);

	return {
		...db_participant,
		team_name: team.name,
		team_individual: team.individual,
		team_points: team.points
	};
}

export async function new_event(event: NewEvent) {
	const { name, event_type } = event;

	const db_event = await invoke<Event>('new_event', { name, event_type }).catch((e) => alert(e));

	return db_event;
}

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
	type: string;
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
