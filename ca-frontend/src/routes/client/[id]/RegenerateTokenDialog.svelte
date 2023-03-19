<script lang="ts">
  import Dialog, { Content, Title } from '@smui/dialog';
  import { DatePicker } from 'date-picker-svelte';
  import Button, { Label } from '@smui/button';
  import toast from 'svelte-french-toast';
  import { regenerateClientToken } from '../../../api/clients/clients';
  import CustomDate from '../../../lib/CustomDate';
  import type { ClientDto } from '../../../api/models';

  export let id: string | null = null;
  export let onClose: (client?: ClientDto) => void;

  let loading = false;
  let date = new Date();

  const minDate = new CustomDate().addDays(1);

  const onOk = async () => {
    if (!id) return;

    loading = true;
    try {
      const created = await toast.promise(
        regenerateClientToken(id, {
          validUntil: date.toISOString(),
          name: null,
        }),
        {
          loading: 'Generating new token...',
          success: 'Token generated',
          error: 'Failed to generate new token',
        }
      );

      onClose(created);
    } finally {
      loading = false;
    }
  };
</script>

<Dialog
  open={!!id}
  aria-labelledby="create-user-title"
  aria-describedby="create-user-content"
>
  <Title id="create-user-title">Create new user</Title>
  <Content id="create-user-content">
    <p>Valid until</p>
    <DatePicker bind:value={date} min={minDate} />
  </Content>
  <div class="button-container">
    <Button on:click={onOk} disabled={loading}>
      <Label>Ok</Label>
    </Button>
    <Button on:click={() => onClose()} disabled={loading}>
      <Label>Cancel</Label>
    </Button>
  </div>
</Dialog>

<style lang="scss">
  .button-container {
    display: flex;
    justify-content: flex-end;
    margin: 10px;
  }
</style>
