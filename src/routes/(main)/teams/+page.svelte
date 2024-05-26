<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { get_teams, delete_team as delete_team_inner, new_popup_window, type Team } from '$lib';

	const add_team = () => new_popup_window('/team/new', 'Add new team');
	const edit_team = (team: Team) => new_popup_window(`/team/edit?edit=${team.id}`, 'Edit team');

	async function delete_team(team_id: number) {
		try {
			await delete_team_inner(team_id);
			alert('Deleted team sucessfully.');
			location.reload();
		} catch (e) {
			alert(`Failed to delete team.\n${e}`);
			console.error(e);
		}
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
					><button class="edit_btn" on:click={() => delete_team(team.id)}>Delete</button>
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
