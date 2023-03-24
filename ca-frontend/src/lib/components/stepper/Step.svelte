<script lang="ts">
	export let current: number;
	export let step: number | number[];
</script>

{#if Array.isArray(step) ? step.includes(current) : current === step}
	<div class="container">
		<slot />
	</div>
{/if}

<style lang="scss">
	.container {
		display: grid;
		grid-template-columns: auto;
		grid-auto-rows: auto;
		justify-content: center;
		justify-items: center;
		margin: 0 60px;
	}
</style>
