<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { get_teams } from '$lib';

	async function get_teams_and_sort() {
		const teams = await get_teams();
		teams.sort((a, b) => b.points - a.points);
		return teams;
	}
</script>

<Banner title="Leaderboard" subtitle="See whose winning!"></Banner>
{#await get_teams_and_sort()}
	<p>loading...</p>
{:then teams}
	<Table headings={['Position', 'Name', 'Points']} highlighted_columns={[0, 1]}>
		{#each teams as team, idx}
			<tr class="row">
				<th class="position">{idx + 1}</th>
				<th class="name">{team.name}</th>
				<td class="points">{team.points}</td>
			</tr>
		{/each}
	</Table>
{:catch error}
	<p>{error.message}</p>
{/await}
