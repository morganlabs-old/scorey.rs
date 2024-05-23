<script lang="ts">
	import type Page from 'src/app.d.ts';
	import { page } from '$app/stores';

	$: current_route = '/' + ($page.url.pathname.split('/')[1] || '');
	export let pages: Page[] = [];
</script>

<aside class="sidebar">
	<span class="section_header">Pages</span>
	<ul class="links">
		{#each pages as page}
			<li class="link_container" class:active={current_route == page.href}>
				<a href={page.href} class="link">{page.title}</a>
			</li>
		{/each}
	</ul>
	<span class="section_header">Danger Zone</span>
	<ul class="links">
		<li class="link_container">
			<a href="/#" class="link">Reset Database</a>
		</li>
	</ul>
</aside>

<style lang="scss">
	.sidebar {
		padding: 10px;
		min-width: 12.5rem;
		background-color: rgba(#eaeaea, 0.85);
	}

	.section_header {
		font-weight: 700;
		font-size: 0.7rem;
		opacity: 25%;
		padding: 3px 12px 6px 8px;
	}

	.link {
		display: block;
		padding: 6px 8px;

		color: black;
		text-decoration: none;
	}

	.active {
		border-radius: 5px;
		background-color: rgba(#000000, 0.1);
	}

	.link {
		font-size: 1rem;
	}
</style>
