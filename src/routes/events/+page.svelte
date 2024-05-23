<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import {
		// type NewEvent,
		type Event,
		// new_event as new_event_inner,
		get_events
	} from '$lib';

	// $: event = {
	// 	name: '',
	// 	event_type: ''
	// } as NewEvent;

	$: new_events = [] as Event[];

	function edit_event(event: Event) {
		alert(`Editing event ${event.name}`);
	}

	// async function new_event() {
	// 	try {
	// 		const db_event = await new_event_inner(event);
	// 		new_events = [...new_events, db_event!];

	// 		event = { name: '', event_type: '' };
	// 	} catch (e) {
	// 		const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
	// 		console.error(e);
	// 		return alert(`Failed to add event.\n${msg}`);
	// 	}
	// }
</script>

<Banner title="Events" subtitle="Click on an event to edit it." />

{#await get_events()}
	<p class="awaiting">Getting events...</p>
{:then events}
	<Table headings={['ID', 'Name', 'Type']} highlighted_columns={[1]}>
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
	</Table>
{:catch error}
	<p class="error">
		Failed to get events...<br />
	</p>
	<pre>{error}</pre>
{/await}

<!-- <form on:submit={new_event}>
	<label>
		Event name
		<input type="text" bind:value={event.name} />
	</label>
	<label>
		Event type
		<select name="type" bind:value={event.event_type}>
			<option value="ACADEMIC">Academic</option>
			<option value="SPORT">Sports</option>
		</select>
	</label>
	<input type="submit" value="Add" />

	<main class="events">
</form> -->

<style lang="scss">
	.edit {
		display: flex;
		justify-content: center;
	}
</style>
