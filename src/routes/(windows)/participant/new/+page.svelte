<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_participant as new_participant_inner, get_teams, type NewParticipant } from '$lib';

	let teams = get_teams();
	let team_id = 0;

	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	async function new_participant() {
		try {
			await new_participant_inner(participant, team_id);
			alert('Added new participant.');
		} catch (e) {
			console.error(e);
			alert('Failed to add new participant.');
		}
	}
</script>

<form on:submit={new_participant}>
	<label>
		First Name
		<input type="text" bind:value={participant.first_name} />
	</label>
	<label>
		Last Name
		<input type="text" bind:value={participant.last_name} />
	</label>
	<label>
		Team Name
		<select bind:value={team_id}>
			{#await teams then teams}
				{#each teams as team}
					{#if !team.individual}
						<option value={team.id}>{team.name}</option>
					{/if}
				{/each}
			{/await}
		</select>
	</label>
	<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
	<button class="primary" type="submit">Save</button>
</form>
