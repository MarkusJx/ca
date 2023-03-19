<script lang="ts">
  import ClientTable from './ClientTable.svelte';
  import type { ClientDto } from '../../api/models';
  import { onMount } from 'svelte';
  import { listClients } from '../../api/clients/clients';
  import toast from 'svelte-french-toast';
  import Button from '@smui/button';
  import CreateClientDialog from './CreateClientDialog.svelte';
  import TokenDialog from '../../components/TokenDialog.svelte';

  let data: ClientDto[] | null = null;
  let createClientDialogOpen = false;
  let token: string | null = null;

  onMount(() => {
    loadData();
  });

  const loadData = async () => {
    const loadingId = toast.loading('Loading clients...');
    try {
      data = await listClients({
        includeInactive: true,
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
</script>

<CreateClientDialog
  bind:open={createClientDialogOpen}
  onClose={onCreateDialogClose}
/>
<TokenDialog bind:token />

<div class="button-container">
  <div class="create-user-button-container">
    <Button on:click={() => (createClientDialogOpen = true)}
      >Create new client</Button
    >
  </div>
</div>

<ClientTable {data} />

<style lang="scss">
  .button-container {
    display: flex;
    justify-content: flex-end;

    .create-user-button-container {
      margin: 15px;
    }
  }
</style>
