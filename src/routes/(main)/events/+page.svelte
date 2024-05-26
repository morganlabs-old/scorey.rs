<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import {
		type Event,
		get_events,
		delete_event as delete_event_inner,
		new_popup_window
	} from '$lib';

	const add_event = () => new_popup_window('/event/new', 'Add new event');
	const edit_event = (event: Event) =>
		new_popup_window(`/event/edit?id=${event.id}`, `Editing ${event.name}`);

	async function delete_event(event_id: number) {
		try {
			await delete_event_inner(event_id);
			alert('Deleted event sucessfully.');
			location.reload();
		} catch (e) {
			alert(`Failed to delete event.\n${e}`);
			console.error(e);
		}
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
					><button class="delete_btn" on:click={() => delete_event(event.id)}>Delete</button></th
				>
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
