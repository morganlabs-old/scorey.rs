<script lang="ts">
	export let headings: string[];
	export let highlighted_columns: number[] = [0];
</script>

<table class="table">
	<thead>
		{#each headings as heading, idx}
			{#if highlighted_columns.includes(idx)}
				<th>{heading}</th>
			{:else}
				<td>{heading}</td>
			{/if}
		{/each}
	</thead>
	<tbody>
		<slot />
	</tbody>
</table>

<style lang="scss">
	.table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.75rem;

		thead {
			border-block: 1px solid rgba(#000, 0.1);

			* {
				border-inline: 1px solid rgba(#000, 0.1);
				vertical-align: middle;
			}
		}

		thead,
		tbody {
			text-align: left;
		}

		:global(tr) {
			&:nth-child(even) {
				background-color: #f2f2f2;
			}
		}

		:global(th),
		:global(td) {
			font-weight: 590;
			padding: 5px;
		}

		:global(td) {
			opacity: 0.5;
		}
	}
</style>
