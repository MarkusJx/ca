<script lang="ts">
	import ClientTable from './ClientTable.svelte';
	import type { ClientDto } from '$lib/api/models';
	import { onMount } from 'svelte';
	import { listClients } from '$lib/api/clients/clients';
	import toast from 'svelte-french-toast';
	import Button from '@smui/button';
	import Switch from '@smui/switch';
	import FormField from '@smui/form-field';
	import CreateClientDialog from './CreateClientDialog.svelte';
	import TokenDialog from '$lib/components/TokenDialog.svelte';

	let data: ClientDto[] | null = null;
	let createClientDialogOpen = false;
	let token: string | null = null;
	let includeInactive: boolean = false;

	onMount(() => {
		loadData();
	});

	const loadData = async () => {
		const loadingId = toast.loading('Loading clients...');
		try {
			data = await listClients({
				includeInactive,
			});
		} catch (_) {
			data = [];
			toast.error('Failed to load clients');
		} finally {
			toast.dismiss(loadingId);
		}
	};

	const onCreateDialogClose = (client?: ClientDto) => {
		createClientDialogOpen = false;
		if (client) {
			token = client.token ?? null;
			loadData();
		}
	};

	const handleSwitchIncludeInactive = () => {
		includeInactive = !includeInactive;
		loadData();
	};
</script>

<CreateClientDialog
	bind:open={createClientDialogOpen}
	onClose={onCreateDialogClose}
/>
<TokenDialog bind:token />

<div class="container">
	<div class="include-inactive">
		<FormField>
			<Switch
				checked={includeInactive}
				disabled={!data}
				on:click={handleSwitchIncludeInactive}
			/>
			<span slot="label">Include inactive</span>
		</FormField>
	</div>
	<div class="button-container">
		<div class="create-client-button-container">
			<Button on:click={() => (createClientDialogOpen = true)}
				>Create new client</Button
			>
		</div>
	</div>
</div>

<ClientTable {data} />

<style lang="scss">
	.container {
		display: grid;
		grid-template-columns: 50% 50%;
		grid-template-rows: 100%;

		.include-inactive {
			margin: auto 5px;
		}
	}

	.button-container {
		display: flex;
		justify-content: flex-end;

		.create-client-button-container {
			margin: 15px;
		}
	}
</style>
