<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import {
		type Team,
		// type NewTeam,
		// type NewParticipant,
		// new_team,
		// new_participant,
		get_teams
	} from '$lib';

	function edit_team(team: Team) {
		alert(`Editing team ${team.name}`);
	}

	// $: team = {
	// 	name: '',
	// 	individual: false
	// } as NewTeam;

	// $: participant = {
	// 	first_name: '',
	// 	last_name: ''
	// } as NewParticipant;

	$: new_teams = [] as Team[];

	// async function new_team_and_participant() {
	// 	try {
	// 		const db_team = await new_team(team);
	// 		new_teams = [...new_teams, db_team!];

	// 		if (team.individual) {
	// 			try {
	// 				await new_participant(participant, db_team!.id);
	// 			} catch (e) {
	// 				const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
	// 				console.error(e);
	// 				return alert(`Failed to add participant.\n${msg}`);
	// 			}
	// 		}

	// 		team = { name: '', individual: team.individual };
	// 		participant = { first_name: '', last_name: '' };
	// 	} catch (e) {
	// 		const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
	// 		console.error(e);
	// 		return alert(`Failed to add team.\n${msg}`);
	// 	}
	// }
</script>

<Banner title="Teams" subtitle="Click on a team to edit it." />
<!-- <form on:submit={new_team_and_participant}>
	<label>
		Team Name
		<input type="text" bind:value={team.name} />
	</label>
	<label>
		Individual?
		<input type="checkbox" bind:checked={team.individual} />
	</label>
	{#if team.individual}
		<h2>Add a new participant</h2>
		<label>
			First Name
			<input type="text" bind:value={participant.first_name} />
		</label>
		<label>
			Last Name
			<input type="text" bind:value={participant.last_name} />
		</label>
	{/if}
	<input type="submit" value="Add" /> -->
<!-- </form> -->

<Table headings={['ID', 'Name', 'Individual?', 'Points']} highlighted_columns={[1]}>
	{#await get_teams() then teams}
		{#each [teams, new_teams].flat() as team}
			<tr class="row">
				<td class="id">{team.id}</td>
				<th class="name">{team.name}</th>
				<td class="individual">
					<input type="checkbox" checked={team.individual} on:click|preventDefault />
				</td>
				<td class="points">{team.points}</td>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_team(team)}>Edit</button>
				</th>
			</tr>
		{/each}
	{:catch error}
		<p class="error">
			Failed to get teams...<br />
		</p>
		<pre>{error}</pre>
	{/await}
</Table>

<style lang="scss">
	.edit {
		display: flex;
		justify-content: center;
	}
</style>
