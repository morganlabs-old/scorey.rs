<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Team, get_teams } from '$lib';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { v4 as uuidv4 } from 'uuid';

	$: teams = get_teams();

	function edit_team(team: Team) {
		const webview = new WebviewWindow(`edit_${team.id}`, {
			url: `http://localhost:5173/team/${team.id}/edit`
		});

		webview.once('tauri://created', async () => await webview.setTitle(`Editing ${team.name}`));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}

	function add_team() {
		const uuid = uuidv4();
		const webview = new WebviewWindow(`new_team_${uuid}`, {
			url: 'http://localhost:5173/team/new'
		});

		webview.once('tauri://created', async () => await webview.setTitle('Add new team'));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}
</script>

<Banner title="Teams" subtitle="Click on a team to edit it.">
	<button class="add" on:click={add_team}>Add</button>
</Banner>

<Table headings={['ID', 'Name', 'Individual?', 'Points']} highlighted_columns={[1]}>
	{#await teams then teams}
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
