<script lang="ts">
	import Table from '$components/Table.svelte';
	import { get_events, enroll_team_in_events, unenroll_team_in_events } from '$lib';

	export let data: { team_id: string; team_events: number[] };
	const { team_id, team_events } = data;

	$: to_enroll = [] as number[];
	$: to_unenroll = [] as number[];
	$: final_events = [team_events, to_enroll].flat().filter((x) => !to_unenroll.includes(x));

	async function enroll() {
		try {
			to_enroll = to_enroll.filter(async (id) => !team_events.includes(id));
			await enroll_team_in_events(+team_id, to_enroll);
		} catch (e) {
			console.error(e);
			alert('Failed to enroll team into events.');
		}
	}

	async function unenroll() {
		try {
			to_unenroll = to_unenroll.filter(async (id) => team_events.includes(id));
			await unenroll_team_in_events(+team_id, to_unenroll);
		} catch (e) {
			console.error(e);
			alert('Failed to unenroll team from events.');
		}
	}
</script>

{#await get_events() then events}
	<Table headings={['ID', 'Event Name', 'Type', 'Enrolled?']} highlighted_columns={[1, 2]}>
		{#each events as event}
			<tr>
				<td>{event.id}</td>
				<th>{event.name}</th>
				<th>{event.event_type}</th>
				<th>
					<input
						type="checkbox"
						checked={final_events.includes(event.id)}
						disabled={!final_events.includes(event.id) && final_events.length >= 5}
						on:change={(e) => {
							// @ts-expect-error It thinks that checked doesn't exist..?
							if (e.target?.checked) {
								if (!team_events.includes(event.id)) to_enroll = [...to_enroll, event.id];
								else to_unenroll = to_unenroll.filter((x) => x !== event.id);
							} else {
								if (!team_events.includes(event.id))
									to_enroll = to_enroll.filter((x) => x !== event.id);
								else to_unenroll = [...to_unenroll, event.id];
							}
						}}
					/>
				</th>
			</tr>
		{/each}
	</Table>
	<button
		on:click={() => {
			enroll();
			unenroll();
			alert('Updated team events.');
		}}>Update</button
	>
{/await}
