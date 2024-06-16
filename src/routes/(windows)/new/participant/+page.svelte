<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_participant, get_teams as get_teams_inner, type NewParticipant } from '$lib';

	$: team_id = 0;
	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	$: team_selector_enabled = true;

	async function add_participant() {
		const db_participant = await new_participant(participant, team_id);
		if (!db_participant) return;
		participant = { first_name: '', last_name: '' };
	}

	async function get_teams() {
		const teams = await get_teams_inner();
		if (!teams) app_window.close();
		else if (teams.length === 1) {
			team_selector_enabled = false;
		}
		return teams!.filter((t) => !t.individual);
	}
</script>

<form on:submit={add_participant}>
	<label>
		<input type="text" placeholder="John" bind:value={participant.first_name} />
		<span>First Name</span>
	</label>
	<label>
		<input type="text" placeholder="Doe" bind:value={participant.last_name} />
		<span>Last Name</span>
	</label>
	<label>
		<select bind:value={team_id} disabled={!team_selector_enabled}>
			{#await get_teams()}
				<option>Loading...</option>
			{:then teams}
				{#if teams.length === 1}
					<option value={teams[0].id} selected={true}>
						{teams[0].name}
					</option>
				{:else}
					{#each teams as team}
						<option value={team.id} selected={team_id === team.id}>
							{team.name}
						</option>
					{/each}
				{/if}
			{/await}
		</select>
		<span>Team Name</span>
	</label>
	<div class="actions">
		<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
		<button class="primary" type="submit">Save</button>
	</div>
</form>
