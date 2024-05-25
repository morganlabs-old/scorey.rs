<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_event, type NewEvent } from '$lib';

	$: event = {
		name: '',
		event_type: ''
	} as NewEvent;

	async function add_event() {
		const db_event = await new_event(event);
		if (!db_event) return;
		event = { name: '', event_type: '' };
	}
</script>

<form on:submit={add_event}>
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
