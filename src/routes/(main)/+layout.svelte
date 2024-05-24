<script lang="ts">
	import Sidebar from '$components/layout/Sidebar.svelte';
	import type Page from 'src/app.d.ts';

	const pages: Page[] = [
		{ title: 'Leaderboard', href: '/', icon: '' },
		{ title: 'Participants', href: '/participants', icon: '' },
		{ title: 'Teams', href: '/teams', icon: '' },
		{ title: 'Events', href: '/events', icon: '' }
	];
</script>

<div class="container">
	<Sidebar {pages} />
	<main class="main">
		<slot />
	</main>
</div>

<style lang="scss">
	.container {
		display: flex;
		min-width: 100vw;
		min-height: 100vh;
	}

	.main {
		background-color: #fff;
		width: 100%;
		height: 100vh;
	}
</style>
