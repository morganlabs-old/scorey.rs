<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Team, get_teams } from '$lib';

	function edit_team(team: Team) {
		alert(`Editing team ${team.name}`);
	}

	$: new_teams = [] as Team[];
</script>

<Banner title="Teams" subtitle="Click on a team to edit it." />

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
