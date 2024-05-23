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
};
