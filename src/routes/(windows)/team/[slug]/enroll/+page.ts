import { type Load } from '@sveltejs/kit';
import { get_team_events } from '$lib';

export const load: Load = async ({ params }) => {
    const team_events = await get_team_events(+params.slug!);

    return {
        team_id: params.slug,
        team_events
    };
};
