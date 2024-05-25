import { invoke as i } from '@tauri-apps/api/tauri';

export const delete_team = async (id: number) => await i('delete_team', { id });
export const delete_participant = async (id: number) => await i('delete_participant', { id });
export const delete_event = async (id: number) => await i('delete_event', { id });
export const unenroll_team_in_events = async (team_id: number, events: number[] | number) => {
	events = (Array.isArray(events) ? events : [events]) as number[];

	const promises = [];

	for (const event_id of events) {
		promises.push(i('unenroll_team_in_events', { team_id, event_id }));
	}

	return Promise.all(promises);
};
