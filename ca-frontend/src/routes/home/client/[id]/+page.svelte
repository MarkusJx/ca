<script lang="ts">
	import type { PageData } from './$types';
	import Spinner from '$lib/components/Spinner.svelte';
	import type { ClientDto, SigningRequestDto } from '$lib/api/models';
	import SigningRequestTable from './SigningRequestTable.svelte';
	import ClientInfo from './ClientInfo.svelte';
	import DeleteElementDialog from '$lib/components/DeleteElementDialog.svelte';
	import type ElementToDelete from '$lib/components/ElementToDelete';
	import { deleteClient } from '$lib/api/clients/clients.js';

	export let data: PageData & {
		client: [ClientDto | null, SigningRequestDto[] | null];
	};

	const [client, signingRequests] = data?.client;

	let deleteDialogClient: ElementToDelete | null = null;
	const onClientDeleted = (deleted: boolean) => {
		deleteDialogClient = null;
		if (deleted) {
			location.href = '/home/client';
		}
	};
</script>

<DeleteElementDialog
	onClose={onClientDeleted}
	element={deleteDialogClient}
	name="client"
	deleteElement={deleteClient}
/>

{#if !client || !signingRequests}
	<Spinner />
{:else}
	<div class="container">
		<div class="inner-grid">
			<ClientInfo data={client} bind:deleteDialogClient />
			<SigningRequestTable data={signingRequests} />
		</div>
	</div>
{/if}

<style lang="scss">
	.container {
		display: flex;
		width: 100%;
		justify-content: center;
		padding: 20px 0;
		align-items: flex-end;
	}

	.inner-grid {
		display: grid;
		grid-row-gap: 20px;
	}
</style>
