<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import {
		type Participant,
		// type NewParticipant,
		// new_participant as new_participant_inner,
		// get_teams,
		get_participants
	} from '$lib';

	// $: participant = {
	// 	first_name: '',
	// 	last_name: '',
	// 	team_id: ''
	// } as NewParticipant & { team_id: string };

	$: new_participants = [] as Participant[];

	function edit_participant(participant: Participant) {
		alert(`Editing participant ${participant.first_name}`);
	}

	// async function new_participant() {
	// 	try {
	// 		const db_participant = await new_participant_inner(participant, +participant.team_id);
	// 		new_participants = [...new_participants, db_participant!];

	// 		participant = { first_name: '', last_name: '', team_id: '' };
	// 	} catch (e) {
	// 		const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
	// 		console.error(e);
	// 		return alert(`Failed to add team.\n${msg}`);
	// 	}
	// }
</script>

<Banner title="Participants" subtitle="Click on a participant to edit them." />
<!-- <form on:submit={new_participant}>
	<label>
		first name
		<input type="text" bind:value={participant.first_name} />
	</label>
	<label>
		last name
		<input type="text" bind:value={participant.last_name} />
	</label>
	<label>
		team
		<select name="team" bind:value={participant.team_id}>
			{#await get_teams() then teams}
				{#each teams as team}
					{#if !team.individual}
						<option value={team.id}>{team.name}</option>
					{/if}
				{/each}
			{:catch error}
				<p class="error">
					failed to get teams...<br />
				</p>
				<pre>{error}</pre>
			{/await}
		</select>
	</label>
	<input type="submit" value="add" />
</form> -->

<Table
	headings={['ID', 'First Name', 'Last Name', 'Team ID', 'Team Name', 'Individual?']}
	highlighted_columns={[1, 2]}
>
	{#await get_participants() then participants}
		{#each [participants, new_participants].flat() as participant}
			<tr>
				<td class="id">{participant.id}</td>
				<th class="first_name">{participant.first_name}</th>
				<th class="last_name">{participant.last_name}</th>
				<td class="team_id">{participant.team_id}</td>
				<td class="team_name">{participant.team_name}</td>
				<th class="individual">
					<input type="checkbox" checked={participant.team_individual} on:click|preventDefault />
				</th>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_participant(participant)}>Edit</button>
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
