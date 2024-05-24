import { type Load } from '@sveltejs/kit';

export const load: Load = ({ params }) => {
	return {
		team_id: params.slug
	};
};
