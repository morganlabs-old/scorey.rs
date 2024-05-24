import { invoke } from '@tauri-apps/api/tauri';

export function enroll_team_in_events(team_id: number, events: number[] | number) {
    events = (Array.isArray(events) ? events : [events]) as number[];

    const promises = [];

    for (const event_id of events) {
        promises.push(invoke('enroll_team_in_events', { team_id, event_id }));
    }

    return Promise.all(promises);
}

export async function delete_participant(participant_id: number) {
    return await invoke('delete_participant', { participant_id });
}

export async function unenroll_team_in_events(team_id: number, events: number[] | number) {
    events = (Array.isArray(events) ? events : [events]) as number[];

    const promises = [];

    for (const event_id of events) {
        promises.push(invoke('unenroll_team_in_events', { team_id, event_id }));
    }

    return Promise.all(promises);
}

export async function update_team(team: Team) {
    const { id, name, individual, points } = team;
    return await invoke<Team>('update_team', { id, name, individual, points });
}

export async function update_event(event: Event) {
    const { id, name, event_type } = event;
    return await invoke<Event>('update_event', { id, name, event_type });
}

export async function update_participant(participant: Participant) {
    const { id, team_id, first_name, last_name } = participant;
    return await invoke<Team>('update_participant', { id, team_id, first_name, last_name });
}

export async function get_team_events(team_id: number) {
    return await invoke<number[]>('get_team_events', { team_id });
}

export async function get_teams() {
    return await invoke<Team[]>('get_teams');
}

export async function get_team(team_id: number) {
    return await invoke<Team>('get_team', { team_id });
}

export async function get_event(event_id: number) {
    return await invoke<Event>('get_event', { event_id });
}

export async function get_participant(participant_id: number) {
    return await invoke<Participant>('get_participant', { participant_id });
}

export async function get_events() {
    return await invoke<Event[]>('get_events');
}

export async function get_participants() {
    return await invoke<Participant[]>('get_participants');
}

export async function new_team(team: NewTeam) {
    const { name, individual } = team;

    try {
        const db_team = await invoke<Team>('new_team', { name, individual });
        return db_team;
    } catch (e) {
        const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
        console.error(e);
        return alert(`Failed to add team.\n${msg}`);
    }
}

export async function new_participant(
    participant: NewParticipant,
    team_id: number
): Promise<Participant> {
    const { first_name, last_name } = participant;

    const db_participant = (await invoke('new_participant', {
        first_name,
        last_name,
        team_id
    }).catch((e) => alert(e))) as Participant;

    const team = await get_team(team_id);

    return {
        ...db_participant,
        team_name: team.name,
        team_individual: team.individual,
        team_points: team.points
    };
}

export async function new_event(event: NewEvent) {
    const { name, event_type } = event;

    const db_event = await invoke<Event>('new_event', { name, event_type }).catch((e) => alert(e));

    return db_event;
}

export type Team = {
    id: number;
    name: string;
    individual: boolean;
    points: number;
};

export type Participant = {
    id: number;
    team_id: number;
    first_name: string;
    last_name: string;
    team_name?: string;
    team_individual?: boolean;
    team_points?: number;
};

export type Event = {
    id: number;
    name: string;
    event_type: string;
};

export type NewTeam = {
    name: string;
    individual: boolean;
};

export type NewParticipant = {
    first_name: string;
    last_name: string;
};

export type NewEvent = {
    name: string;
    event_type: string;
};
