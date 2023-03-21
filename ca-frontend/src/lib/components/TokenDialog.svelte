<script lang="ts">
	import Dialog, { Content, Title } from '@smui/dialog';
	import CopyTextField from './CopyTextField.svelte';
	import Actions from '@smui/dialog/src/Actions.js';
	import Button from '@smui/button';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let token: string | null = null;

	const closeHandler = () => {
		token = null;
		dispatch('closed');
	};
</script>

<Dialog
	open={!!token}
	on:SMUIDialog:closed={closeHandler}
	aria-labelledby="token-title"
	aria-describedby="token-content"
>
	<Title id="token-title">Client token</Title>
	<Content id="token-content">
		Copy this token to use in your client application. You will not be able to
		see it again, but you can always generate a new one.
		<br />
		<br />
		<CopyTextField value={token} label="Token" style="width: 100%" />
	</Content>
	<Actions>
		<Button>Close</Button>
	</Actions>
</Dialog>
