<script lang="ts">
	import {
		update_participant,
		get_participant as get_participant_inner,
		get_team,
		type Participant,
		get_teams as get_teams_inner
	} from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	export let data: { id: number };
	const { id } = data;

	$: team_selector_enabled = true;
	$: new_participant = {
		id: 0,
		team_id: 0,
		first_name: '',
		last_name: '',
		team_individual: false,
		team_name: ''
	} as Participant;

	async function get_participant() {
		const participant = await get_participant_inner(id);
		if (!participant) app_window.close();

		new_participant.id = participant!.id;
		new_participant.team_id = participant!.team_id;
		new_participant.first_name = participant!.first_name;
		new_participant.last_name = participant!.last_name;

		const team = await get_team(participant!.team_id);
		if (!team) app_window.close();

		new_participant.team_individual = team!.individual;
		new_participant.team_name = team!.name;
	}

	async function get_teams() {
		const teams = await get_teams_inner();

		if (!teams) app_window.close();

		if (teams!.length === 1) {
			team_selector_enabled = false;
		}
		return teams!;
	}
</script>

{#await get_participant() then}
	<form on:submit={() => update_participant(new_participant)}>
		<label>
			<input type="text" placeholder="John" bind:value={new_participant.first_name} />
			<span>First Name</span>
		</label>
		<label>
			<input type="text" placeholder="Doe" bind:value={new_participant.last_name} />
			<span>Last Name</span>
		</label>
		<label>
			<select bind:value={new_participant.team_id} disabled={!team_selector_enabled}>
				{#await get_teams()}
					<option>Loading...</option>
				{:then teams}
					{#if teams.length === 1}
						<option value={new_participant.team_id} selected={true}>
							{new_participant.team_name}
						</option>
					{:else}
						{#each teams as team}
							<option value={team.id} selected={new_participant.team_id === team.id}>
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
{/await}
