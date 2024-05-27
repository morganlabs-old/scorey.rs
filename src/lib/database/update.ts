import { invoke as i } from '@tauri-apps/api/tauri';
import { handle_error, type Team, type Event, type Participant } from './';

export async function update_team(team: Team) {
	try {
		const db_team = await i<Team>('update_team', {
			id: team.id,
			name: team.name,
			individual: team.individual,
			points: team.points
		});

		alert(`Team ${db_team.name} updated successfully.`);
		return db_team;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function update_participant(participant: Participant) {
	try {
		const db_participant = await i<Participant>('update_participant', {
			id: participant.id,
			team_id: participant.team_id,
			first_name: participant.first_name,
			last_name: participant.last_name
		});

		alert(
			`Participant ${db_participant.first_name} ${db_participant.last_name} updated successfully.`
		);
		return db_participant;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}

export async function update_event(event: Event) {
	try {
		const db_event = await i<Event>('update_event', {
			id: event.id,
			name: event.name,
			event_type: event.event_type
		});

		alert(`Event ${db_event.name} updated successfully.`);
		return db_event;
	} catch (e) {
		handle_error(e as Record<string, string>);
	}
}
