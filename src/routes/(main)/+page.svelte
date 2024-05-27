<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { get_teams } from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	async function get_teams_and_sort() {
		const teams = await get_teams();
		if (!teams) app_window.close();

		teams!.sort((a, b) => b.points - a.points);
		return teams!;
	}
</script>

<Banner title="Leaderboard" subtitle="See who's winning!"></Banner>
<Table headings={['Position', 'Name', 'Points']} highlighted_columns={[0, 1]}>
	{#await get_teams_and_sort() then teams}
		{#each teams as team, idx}
			<tr class="row">
				<th class="position">{idx + 1}</th>
				<th class="name">{team.name}</th>
				<td class="points">{team.points}</td>
			</tr>
		{/each}
	{/await}
</Table>
