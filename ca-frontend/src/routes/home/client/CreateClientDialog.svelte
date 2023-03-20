<script lang="ts">
  import Dialog, { Content, Title } from '@smui/dialog';
  import Button, { Label } from '@smui/button';
  import { field, form } from 'svelte-forms';
  import { between, required } from 'svelte-forms/validators';
  import toast from 'svelte-french-toast';
  import { createClient } from '$lib/api/clients/clients';
  import { DatePicker } from 'date-picker-svelte';
  import FormTextField from '$lib/components/FormTextField.svelte';
  import CustomDate from '$lib/CustomDate';
  import type { ClientDto } from '$lib/api/models';

  export let open = false;
  export let onClose: (client?: ClientDto) => void;

  let loading = false;
  let date = new Date();

  const name = field('name', '', [required(), between(3, 20)]);
  const createForm = form(name);

  const minDate = new CustomDate().addDays(1);

  const onOk = async () => {
    await createForm.validate();
    if (!$createForm.valid) {
      toast.error('Please fix the errors in the form');
      return;
    }

    try {
      loading = true;
      const created = await toast.promise(
        createClient({
          name: $name.value,
          validUntil: date.toISOString(),
        }),
        {
          loading: 'Creating client...',
          success: 'Client created',
          error: 'Failed to create client',
        }
      );

      createForm.reset();
      onClose(created);
    } finally {
      loading = false;
    }
  };

  const onCancel = () => {
    createForm.reset();
    onClose();
  };
</script>

<Dialog
  bind:open
  aria-labelledby="create-client-title"
  aria-describedby="create-client-content"
>
  <Title id="create-client-title">Create new client</Title>
  <Content id="create-client-content">
    <div class="text-field-container">
      <FormTextField
        label="Name"
        bind:value={$name.value}
        errors={$createForm.errors}
        disabled={loading}
        required
        errorText="Name must be between 3 and 20 characters long"
      />
      <p>Valid until</p>
      <DatePicker bind:value={date} min={minDate} />
    </div>
  </Content>
  <div class="button-container">
    <Button on:click={onOk} disabled={!$createForm.valid || loading}>
      <Label>Ok</Label>
    </Button>
    <Button on:click={onCancel} disabled={loading}>
      <Label>Cancel</Label>
    </Button>
  </div>
</Dialog>

<style lang="scss">
  .text-field-container {
    margin-top: 10px;
    display: grid;
    grid-auto-rows: max-content;
  }

  .button-container {
    display: flex;
    justify-content: flex-end;
    margin: 10px;
  }
</style>
