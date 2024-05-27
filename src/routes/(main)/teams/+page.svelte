<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { get_teams as get_teams_inner, delete_team, new_popup_window, type Team } from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	const add_team = () => new_popup_window('/new/team', 'Add new team');
	const edit_team = (team: Team) => new_popup_window(`/edit/team?id=${team.id}`, 'Edit team');

	async function get_teams() {
		const teams = await get_teams_inner();
		if (!teams) app_window.close();
		return teams!;
	}
</script>

<Banner title="Teams">
	<button class="add" on:click={add_team}>+</button>
</Banner>

<Table headings={['ID', 'Name', 'Individual?', 'Points']} highlighted_columns={[1]}>
	{#await get_teams() then teams}
		{#each teams as team}
			<tr class="row">
				<td class="id">{team.id}</td>
				<th class="name">{team.name}</th>
				<th class="individual">
					<input type="checkbox" checked={team.individual} on:click|preventDefault />
				</th>
				<td class="points">{team.points}</td>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_team(team)}>Edit</button>
				</th>
				<th class="delete"
					><button
						class="edit_btn"
						on:click={() => {
							delete_team(team.id);
							location.reload();
						}}>Delete</button
					>
				</th>
			</tr>
		{/each}
	{/await}
</Table>

<style lang="scss">
	.edit {
		display: flex;
		justify-content: center;
	}
</style>
