<script lang="ts">
	import Button from '@smui/button';

	export let current: number;
	export let numSteps: number;
	export let disabled: boolean = false;

	const handleNext = () => {
		if (current < numSteps) {
			current++;
		}
	};
</script>

<Button on:click={handleNext} {disabled}>
	<slot />
</Button>
