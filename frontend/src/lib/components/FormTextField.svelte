<script lang="ts">
	import Textfield from '@smui/textfield';
	import HelperText from '@smui/textfield/helper-text';

	export let label: string;
	export let errors: string[];
	export let value: string;
	export let variant: string = 'outlined';
	export let name: string = label?.toLowerCase();
	export let type: string = 'text';
	export let errorText: string = '';
	export let required: boolean = false;
	export let disabled: boolean = false;

	let invalid: boolean = false;

	const hasError = (field: string) => {
		return !!errors.find((key) => key.startsWith(field + '.'));
	};

	const onChange = () => {
		invalid = hasError(name);
	};
</script>

<div class="container">
	{#if !!errorText}
		<Textfield
			{variant}
			bind:value
			bind:invalid
			{label}
			{type}
			{required}
			{disabled}
			style="width: 100%"
			on:change={onChange}
		>
			<HelperText validationMsg slot="helper">{errorText}</HelperText>
		</Textfield>
	{:else}
		<Textfield
			{variant}
			bind:value
			bind:invalid
			{label}
			{type}
			{disabled}
			{required}
			style="width: 100%"
			on:change={onChange}
		/>
	{/if}
</div>

<style lang="scss">
	.container {
		width: 100%;
	}
</style>
