<script lang="ts">
	import {
		type Team,
		type NewTeam,
		type NewParticipant,
		new_team,
		new_participant,
		get_teams
	} from '$lib';

	$: team = {
		name: '',
		individual: false
	} as NewTeam;

	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	$: new_teams = [] as Team[];

	async function new_team_and_participant() {
		try {
			const db_team = await new_team(team);
			new_teams = [...new_teams, db_team!];

			if (team.individual) {
				try {
					await new_participant(participant, db_team!.id);
				} catch (e) {
					const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
					console.error(e);
					return alert(`Failed to add participant.\n${msg}`);
				}
			}

			team = { name: '', individual: team.individual };
			participant = { first_name: '', last_name: '' };
		} catch (e) {
			const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
			console.error(e);
			return alert(`Failed to add team.\n${msg}`);
		}
	}
</script>

<h1>Teams</h1>

<h2>Add a new team</h2>
<form on:submit={new_team_and_participant}>
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
	<input type="submit" value="Add" />

	<main class="teams">
		{#await get_teams()}
			<p class="awaiting">Getting teams...</p>
		{:then teams}
			<table>
				<thead>
					<th scope="col">ID</th>
					<th scope="col">Name</th>
					<th scope="col">Individual?</th>
					<th scope="col">Points</th>
				</thead>
				<tbody>
					{#each [teams, new_teams].flat() as team}
						<tr>
							<th class="id">{team.id}</th>
							<td class="name">{team.name}</td>
							<td class="individual">
								<input type="checkbox" checked={team.individual} disabled />
							</td>
							<td class="points">{team.points}</td>
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
