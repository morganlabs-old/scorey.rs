import { invoke as i } from '@tauri-apps/api/tauri';
import { handle_error, type Team, type Event, type Participant } from './';

export async function update_team({ id, name, points }: Team) {
	try {
		const db_team = await i<Team>('update_team', { id, name, points });

		alert(`Team ${db_team.name} updated successfully.`);
		return db_team;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function update_participant({ id, team_id, first_name, last_name }: Participant) {
	try {
		const db_participant = await i<Participant>('update_participant', {
			id,
			team_id,
			first_name,
			last_name
		});
		alert(
			`Participant ${db_participant.first_name} ${db_participant.last_name} updated successfully.`
		);
		return db_participant;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function update_event({ id, name, event_type }: Event) {
	try {
		const db_event = await i<Event>('update_event', { id, name, event_type });

		alert(`Event ${db_event.name} updated successfully.`);
		return db_event;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}
