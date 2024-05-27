import { invoke as i } from '@tauri-apps/api/tauri';
import { handle_error, type Team, type Event, type Participant } from './';

export async function get_team(id: number) {
	try {
		return await i<Team>('get_team', { id });
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function get_participant(id: number) {
	try {
		return await i<Participant>('get_participant', { id });
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function get_event(id: number) {
	try {
		return await i<Event>('get_event', { id });
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}
