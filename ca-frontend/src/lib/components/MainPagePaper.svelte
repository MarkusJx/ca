<script lang="ts">
	import Paper, { Title, Content } from '@smui/paper';
	import Ripple from '@smui/ripple';
	import { createEventDispatcher } from 'svelte';
	import NoSelect from '$lib/components/NoSelect.svelte';

	export let href: string | undefined = undefined;
	export let title: string;

	const dispatch = createEventDispatcher();

	const onClick = (e: MouseEvent) => {
		dispatch('click', e);
		if (href) {
			location.href = href;
		}
	};
</script>

<Paper style="padding: 0" on:click={onClick}>
	<NoSelect>
		<div use:Ripple={{ surface: true }} class="paper-ripple-container">
			<Title style="text-align: center">{title}</Title>
			<Content style="text-align: center">
				<slot />
			</Content>
		</div>
	</NoSelect>
</Paper>

<style lang="scss">
	.paper-ripple-container {
		padding: 24px 16px;
		cursor: pointer;
	}
</style>
