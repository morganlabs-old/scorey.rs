<script lang="ts">
	import { appWindow as app_window } from '@tauri-apps/api/window';
	import { new_team, new_participant, delete_team, type NewTeam, type NewParticipant } from '$lib';

	$: team = {
		name: '',
		individual: false
	} as NewTeam;

	$: participant = {
		first_name: '',
		last_name: ''
	} as NewParticipant;

	async function add_team() {
		const db_team = await new_team(team);
		if (!db_team) return;

		if (team.individual) {
			const db_participant = await new_participant(participant, db_team.id);
			if (!db_participant) return await delete_team(db_team.id);

			team = { name: '', individual: true };
			participant = { first_name: '', last_name: '' };
		} else {
			team = { name: '', individual: false };
			participant = { first_name: '', last_name: '' };
		}
	}
</script>

<form on:submit={add_team}>
	<label>
		<input
			type="text"
			placeholder={`${participant.first_name ? participant.first_name + 's' : 'An'} Amazing Team`}
			bind:value={team.name}
		/>
		<span>Team Name</span>
	</label>
	<label>
		<input type="checkbox" bind:checked={team.individual} />
		<span>Individual</span>
	</label>
	{#if team.individual}
		<label>
			<input type="text" placeholder="John" bind:value={participant.first_name} />
			<span>Participant First Name</span>
		</label>
		<label>
			<input type="text" placeholder="Doe" bind:value={participant.last_name} />
			<span>Participant Last Name</span>
		</label>
	{/if}
	<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
	<button class="primary" type="submit">Save</button>
</form>
