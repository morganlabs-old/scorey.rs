import { exit } from '@tauri-apps/api/process';

export * from './delete';
export * from './get';
export * from './get_many';
export * from './new';
export * from './update';

export function handle_error(e: Record<string, string>) {
	console.error('Error occured', e);

	const { type, info } = e;

	switch (type) {
		case 'Generic': {
			alert('An error has occured.');
			break;
		}

		// Database errors
		case 'DatabaseDirectoryCreation': {
			alert(
				`Failed to create SQLite database directory at ${info}. Do you have write permissions to this directory?`
			);
			exit(1);
			break;
		}
		case 'DatabaseCreation': {
			alert(
				`Failed to create SQLite database file at ${info}. Do you have write permissions to this directory?`
			);
			exit(1);
			break;
		}
		case 'DatabaseConnectionFailure': {
			alert(`Failed to connect to SQLite database.`);
			exit(1);
			break;
		}
		case 'DatabaseMigrationFailure': {
			alert(`Failed to push migration(s) to SQLite database.`);
			exit(1);
			break;
		}
		case 'DatabaseQueryFailure': {
			alert(`Failed to query database:\n${info}.`);
			break;
		}
		case 'DatabaseNewEntryFailure': {
			alert(`Failed to create new entry in database:\n${info}.`);
			break;
		}
		case 'DatabaseDeleteEntryFailure': {
			alert(`Failed to delete database item:\n${info}.`);
			break;
		}
		case 'DatabaseUpdateEntryFailure': {
			alert(`Failed to update value(s) within the database:\n${info}.`);
			break;
		}

		// Data modification errors
		case 'ValidationIncorrectEventType': {
			alert(`Event type "${info}" is not valid. Valid types are:\nACADEMIC\nSPORT.`);
			break;
		}
		case 'ValidationUniqueConstraintFailed': {
			const field = info.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" must be unique.`);
			break;
		}
		case 'ValidationCheckViolation': {
			alert(`A check has been violated:\n${info}`);
			break;
		}
		case 'ValidationFieldIsRequired': {
			const field = info.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" is required.`);
			break;
		}
		case 'ValidationMustOnlyContainLettersAndSpaces': {
			const field = info.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" must only contain letters and spaces.`);
			break;
		}
		case 'ValidationMustOnlyContainLetters': {
			const field = info.split(':')[1].replaceAll('.', ' ');
			alert(`Field "${field}" must only contain letters.`);
			break;
		}
		case 'ValidationCannotDeleteNonIndividualTeamWithMembers': {
			alert('Cannot delete a team with members.');
			break;
		}
		case 'ValidationCannotDeleteEventsWithTeamsEnrolled': {
			alert('Cannot delete an event with teams enrolled.');
			break;
		}
		case 'ValidationInvalidTeam': {
			alert(`Please select a team.`);
			break;
		}
		default: {
			alert(`Failed to add entry.\n${info}`);
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
