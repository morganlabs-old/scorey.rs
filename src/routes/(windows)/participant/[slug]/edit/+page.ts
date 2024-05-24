import { type Load } from '@sveltejs/kit';

export const load: Load = ({ params }) => {
    return {
        participant_id: params.slug
    };
};
