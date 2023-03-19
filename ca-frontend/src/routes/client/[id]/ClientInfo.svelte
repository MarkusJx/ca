<script lang="ts">
  import Paper, { Title, Content } from '@smui/paper';
  import type { ClientDto } from '../../../api/models';
  import ClientInfoElement from './ClientInfoElement.svelte';
  import CustomDate from '$lib/CustomDate.js';
  import Button from '@smui/button';
  import RegenerateTokenDialog from './RegenerateTokenDialog.svelte';
  import TokenDialog from '../../../components/TokenDialog.svelte';
  import { invalidateAll } from '$app/navigation';
  import type ElementToDelete from '../../../components/ElementToDelete';

  export let data: ClientDto;
  export let deleteDialogClient: ElementToDelete | null = null;

  let regenerateTokenDialogId: string | null = null;
  let token: string | null = null;

  const onRegenerateTokenDialogClose = async (client?: ClientDto) => {
    regenerateTokenDialogId = null;
    if (client) {
      token = client.token ?? null;
    }
  };

  const onRegenerateTokenClick = () => {
    regenerateTokenDialogId = data.id;
  };
</script>

<RegenerateTokenDialog
  id={regenerateTokenDialogId}
  onClose={onRegenerateTokenDialogClose}
/>
<TokenDialog bind:token on:closed={() => invalidateAll()} />

<Paper>
  <Title>Client info</Title>
  <Content>
    <ClientInfoElement label="Id" value={data.id} />
    <ClientInfoElement label="Name" value={data.name} />
    <ClientInfoElement label="Token hash" value={data.tokenHash} />
    <ClientInfoElement
      label="Created at"
      value={CustomDate.format(data.createdAt)}
    />
    <ClientInfoElement
      label="Last updated at"
      value={CustomDate.format(data.updatedAt)}
    />
    <ClientInfoElement
      label="Valid until"
      value={CustomDate.format(data.validUntil)}
    />
    <ClientInfoElement label="Active" value={data.active ? 'yes' : 'no'} />
    <div class="actions-container">
      <Button on:click={onRegenerateTokenClick}>Generate new token</Button>
      <Button on:click={() => (deleteDialogClient = data)}>Delete</Button>
    </div>
  </Content>
</Paper>

<style lang="scss">
  .actions-container {
    display: flex;
    justify-content: flex-end;
  }
</style>
