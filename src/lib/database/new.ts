import { invoke as i } from '@tauri-apps/api/tauri';
import { handle_error } from './';
import type { Team, Event, Participant, NewTeam, NewEvent, NewParticipant } from './';

export async function new_team(team: NewTeam) {
	const { name, individual } = team;

	try {
		const team = await i<Team>('new_team', { name, individual });
		alert(`Team "${team.name}" added successfully!`);
		return team;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function new_participant(participant: NewParticipant, team_id: number) {
	const { first_name, last_name } = participant;

	try {
		const participant = await i<Participant>('new_participant', { first_name, last_name, team_id });
		alert(`Participant "${participant.first_name} ${participant.last_name}" added scucessfully!`);
		return participant;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function new_event(event: NewEvent) {
	const { name, event_type } = event;

	try {
		const db_event = await i<Event>('new_event', { name, event_type });
		alert(`Event "${db_event.name}" added successfully!`);
		return db_event;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function enroll_team_in_events(team_id: number, events: number[] | number) {
	events = (Array.isArray(events) ? events : [events]) as number[];

	const promises = [];

	for (const event_id of events) {
		promises.push(i('enroll_team_in_events', { team_id, event_id }));
	}

	return Promise.all(promises);
}
