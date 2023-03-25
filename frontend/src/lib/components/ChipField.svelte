<script lang="ts">
	import Chip, { Set, Text } from '@smui/chips';
	import NotchedOutline from '@smui/notched-outline';
	import FloatingLabel from '@smui/floating-label';
	import { onMount } from 'svelte';
	import { onVisible } from '$lib/util';
	import { classMap } from '@smui/common/internal';
	import { mapStyle } from '$lib/util.js';

	export let values: string[];
	export let selected: string[];
	export let title: string;
	export let focused: boolean = false;
	export let style: string | null = null;

	let notch: NotchedOutline & { notch: (width: number) => void };
	let label: FloatingLabel & { getElement: () => HTMLLabelElement };

	onMount(() => {
		onVisible(label.getElement(), (el) => {
			notch.notch(label.getElement().offsetWidth);
			setTimeout(() => {
				notch.notch(el.getBoundingClientRect().width);
			}, 140);
		});
	});
</script>

<div
	style={mapStyle('flex-direction: unset', style)}
	class={classMap({
		'mdc-text-field': true,
		'mdc-text-field--outlined': true,
		'mdc-text-field--label-floating': true,
		'mdc-text-field--textarea': true,
		'mdc-text-field--focused': focused,
	})}
>
	<NotchedOutline notched bind:this={notch}>
		<FloatingLabel floatAbove bind:this={label}>
			{title}
		</FloatingLabel>
	</NotchedOutline>
	<div class="container">
		<Set chips={values} let:chip filter bind:selected style="gap: 8px">
			<Chip {chip} touch style="margin: 0">
				<Text>{chip}</Text>
			</Chip>
		</Set>
	</div>
</div>

<style lang="scss">
	.container {
		display: flex;
		flex-wrap: wrap;
		gap: 0 8px;
		padding: 8px 6px;
	}
</style>
