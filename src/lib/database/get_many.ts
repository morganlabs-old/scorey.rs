import { invoke as i } from '@tauri-apps/api/tauri';
import type { Team, Event, Participant } from './';

export const get_teams = async () => await i<Team[]>('get_teams');
export const get_participants = async () => await i<Participant[]>('get_participants');
export const get_events = async () => await i<Event[]>('get_events');
export const get_team_events = async (team_id: number) =>
	await i<number[]>('get_team_events', { team_id });
