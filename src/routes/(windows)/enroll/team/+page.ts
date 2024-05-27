import { error, type Load } from '@sveltejs/kit';

export const load: Load = async ({ url }) => {
	const id = url.searchParams.get('id');
	if (!id) return error(400, 'Missing ID query parameter.');

	return { id: +id };
};
