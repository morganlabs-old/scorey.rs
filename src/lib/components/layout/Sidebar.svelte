<script lang="ts">
	import { page } from '$app/stores';
	import type Page from '$types';

	$: current_route = '/' + ($page.url.pathname.split('/')[1] || '');
	export let pages: Page[] = [];
</script>

<aside class="sidebar">
	<span class="section_header">Pages</span>
	<ul class="links">
		{#each pages as page}
			<li class="link_container" class:active={current_route == page.href}>
				<a href={page.href} class="link">
					<div class="icon_container">
						<span class="icon">
							{@html page.icon}
						</span>
					</div>
					<span>{page.title}</span>
				</a>
			</li>
		{/each}
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

	.icon_container {
		display: block;
		width: auto;
		height: 14px;
		aspect-ratio: 1;
		overflow-y: hidden;
	}

	.icon {
		width: 100%;
		height: 100%;

		:global(*) {
			stroke: #007aff;
		}
	}

	.link {
		display: flex;
		padding: 6px 8px;
		align-items: center;
		gap: 0.25rem;

		color: black;
		opacity: 0.85;
		text-decoration: none;
		font-size: 1rem;
	}

	.active {
		border-radius: 5px;
		background-color: rgba(#000000, 0.1);
	}
</style>
