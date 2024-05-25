import { invoke as i } from '@tauri-apps/api/tauri';

export const delete_team = async (team_id: number) => await i('delete_team', { team_id });
export const delete_participant = async (participant_id: number) =>
	await i('delete_participant', { participant_id });
export const delete_event = async (event_id: number) => await i('delete_event', { event_id });
export const unenroll_team_in_events = async (team_id: number, events: number[] | number) => {
	events = (Array.isArray(events) ? events : [events]) as number[];

	const promises = [];

	for (const event_id of events) {
		promises.push(i('unenroll_team_in_events', { team_id, event_id }));
	}

	return Promise.all(promises);
};
