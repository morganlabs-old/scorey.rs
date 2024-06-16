<script lang="ts">
	import { update_team, get_team as get_team_inner, new_popup_window, type Team } from '$lib';
	import { appWindow as app_window } from '@tauri-apps/api/window';

	export let data: { id: number };
	const { id } = data;

	$: new_team = {
		id: 0,
		name: '',
		individual: false,
		points: 0
	} as Team;

	async function get_team() {
		const team = await get_team_inner(id);
		if (!team) app_window.close();

		new_team.id = team!.id;
		new_team.name = team!.name;
		new_team.individual = team!.individual;
		new_team.points = team!.points;

		return team!;
	}

	const select_events = (team: Team) =>
		new_popup_window(`/enroll/team?id=${team.id}`, `Enroll ${team.name} into events`);
</script>

{#await get_team() then team}
	<form on:submit|preventDefault={() => update_team(new_team)}>
		<label>
			<input type="text" placeholder="An Amazing Team" bind:value={new_team.name} />
			<span>Team Name</span>
		</label>
		<label>
			<input type="number" placeholder="0" bind:value={new_team.points} />
			<span>Points</span>
		</label>
		<label>
			<button on:click|preventDefault={() => select_events(team)}>Select events</button>
			<span>Events</span>
		</label>
		<label>
			<input type="checkbox" checked={new_team.individual} on:click|preventDefault disabled />
			<span>Individual</span>
		</label>
		<div class="actions">
			<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
			<button class="primary" type="submit">Save</button>
		</div>
	</form>
{/await}
