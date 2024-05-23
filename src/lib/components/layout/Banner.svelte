<script lang="ts">
	export let title: string = '';
	export let subtitle: string = '';
</script>

<div class="banner">
	<div class="left">
		<h1 class="title">{title}</h1>
		<span class="subtitle">{subtitle}</span>
	</div>
	<div class="right">
		<slot />
	</div>
</div>

<style lang="scss">
	.banner {
		display: grid;
		grid-template-columns: repeat(2, auto);
		align-items: center;
		background-color: rgba(#eaeaea, 0.8);
		height: 64px;
		padding: 0 0.75rem;
	}

	.right {
		justify-self: flex-end;
	}

	.title {
		font-size: 1.2rem;
		opacity: 0.85;
	}

	.subtitle {
		font-size: 0.9rem;
		opacity: 0.5;
	}
</style>
