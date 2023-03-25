<script lang="ts">
	import { page } from '$app/stores';
	import Center from '$lib/components/Center.svelte';
	import { AltErrors, httpErrors } from '$lib/httpErrors';
	import NoSelect from '$lib/components/NoSelect.svelte';

	const altError = AltErrors[$page.status];
	let altErrorContainer: HTMLHeadingElement;

	const handleMouseEnter = () => {
		altErrorContainer.classList.add('hovered');
	};

	const handleMouseLeave = () => {
		altErrorContainer.classList.remove('hovered');
	};
</script>

<Center>
	<NoSelect>
		<div
			class="container"
			on:mouseenter={handleMouseEnter}
			on:mouseleave={handleMouseLeave}
		>
			<div class="heading-container">
				<h1>{$page.status}</h1>
				<div class="divider" />
				<h2>{httpErrors[$page.status]}</h2>
			</div>
			{#if altError}
				<h3 class="alt-error" bind:this={altErrorContainer}>{altError}</h3>
			{/if}
		</div>
	</NoSelect>
</Center>

<style lang="scss">
	.container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		font-family: Roboto, sans-serif;

		.alt-error {
			font-size: 1.5rem;
			font-weight: 100;
			margin: -5px 0 0;
			transition: all ease-in-out 0.35s;
			opacity: 0;
		}

		:global(.alt-error.hovered) {
			opacity: 1;
			margin-top: 15px;
			transition-delay: 0.25s;
			pointer-events: all;
		}
	}

	.heading-container {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		font-weight: 100;
		gap: 1.25rem;

		h1 {
			font-size: 2.5rem;
			font-weight: inherit;
			margin: 0;
		}

		.divider {
			width: 1px;
			height: 2.5rem;
			background-color: white;
		}

		h2 {
			font-size: 2.3rem;
			font-weight: inherit;
			margin: 0;
		}
	}
</style>
