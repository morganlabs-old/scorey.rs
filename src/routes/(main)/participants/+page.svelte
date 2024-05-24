<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Participant, get_participants } from '$lib';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { v4 as uuidv4 } from 'uuid';

	function edit_participant(participant: Participant) {
		const webview = new WebviewWindow(`edit_${participant.id}`, {
			url: `http://localhost:5173/participant/${participant.id}/edit`
		});

		webview.once(
			'tauri://created',
			async () =>
				await webview.setTitle(`Editing ${participant.first_name} ${participant.last_name}`)
		);
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}

	function add_participant() {
		const uuid = uuidv4();
		const webview = new WebviewWindow(`new_participant_${uuid}`, {
			url: 'http://localhost:5173/participant/new'
		});

		webview.once('tauri://created', async () => await webview.setTitle('Add new participant'));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}
</script>

<Banner title="Participants" subtitle="Click on a participant to edit them.">
	<button class="add" on:click={add_participant}>Add</button>
</Banner>

<Table
	headings={['ID', 'First Name', 'Last Name', 'Team ID', 'Team Name', 'Individual?']}
	highlighted_columns={[1, 2]}
>
	{#await get_participants() then participants}
		{#each participants as participant}
			<tr>
				<td class="id">{participant.id}</td>
				<th class="first_name">{participant.first_name}</th>
				<th class="last_name">{participant.last_name}</th>
				<td class="team_id">{participant.team_id}</td>
				<td class="team_name">{participant.team_name}</td>
				<th class="individual">
					<input type="checkbox" checked={participant.team_individual} on:click|preventDefault />
				</th>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_participant(participant)}>Edit</button>
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
