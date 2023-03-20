<script lang="ts">
  import Textfield from '@smui/textfield';
  import HelperText from '@smui/textfield/helper-text';

  export let label: string;
  export let errors: string[];
  export let value: string;
  export let variant: string = 'outlined';
  export let name: string = label?.toLowerCase();
  export let type: string = 'text';
  export let errorText: string = '';
  export let required: boolean = false;
  export let disabled: boolean = false;

  let invalid: boolean = false;

  const hasError = (field: string) => {
    return !!errors.find((key) => key.startsWith(field + '.'));
  };

  const onChange = () => {
    invalid = hasError(name);
  };
</script>

{#if !!errorText}
  <Textfield
    {variant}
    bind:value
    bind:invalid
    {label}
    {type}
    on:change={onChange}
    {required}
    {disabled}
  >
    <HelperText validationMsg slot="helper">{errorText}</HelperText>
  </Textfield>
{:else}
  <Textfield
    {variant}
    bind:value
    bind:invalid
    {label}
    {type}
    {disabled}
    {required}
    on:change={onChange}
  />
{/if}
