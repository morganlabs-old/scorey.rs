<script lang="ts">
	import Banner from '$components/layout/Banner.svelte';
	import Table from '$components/Table.svelte';
	import { get_teams, get_events, get_team_events } from '$lib';
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
