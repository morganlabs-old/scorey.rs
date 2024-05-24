<script lang="ts">
	import {
		update_participant as update_participant_inner,
		get_participant as get_participant_inner,
		type Participant,
		get_teams
	} from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	export let data: { participant_id: string };
	const { participant_id } = data;

	$: new_participant = {
		id: 0,
		team_id: 0,
		first_name: '',
		last_name: '',
		team_individual: false
	} as Participant;

	async function get_participant_return_teams() {
		const participant = await get_participant_inner(+participant_id);
		const teams = await get_teams();
		const participants_team = teams.find((x) => x.id === participant.team_id)!;
		new_participant.id = participant.id;
		new_participant.team_id = participant.team_id;
		new_participant.first_name = participant.first_name;
		new_participant.last_name = participant.last_name;
		new_participant.team_individual = participants_team.individual;
		return teams;
	}

	async function update_participant() {
		try {
			await update_participant_inner(new_participant);
			alert('Updated participant sucessfully.');
		} catch (e) {
			console.error(e);
			alert('Failed to update participant.');
		}
	}
</script>

{#await get_participant_return_teams() then teams}
	<form on:submit={update_participant}>
		<label>
			<input type="text" bind:value={new_participant.first_name} />
			<span>First Name</span>
		</label>
		<label>
			<input type="text" bind:value={new_participant.last_name} />
			<span>Last Name</span>
		</label>
		<label>
			{#if !new_participant.team_individual}
				<select bind:value={new_participant.team_id}>
					{#each teams as team}
						{#if !new_participant.team_individual}
							{#if !team.individual}
								<option value={team.id} selected={new_participant.team_id === team.id}>
									{team.name}
								</option>
							{/if}
						{/if}
					{/each}
				</select>
			{:else}
				<select bind:value={new_participant.team_id} disabled>
					{#each teams as team}
						{#if team.id === new_participant.team_id}
							<option value={team.id} selected={true}>{team.name}</option>
						{/if}
					{/each}
				</select>
			{/if}
			<span>Team Name</span>
		</label>
		<div class="actions">
			<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
			<button class="primary" type="submit">Save</button>
		</div>
	</form>
{:catch error}
	<p>{error.message}</p>
{/await}
