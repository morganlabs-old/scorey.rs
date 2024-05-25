import { invoke as i } from '@tauri-apps/api/tauri';
import { get_team } from './get';
import type { Team, Event, Participant, NewTeam, NewEvent, NewParticipant } from './';

export const new_team = async (team: NewTeam) => {
	const { name, individual } = team;

	try {
		const db_team = await i<Team>('new_team', { name, individual });
		return db_team;
	} catch (e) {
		const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
		console.error(e);
		return alert(`Failed to add team.\n${msg}`);
	}
};

export const new_participant = async (
	participant: NewParticipant,
	team_id: number
): Promise<Participant> => {
	const { first_name, last_name } = participant;

	const db_participant = (await i('new_participant', {
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
};

export const new_event = async (event: NewEvent) => {
	const { name, event_type } = event;

	const db_event = await i<Event>('new_event', { name, event_type }).catch((e) => alert(e));

	return db_event;
};

export const enroll_team_in_events = (team_id: number, events: number[] | number) => {
	events = (Array.isArray(events) ? events : [events]) as number[];

	const promises = [];

	for (const event_id of events) {
		promises.push(i('enroll_team_in_events', { team_id, event_id }));
	}

	return Promise.all(promises);
};
