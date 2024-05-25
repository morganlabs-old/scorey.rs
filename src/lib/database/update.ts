import { invoke as i } from '@tauri-apps/api/tauri';
import type { Team, Event, Participant } from './';

export const update_team = async (team: Team) =>
	await i<Team>('update_team', {
		id: team.id,
		name: team.name,
		individual: team.individual,
		points: team.points
	});

export const update_participant = async (participant: Participant) =>
	await i<Team>('update_participant', {
		id: participant.id,
		team_id: participant.team_id,
		first_name: participant.first_name,
		last_name: participant.last_name
	});

export const update_event = async (event: Event) =>
	await i<Event>('update_event', { id: event.id, name: event.name, event_type: event.event_type });
