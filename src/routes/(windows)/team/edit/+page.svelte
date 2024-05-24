<script lang="ts">
	import { update_team as update_team_inner, get_team as get_team_inner, type Team } from '$lib';
	import { WebviewWindow, appWindow as app_window } from '@tauri-apps/api/window';

	export let data: { team_id: string };
	const { team_id } = data;

	$: new_team = {
		id: 0,
		name: '',
		individual: false,
		points: 0
	} as Team;

	async function get_team() {
		const team = await get_team_inner(+team_id);
		new_team.id = team.id;
		new_team.name = team.name;
		new_team.individual = team.individual;
		new_team.points = team.points;
		return team;
	}

	async function update_team() {
		try {
			await update_team_inner(new_team);
			alert('Updated team sucessfully.');
		} catch (e) {
			console.error(e);
			alert('Failed to update team.');
		}
	}

	function select_events(team: Team) {
		const webview = new WebviewWindow(`enroll_${team.id}`, {
			url: `http://localhost:5173/team/enroll?id=${team.id}`
		});

		webview.once('tauri://created', () => webview.setTitle(`Enrolling ${team.name} into events`));
		webview.once('tauri://error', (e) => console.error('Failed to create window', e));
	}
</script>

{#await get_team() then team}
	<form on:submit={update_team}>
		<label>
			Team Name
			<input type="text" bind:value={new_team.name} />
		</label>
		<label>
			Points
			<input type="number" bind:value={new_team.points} />
		</label>
		<label>
			Events
			<button on:click|preventDefault={() => select_events(team)}>Select events</button>
		</label>
		<label>
			Individual
			<input type="checkbox" checked={new_team.individual} on:click|preventDefault disabled />
		</label>
		<button type="button" on:click|preventDefault={() => app_window.close()}>Cancel</button>
		<button class="primary" type="submit">Save</button>
	</form>
{:catch error}
	<p>{error.message}</p>
{/await}
