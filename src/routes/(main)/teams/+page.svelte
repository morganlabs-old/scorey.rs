<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Team, get_teams } from '$lib';
	import { WebviewWindow } from '@tauri-apps/api/window';

	$: new_teams = [] as Team[];
	$: teams = get_teams();

	function edit_team(team: Team) {
		const webview = new WebviewWindow(`edit_${team.id}`, {
			url: `http://localhost:5173/team/${team.id}/edit`
		});

		webview.once('tauri://created', async () => await webview.setTitle(`Editing ${team.name}`));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}
</script>

<Banner title="Teams" subtitle="Click on a team to edit it." />

<Table headings={['ID', 'Name', 'Individual?', 'Points']} highlighted_columns={[1]}>
	{#await teams then teams}
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
