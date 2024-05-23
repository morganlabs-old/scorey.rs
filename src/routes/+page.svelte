<script lang="ts">
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
	<table>
		<thead>
			<th scope="col">ID</th>
			<th scope="col">Name</th>
			<th scope="col">Points</th>
		</thead>
		<tbody>
			{#each teams as team}
				<tr class="row">
					<th class="id">{team.id}</th>
					<td class="name">{team.name}</td>
					<td class="points">{team.points}</td>
				</tr>
			{/each}
		</tbody>
	</table>
{:catch error}
	<p>{error.message}</p>
{/await}
