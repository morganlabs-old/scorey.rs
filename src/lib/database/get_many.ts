import { invoke as i } from '@tauri-apps/api/tauri';
import { handle_error, type Team, type Event, type Participant } from './';

export async function get_teams() {
	try {
		return await i<Team[]>('get_teams');
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function get_participants() {
	try {
		return await i<Participant[]>('get_participants');
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function get_events() {
	try {
		return await i<Event[]>('get_events');
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function get_team_events(id: number) {
	try {
		return await i<number[]>('get_team_events', { id });
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}
