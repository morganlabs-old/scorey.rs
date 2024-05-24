<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_event as new_event_inner, type NewEvent } from '$lib';

	$: event = {
		name: '',
		event_type: ''
	} as NewEvent;

	async function new_event() {
		try {
			await new_event_inner(event);
			alert('Added new event.');
		} catch (e) {
			console.error(e);
			alert('Failed to add new event.');
		}
	}
</script>

<form on:submit={new_event}>
	<label>
		<input type="text" bind:value={event.name} />
		<span>Name</span>
	</label>
	<label>
		<select bind:value={event.event_type}>
			<option value="ACADEMIC">Academics</option>
			<option value="SPORT">Sports</option>
		</select>
		<span>Event Type</span>
	</label>
	<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
	<button class="primary" type="submit">Save</button>
</form>
