<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Event, get_events as get_events_inner, delete_event, new_popup_window } from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	const add_event = () => new_popup_window('/new/event', 'Add new event');
	const edit_event = (event: Event) =>
		new_popup_window(`/edit/event?id=${event.id}`, `Editing ${event.name}`);

	async function get_events() {
		const events = await get_events_inner();
		if (!events) app_window.close();
		return events!;
	}
</script>

<Banner title="Events">
	<button class="add" on:click={add_event}>+</button>
</Banner>

<Table headings={['ID', 'Name', 'Type']} highlighted_columns={[1]}>
	{#await get_events() then events}
		{#each events as event}
			<tr>
				<td class="id">{event.id}</td>
				<th class="name">{event.name}</th>
				<td class="type">{event.event_type}</td>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_event(event)}>Edit</button></th
				>
				<th class="delete"
					><button
						class="delete_btn"
						on:click={() => {
							delete_event(event.id);
							location.reload();
						}}>Delete</button
					></th
				>
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
