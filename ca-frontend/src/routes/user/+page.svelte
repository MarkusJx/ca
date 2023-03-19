<script lang="ts">
  import UserTable from './UserTable.svelte';
  import Button from '@smui/button';

  import { onMount } from 'svelte';
  import type LayoutData from '../../types/LayoutData';
  import CreateUserDialog from './CreateUserDialog.svelte';
  import type { UserDto } from '../../api/models';
  import toast from 'svelte-french-toast';
  import { listUsers } from '../../api/users/users';
  import DeleteElementDialog from '../../components/DeleteElementDialog.svelte';
  import type ElementToDelete from '../../components/ElementToDelete';
  import { deleteUser } from '../../api/users/users.js';

  export let data: LayoutData;

  let createDialogOpen = false;
  let deleteDialogUser: ElementToDelete | null = null;
  let userData: UserDto[] | null = null;

  onMount(() => {
    if (!data.keycloak?.hasRealmRole('admin')) {
      window.location.href = '/';
      return;
    }

    loadData();
  });

  const loadData = async () => {
    userData = null;
    try {
      userData = await listUsers({
        includeInactive: true,
      });
    } catch (e) {
      toast.error('Failed to load data');
      userData = [];
    }
  };

  const onCreateDialogClose = (userCreated: boolean) => {
    createDialogOpen = false;
    if (userCreated) {
      loadData();
    }
  };

  const onDeleteDialogClose = (userDeleted: boolean) => {
    deleteDialogUser = null;
    if (userDeleted) {
      loadData();
    }
  };
</script>

<CreateUserDialog bind:open={createDialogOpen} onClose={onCreateDialogClose} />
<DeleteElementDialog
  onClose={onDeleteDialogClose}
  element={deleteDialogUser}
  name="user"
  deleteElement={deleteUser}
/>

<div class="button-container">
  <div class="create-user-button-container">
    <Button on:click={() => (createDialogOpen = true)}>Create new user</Button>
  </div>
</div>
<UserTable data={userData} deleteUser={(id) => (deleteDialogUser = id)} />

<style lang="scss">
  .button-container {
    display: flex;
    justify-content: flex-end;

    .create-user-button-container {
      margin: 15px;
    }
  }
</style>
