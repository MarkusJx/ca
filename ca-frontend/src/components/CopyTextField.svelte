<script lang="ts">
  import Textfield from '@smui/textfield';
  import IconButton from '@smui/icon-button';
  import { onMount } from 'svelte';
  import toast from 'svelte-french-toast';

  export let value: any = '';
  export let label: string | undefined = undefined;
  export let style: string | undefined = undefined;

  let input: { getElement(): HTMLInputElement } | undefined;
  onMount(() => {
    if (input) {
      input.getElement().readOnly = true;
    }
  });

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(value);
      toast.success('Value copied to clipboard');
    } catch (_) {
      toast.error('Failed to copy value to clipboard');
    }
  };
</script>

<Textfield variant="outlined" {value} {label} {style} bind:input>
  <IconButton
    class="material-icons"
    slot="trailingIcon"
    on:click={copy}
    style="margin: auto"
  >
    content_copy
  </IconButton>
</Textfield>
