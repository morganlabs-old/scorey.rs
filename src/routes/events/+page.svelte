<script lang="ts">
	import { type NewEvent, type Event, new_event as new_event_inner, get_events } from '$lib';

	$: event = {
		name: '',
		event_type: ''
	} as NewEvent;

	$: new_events = [] as Event[];

	async function new_event() {
		try {
			const db_event = await new_event_inner(event);
			new_events = [...new_events, db_event!];

			event = { name: '', event_type: '' };
		} catch (e) {
			const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
			console.error(e);
			return alert(`Failed to add event.\n${msg}`);
		}
	}
</script>

<h1>Events</h1>

<h2>Add a new event</h2>
<form on:submit={new_event}>
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
		{#await get_events()}
			<p class="awaiting">Getting events...</p>
		{:then events}
			<table>
				<thead>
					<th scope="col">ID</th>
					<th scope="col">Name</th>
					<th scope="col">Type</th>
				</thead>
				<tbody>
					{#each [events, new_events].flat() as event}
						<tr>
							<th class="id">{event.id}</th>
							<td class="name">{event.name}</td>
							<td class="type">{event.event_type}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{:catch error}
			<p class="error">
				Failed to get events...<br />
			</p>
			<pre>{error}</pre>
		{/await}
	</main>
</form>
