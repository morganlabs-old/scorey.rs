<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import { type Participant, get_participants } from '$lib';

	$: new_participants = [] as Participant[];

	function edit_participant(participant: Participant) {
		alert(`Editing participant ${participant.first_name}`);
	}
</script>

<Banner title="Participants" subtitle="Click on a participant to edit them." />

<Table
	headings={['ID', 'First Name', 'Last Name', 'Team ID', 'Team Name', 'Individual?']}
	highlighted_columns={[1, 2]}
>
	{#await get_participants() then participants}
		{#each [participants, new_participants].flat() as participant}
			<tr>
				<td class="id">{participant.id}</td>
				<th class="first_name">{participant.first_name}</th>
				<th class="last_name">{participant.last_name}</th>
				<td class="team_id">{participant.team_id}</td>
				<td class="team_name">{participant.team_name}</td>
				<th class="individual">
					<input type="checkbox" checked={participant.team_individual} on:click|preventDefault />
				</th>
				<th class="edit"
					><button class="edit_btn" on:click={() => edit_participant(participant)}>Edit</button>
				</th>
			</tr>
		{/each}
	{:catch error}
		<p class="error">
			Failed to get teams...<br />
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
