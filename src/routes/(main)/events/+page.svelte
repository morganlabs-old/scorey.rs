<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Event, get_events } from '$lib';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { v4 as uuidv4 } from 'uuid';

	$: events = async () =>
		(await get_events()).sort((a, b) => b.event_type.localeCompare(a.event_type));

	function edit_event(event: Event) {
		const webview = new WebviewWindow(`edit_${event.id}`, {
			url: `http://localhost:5173/event/edit?id=${event.id}`
		});

		webview.once('tauri://created', async () => await webview.setTitle(`Editing ${event.name}`));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}

	function add_event() {
		const uuid = uuidv4();
		const webview = new WebviewWindow(`new_event_${uuid}`, {
			url: 'http://localhost:5173/event/new'
		});

		webview.once('tauri://created', async () => await webview.setTitle('Add new event'));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
		webview.once('tauri://close-requested', () => location.reload());
	}
</script>

<Banner title="Events" subtitle="Click on an event to edit it.">
	<button class="add" on:click={add_event}>Add</button>
</Banner>

<Table headings={['ID', 'Name', 'Type']} highlighted_columns={[1]}>
	{#await events() then events}
		{#each events as event}
			<tr>
				<td class="id">{event.id}</td>
				<th class="name">{event.name}</th>
				<td class="type">{event.event_type}</td>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_event(event)}>Edit</button>
				</th>
			</tr>
		{/each}
	{:catch error}
		<p class="error">
			Failed to get events...<br />
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
