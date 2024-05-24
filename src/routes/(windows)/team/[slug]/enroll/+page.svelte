<script lang="ts">
	import Table from '$components/Table.svelte';
	import { get_events, enroll_team_in_events, get_team_events } from '$lib';

	export let data: { team_id: string };
	const { team_id } = data;

	$: to_enroll = [] as number[];
	$: to_unenroll = [] as number[];
	$: team_events = get_team_events(+team_id);
	$: final_events = async () => {
		const arr = [await team_events, to_enroll].flat().filter((x) => !to_unenroll.includes(x));
		return arr;
	};

	$: events = get_events();

	async function enroll() {
		try {
			to_enroll = to_enroll.filter(async (id) => !(await team_events).includes(id));
			await enroll_team_in_events(+team_id, to_enroll);
		} catch (e) {
			console.error(e);
			alert('Failed to enroll team into events.');
		}
	}

	async function unenroll() {
		try {
			to_unenroll = to_unenroll.filter(async (id) => (await team_events).includes(id));
			await unenroll_team_in_events(+team_id, to_unenroll);
		} catch (e) {
			console.error(e);
			alert('Failed to unenroll team from events.');
		}
	}
</script>

{#await events then events}
	{#await team_events then team_events}
		{#await final_events() then final_events}
			<Table
				headings={['ID', 'Event Name', 'Type', 'Enrolled? (Y/N)']}
				highlighted_columns={[1, 2]}
			>
				{#each events as event}
					<tr>
						<td>{event.id}</td>
						<th>{event.name}</th>
						<th>{event.event_type}</th>
						<th>
							<input
								type="radio"
								name={`${event.id}_${event.name}`}
								checked={team_events.includes(event.id)}
								value={true}
								on:change={() => {
									if (!team_events.includes(event.id)) to_enroll.push(event.id);
									to_unenroll = to_unenroll.filter((x) => x !== event.id);
									console.log('To Enroll', to_enroll);
								}}
								disabled={final_events.length >= 5}
							/>
							<input
								type="radio"
								name={`${event.id}_${event.name}`}
								checked={!team_events.includes(event.id)}
								value={false}
								on:change={() => {
									to_unenroll.push(event.id);
									console.log('To Unenroll', to_unenroll);
								}}
							/>
						</th>
					</tr>
				{/each}
			</Table>
		{/await}
		<button
			on:click={() => {
				enroll();
				unenroll();
				alert('Updated team events.');
			}}>Update</button
		>
	{/await}
{/await}
