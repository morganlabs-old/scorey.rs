<script lang="ts">
	import {
		type Participant,
		type NewParticipant,
		new_participant as new_participant_inner,
		get_teams,
		get_participants
	} from '$lib';

	$: participant = {
		first_name: '',
		last_name: '',
		team_id: ''
	} as NewParticipant & { team_id: string };

	$: new_participants = [] as Participant[];

	async function new_participant() {
		try {
			const db_participant = await new_participant_inner(participant, +participant.team_id);
			new_participants = [...new_participants, db_participant!];

			participant = { first_name: '', last_name: '', team_id: '' };
		} catch (e) {
			const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
			console.error(e);
			return alert(`Failed to add team.\n${msg}`);
		}
	}
</script>

<h1>Participant</h1>

<h2>Add a new participant</h2>
<form on:submit={new_participant}>
	<label>
		First name
		<input type="text" bind:value={participant.first_name} />
	</label>
	<label>
		Last name
		<input type="text" bind:value={participant.last_name} />
	</label>
	<label>
		Team
		<select name="team" bind:value={participant.team_id}>
			{#await get_teams() then teams}
				{#each teams as team}
					{#if !team.individual}
						<option value={team.id}>{team.name}</option>
					{/if}
				{/each}
			{:catch error}
				<p class="error">
					Failed to get teams...<br />
				</p>
				<pre>{error}</pre>
			{/await}
		</select>
	</label>
	<input type="submit" value="Add" />

	<main class="participantss">
		{#await get_participants()}
			<p class="awaiting">Getting participants...</p>
		{:then participants}
			<table>
				<thead>
					<th scope="col">ID</th>
					<th scope="col">First name</th>
					<th scope="col">Last name</th>
					<th scope="col">Team ID</th>
					<th scope="col">Team name</th>
					<th scope="col">Individual?</th>
				</thead>
				<tbody>
					{#each [participants, new_participants].flat() as participant}
						<tr>
							<th class="id">{participant.id}</th>
							<td class="first_name">{participant.first_name}</td>
							<td class="last_name">{participant.last_name}</td>
							<td class="team_id">{participant.team_id}</td>
							<td class="team_name">{participant.team_name}</td>
							<td class="individual">
								<input type="checkbox" checked={participant.team_individual} disabled />
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{:catch error}
			<p class="error">
				Failed to get teams...<br />
			</p>
			<pre>{error}</pre>
		{/await}
	</main>
</form>
