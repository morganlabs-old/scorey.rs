import { type Load } from '@sveltejs/kit';

export const load: Load = ({ params }) => {
    return {
        event_id: params.slug
    };
};
