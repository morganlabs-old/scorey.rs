<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import {
		type Participant,
		get_participants,
		delete_participant as delete_participant_inner
	} from '$lib';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { v4 as uuidv4 } from 'uuid';

	function edit_participant(participant: Participant) {
		const webview = new WebviewWindow(`edit_${participant.id}`, {
			url: `http://localhost:5173/participant/edit?id=${participant.id}`,
			width: 500,
			height: 270,
			center: true
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
			url: 'http://localhost:5173/participant/new',
			width: 500,
			height: 270,
			center: true
		});

		webview.once('tauri://created', async () => await webview.setTitle('Add new participant'));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}

	async function delete_participant(participant_id: number) {
		try {
			await delete_participant_inner(participant_id);
			alert('Deleted participant sucessfully.');
			location.reload();
		} catch (e) {
			alert(`Failed to delete participant.\n${e}`);
			console.error(e);
		}
	}
</script>

<Banner title="Participants">
	<button class="add" on:click={add_participant}>+</button>
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
				<th class="edit">
					<button class="edit_btn" on:click={() => edit_participant(participant)}>Edit</button>
				</th>
				<th class="delete">
					<button class="delete_btn" on:click={() => delete_participant(participant.id)}
						>Delete</button
					>
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
