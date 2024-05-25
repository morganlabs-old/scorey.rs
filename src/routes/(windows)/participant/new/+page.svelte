<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_participant, get_teams, type NewParticipant, type Team } from '$lib';

	$: team_id = 0;
	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	async function add_participant() {
		const db_participant = await new_participant(participant, team_id);
		if (!db_participant) return;
		participant = { first_name: '', last_name: '' };
	}

	function is_selected(teams: Team[], idx: number) {
		const team = teams[idx];
		const is_selected = teams.length === 1 && idx === 0;
		if (is_selected) team_id = team.id;
		return is_selected;
	}
</script>

<form on:submit={add_participant}>
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
			{#await get_teams() then teams}
				{#if teams.filter((t) => !t.individual).length > 1}
					<option value={0} selected>None</option>
				{/if}
				{#each teams as team, idx}
					{#if !team.individual}
						<option value={team.id} selected={(() => is_selected(teams, idx))()}>{team.name}</option
						>
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
