<script lang="ts">
  import Button, { Label } from '@smui/button';
  import Dialog, { Title, Content } from '@smui/dialog';
  import toast from 'svelte-french-toast';
  import FormField from '@smui/form-field';
  import Checkbox from '@smui/checkbox';
  import type ElementToDelete from './ElementToDelete';
  import { capitalizeFirstLetter } from '../lib/util';

  export let element: ElementToDelete | null;
  export let onClose: (userDeleted: boolean) => void;
  export let deleteElement: (
    id: string,
    opts: { deleteInDatabase: boolean }
  ) => Promise<void>;
  export let name: string;

  let loading = false;
  let deleteInDatabase = false;

  const onConfirm = async () => {
    if (!element) {
      onClose(false);
    }

    loading = true;
    try {
      await toast.promise(
        deleteElement(element?.id!, {
          deleteInDatabase,
        }),
        {
          loading: `Deleting ${name}...`,
          success: `${capitalizeFirstLetter(name)} deleted`,
          error: `Failed to delete ${name}`,
        }
      );
    } catch (_) {
      return;
    } finally {
      loading = false;
    }

    onClose(true);
  };
</script>

<Dialog
  open={!!element}
  aria-labelledby="delete-element-title"
  aria-describedby="delete-element-content"
>
  <Title id="delete-element-title">Delete {name}</Title>
  <Content id="delete-element-content">
    Are you sure you want to delete {name} '{element?.name}'?

    <div>
      <FormField>
        <Checkbox bind:checked={deleteInDatabase} />
        <span slot="label">Delete {name} rather than disabling it</span>
      </FormField>
    </div>
  </Content>
  <div class="button-container">
    <Button on:click={onConfirm} disabled={loading}>
      <Label>Yes</Label>
    </Button>
    <Button on:click={() => onClose(false)} disabled={loading}>
      <Label>No</Label>
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
