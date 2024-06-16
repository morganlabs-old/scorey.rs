<script lang="ts">
	import { update_event, get_event as get_event_inner, type Event } from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	export let data: { id: number };
	const { id } = data;

	$: new_event = {
		id: 0,
		name: '',
		event_type: ''
	} as Event;

	async function get_event() {
		const event = await get_event_inner(id);
		if (!event) app_window.close();

		new_event.id = event!.id;
		new_event.name = event!.name;
		new_event.event_type = event!.event_type;
	}
</script>

{#await get_event() then}
	<form on:submit|preventDefault={() => update_event(new_event)}>
		<label>
			<input type="text" placeholder="Speedreading" bind:value={new_event.name} />
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
{/await}
