<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import {
		new_team as new_team_inner,
		new_participant as new_participant_inner,
		type NewTeam,
		type NewParticipant
	} from '$lib';

	$: team = {
		name: '',
		individual: false
	} as NewTeam;

	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	async function new_team() {
		try {
			const new_team = await new_team_inner(team);

			if (team.individual) {
				await new_participant(new_team!.id);
			}
		} catch (e) {
			console.error(e);
			alert('Failed to add new team');
		}
	}

	async function new_participant(team_id: number) {
		try {
			await new_participant_inner(participant, team_id);
		} catch (e) {
			console.error(e);
			alert('Failed to add new participant.');
		}
	}
</script>

<form on:submit={new_team}>
	<label>
		Team Name
		<input type="text" bind:value={team.name} />
	</label>
	<label>
		Individual
		<input type="checkbox" bind:checked={team.individual} />
	</label>
	{#if team.individual}
		<label>
			Participant First Name
			<input type="text" bind:value={participant.first_name} />
		</label>
		<label>
			Participant Last Name
			<input type="text" bind:value={participant.last_name} />
		</label>
	{/if}
	<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
	<button class="primary" type="submit">Save</button>
</form>
