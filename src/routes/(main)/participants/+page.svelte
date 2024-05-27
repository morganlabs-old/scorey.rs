<script lang="ts">
	import Table from '$components/Table.svelte';
	import Banner from '$components/layout/Banner.svelte';
	import {
		type Participant,
		get_participants as get_participants_inner,
		delete_participant as delete_participant_inner,
		get_teams,
		new_popup_window
	} from '$lib';

	async function get_participants() {
		const participants = await get_participants_inner();
		if (!participants) return [];
		return participants;
	}

	const add_participant = async () => {
		const non_individual_teams = await get_teams().then((teams) =>
			(teams || []).filter((team) => !team.individual)
		);

		if (non_individual_teams.length === 0) {
			alert('No teams available to add participant to.\nPlease create a team first.');
			return;
		}

		new_popup_window('/new/participant', 'Add new participant');
	};

	const edit_participant = (participant: Participant) =>
		new_popup_window(
			`/edit/participant?id=${participant.id}`,
			`Editing ${participant.first_name} ${participant.last_name}`
		);

	async function delete_participant(participant_id: number) {
		try {
			await delete_participant_inner(participant_id);
			alert('Deleted participant sucessfully.');
			location.reload();
		} catch (e) {
			alert(`Failed to delete participant.\n${e}`);
			console.error(e);
		}
	}
</script>

<Banner title="Participants">
	<button class="add" on:click={add_participant}>+</button>
</Banner>

<Table
	headings={['ID', 'First Name', 'Last Name', 'Team ID', 'Team Name', 'Individual?']}
	highlighted_columns={[1, 2]}
>
	{#await get_participants() then participants}
		{#each participants as participant}
			<tr>
				<td class="id">{participant.id}</td>
				<th class="first_name">{participant.first_name}</th>
				<th class="last_name">{participant.last_name}</th>
				<td class="team_id">{participant.team_id}</td>
				<td class="team_name">{participant.team_name}</td>
				<th class="individual">
					<input type="checkbox" checked={participant.team_individual} on:click|preventDefault />
				</th>
				<th class="edit">
					<button class="edit_btn" on:click={() => edit_participant(participant)}>Edit</button>
				</th>
				<th class="delete">
					<button class="delete_btn" on:click={() => delete_participant(participant.id)}
						>Delete</button
					>
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
