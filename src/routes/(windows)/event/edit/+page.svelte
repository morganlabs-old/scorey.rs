<script lang="ts">
	import {
		update_event as update_event_inner,
		get_event as get_event_inner,
		type Event
	} from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	export let data: { event_id: string };
	const { event_id } = data;

	$: new_event = {
		id: 0,
		name: '',
		event_type: ''
	} as Event;

	async function get_event() {
		const event = await get_event_inner(+event_id);
		new_event.id = event.id;
		new_event.name = event.name;
		new_event.event_type = event.event_type;
	}

	async function update_event() {
		try {
			await update_event_inner(new_event);
			alert('Updated event sucessfully.');
		} catch (e) {
			console.error(e);
			alert('Failed to update event.');
		}
	}
</script>

{#await get_event() then}
	<form on:submit={update_event}>
		<label>
			<input type="text" bind:value={new_event.name} />
			<span>Name</span>
		</label>
		<label>
			<select bind:value={new_event.event_type}>
				<option value="ACADEMIC">Academics</option>
				<option value="SPORT">Sports</option>
			</select>
			<span>Event Type</span>
		</label>
		<div class="actions">
			<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
			<button class="primary" type="submit">Save</button>
		</div>
	</form>
{:catch error}
	<p>{error.message}</p>
{/await}
