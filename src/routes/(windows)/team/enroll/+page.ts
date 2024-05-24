import { get_team_events } from '$lib';
import { error, type Load } from '@sveltejs/kit';

export const load: Load = async ({ url }) => {
	const id = url.searchParams.get('id');
	if (!id) return error(400, 'Missing ID query parameter.');
	const team_events = await get_team_events(+id);

	return {
		team_id: +id,
		team_events
	};
};
