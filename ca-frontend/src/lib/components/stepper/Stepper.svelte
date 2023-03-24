<script lang="ts">
	import { Steps } from 'svelte-steps';
	import type IStep from '$lib/components/stepper/IStep';

	export let steps: IStep[];
	export let clickable: boolean = true;
	export let current: number = 0;
	export let size: string | undefined = undefined;
	export let line: string | undefined = undefined;
	export let divider: boolean = false;
	export let primary: string | undefined = undefined;
</script>

<div>
	<Steps {steps} {clickable} {current} {line} {size} {primary} />
	{#if divider}
		<div class="divider" />
	{/if}
	<slot />
</div>

<style lang="scss">
	.divider {
		width: calc(100% - 120px);
		height: 1px;
		background-color: var(--mdc-theme-secondary, #4c4c4c);
		margin: 30px 60px;
	}
</style>
