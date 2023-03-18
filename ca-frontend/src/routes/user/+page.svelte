<script lang="ts">
  import UserTable from './UserTable.svelte';
  import Button, {Label} from '@smui/button';
  import Dialog, { Title, Content, Actions } from '@smui/dialog';
  import { onMount } from "svelte";
  import type LayoutData from "../../types/LayoutData";

  export let data: LayoutData;

  onMount(() => {
    if (!data.keycloak?.hasRealmRole('admin')) {
      window.location.href = '/';
    }
  });

  let createDialogOpen = false;
</script>

<Dialog
  bind:open={createDialogOpen}
  aria-labelledby="simple-title"
  aria-describedby="simple-content"
>
  <Title id="simple-title">Create new user</Title>
  <Content id="simple-content">Super awesome dialog body text?</Content>
  <Actions>
    <Button>
      <Label>No</Label>
    </Button>
    <Button>
      <Label>Yes</Label>
    </Button>
  </Actions>
</Dialog>

<div class="button-container">
  <div class="create-user-button-container">
    <Button class="create-user-button" on:click={() => createDialogOpen = true}
      >Create new user</Button
    >
  </div>
</div>
<UserTable />

<style lang="scss">
  .button-container {
    display: flex;
    justify-content: flex-end;

    .create-user-button-container {
      margin: 15px;
    }
  }
</style>
