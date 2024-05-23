<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { type Team, type Participant } from '$lib';

	$: team = {
		name: '',
		individual: false
	};

	$: participant = {
		first_name: '',
		last_name: ''
	};

	async function new_team() {
		const { name, individual } = team;

		try {
			const db_team = (await invoke('new_team', { name, individual })) as Team;

			if (individual) {
				const db_participant = await new_participant(db_team.id).catch((e) =>
					alert('Failed to add participant. Please try again.')
				);
				console.log(db_participant);
			}
		} catch (e) {
			const msg = (e as Record<string, string>).DatabaseNewEntryFailure;
			console.error(e);
			return alert(`Failed to add team.\n${msg}`);
		}

		team = { name: '', individual };
		participant = { first_name: '', last_name: '' };
	}

	async function new_participant(team_id: number): Promise<Participant> {
		const { first_name, last_name } = participant;

		const db_participant = (await invoke('new_participant', {
			first_name,
			last_name,
			team_id
		}).catch((e) => alert(e))) as Participant;

		return db_participant;
	}
</script>

<h1>Teams</h1>

<h2>Add a new team</h2>
<form on:submit={new_team}>
	<label>
		Team Name
		<input type="text" bind:value={team.name} />
	</label>
	<label>
		Individual?
		<input type="checkbox" bind:checked={team.individual} />
	</label>
	{#if team.individual}
		<h2>Add a new participant</h2>
		<label>
			First Name
			<input type="text" bind:value={participant.first_name} />
		</label>
		<label>
			Last Name
			<input type="text" bind:value={participant.last_name} />
		</label>
	{/if}
	<input type="submit" value="Add" />
</form>
