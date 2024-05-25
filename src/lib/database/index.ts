export * from './delete';
export * from './get';
export * from './get_many';
export * from './new';
export * from './update';

export function handle_error(e: Record<string, string>) {
	console.error('Error occured', e);

	const { type, message } = e;

	switch (type) {
		case 'UniqueConstraintFailed': {
			const field = message.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" must be unique.`);
			break;
		}
		case 'FieldIsRequired': {
			const field = message.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" is required.`);
			break;
		}
		case 'MustOnlyContainLettersAndSpaces': {
			const field = message.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" must only contain letters and spaces.`);
			break;
		}
		case 'MustOnlyContainLetters': {
			const field = message.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" must only contain letters.`);
			break;
		}
		case 'CheckViolation': {
			alert(`A check has been violated:\n${message}`);
			break;
		}
		case 'InvalidTeam': {
			alert(`Please select a team.`);
			break;
		}
		case 'IncorrectEventType': {
			alert(`Please choose a type of event.`);
			break;
		}
		default: {
			alert(`Failed to add entry.\n${message || e}`);
			break;
		}
	}
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
