<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Event, get_events } from '$lib';

	$: new_events = [] as Event[];

	function edit_event(event: Event) {
		alert(`Editing event ${event.name}`);
	}
</script>

<Banner title="Events" subtitle="Click on an event to edit it." />

<Table headings={['ID', 'Name', 'Type']} highlighted_columns={[1]}>
	{#await get_events() then events}
		{#each [events, new_events].flat() as event}
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
