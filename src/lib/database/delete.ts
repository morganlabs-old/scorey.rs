import { invoke as i } from '@tauri-apps/api/tauri';
import { handle_error } from '.';

export async function delete_team(id: number) {
	try {
		await i('delete_team', { id });
		alert('Team deleted successfully.');
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function delete_participant(id: number) {
	try {
		await i('delete_participant', { id });
		alert('Participant deleted successfully.');
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function delete_event(id: number) {
	try {
		await i('delete_event', { id });
		alert('Event deleted successfully.');
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function unenroll_team_in_events(team_id: number, events: number[] | number) {
	events = (Array.isArray(events) ? events : [events]) as number[];

	for (const event_id of events) {
		try {
			i('unenroll_team_in_events', { team_id, event_id });
		} catch (e) {
			handle_error(e as Record<string, string>);
		}
	}
}
