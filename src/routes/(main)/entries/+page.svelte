<script lang="ts">
	import Banner from '$components/layout/Banner.svelte';
	import Table from '$components/Table.svelte';
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import {
		get_teams as get_teams_inner,
		get_events as get_events_inner,
		get_team_events as get_team_events_inner
	} from '$lib';

	async function get_events() {
		const events = await get_events_inner();
		if (!events) app_window.close();
		return events!;
	}

	async function get_teams() {
		const teams = await get_teams_inner();
		if (!teams) app_window.close();
		return teams!;
	}

	async function get_team_events(id: number) {
		const events = await get_team_events_inner(id);
		if (!events) app_window.close();
		return events!;
	}
</script>

<Banner title="Event Entries" subtitle="See which events each team has entered." />
{#await get_events() then events}
	{#await get_teams() then teams}
		{#each events as event}
			<h3>{event.name}</h3>
			<Table headings={['ID', 'Name', 'Individual?']} highlighted_columns={[1]}>
				{#each teams as team}
					{#await get_team_events(team.id) then events}
						{#if events.includes(event.id)}
							<tr>
								<td>{team.id}</td>
								<th>{team.name}</th>
								<th>
									<input type="checkbox" on:click|preventDefault checked={team.individual} />
								</th>
							</tr>
						{/if}
					{/await}
				{/each}
			</Table>
		{/each}
	{/await}
{/await}

<style lang="scss">
	h3 {
		padding: 1rem;
	}
</style>
