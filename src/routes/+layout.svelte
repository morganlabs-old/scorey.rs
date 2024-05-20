<script lang="ts">
	import '$styles/global.scss';
	import { page } from '$app/stores';
	$: current_route = '/' + ($page.url.pathname.split('/')[1] || '');

	type Page = {
		title: string;
		href: string;
	};

	const pages: Page[] = [
		{ title: 'Leaderboard', href: '/' },
		{ title: 'Teams', href: '/teams' },
		{ title: 'Participants', href: '/participants' },
		{ title: 'Events', href: '/events' }
	];
</script>

<div class="container">
	<nav class="nav">
		<ul class="links">
			{#each pages as page}
				<li class="link_container">
					<a class="link" class:current={page.href === current_route} href={page.href}
						>{page.title}</a
					>
				</li>
			{/each}
		</ul>
	</nav>
	<main class="main">
		<slot />
	</main>
</div>

<style lang="scss">
	.nav {
		height: 3rem;
		background-color: red;
	}

	.links {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 2rem;
		list-style: none;
		height: 100%;
	}

	.link {
		&.current {
			color: green;
		}
	}
</style>
