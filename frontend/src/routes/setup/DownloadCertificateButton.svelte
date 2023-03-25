<script lang="ts">
	import Button from '@smui/button';
	import { downloadFile } from '$lib/util';

	export let content: string;
	export let download: string;

	const handleClick = () => {
		downloadFile([content], download);
	};
</script>

<div style="margin-top: 10px">
	<Button on:click={handleClick}>
		<slot />
	</Button>
</div>
