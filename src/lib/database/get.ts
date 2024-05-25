import { invoke as i } from '@tauri-apps/api/tauri';
import type { Team, Event, Participant } from './';

export const get_team = async (team_id: number) => await i<Team>('get_team', { team_id });
export const get_participant = async (participant_id: number) =>
	await i<Participant>('get_participant', { participant_id });
export const get_event = async (event_id: number) => await i<Event>('get_event', { event_id });
