<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_participant as new_participant_inner, get_teams, type NewParticipant } from '$lib';

	const teams = (async () => {
		const teams = await get_teams();
		return teams;
	})();

	$: team_id = 0;
	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	async function new_participant() {
		try {
			if (team_id === 0) {
				alert('Please select a team.');
				return;
			}
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
		<input type="text" bind:value={participant.first_name} />
		<span>First Name</span>
	</label>
	<label>
		<input type="text" bind:value={participant.last_name} />
		<span>Last Name</span>
	</label>
	<label>
		<select bind:value={team_id}>
			<option value={0} selected>None</option>
			{#await teams then teams}
				{#each teams as team}
					{#if !team.individual}
						<option value={team.id}>{team.name}</option>
					{/if}
				{/each}
			{/await}
		</select>
		<span>Team Name</span>
	</label>
	<div class="actions">
		<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
		<button class="primary" type="submit">Save</button>
	</div>
</form>
