<script lang="ts">
  import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
  import LinearProgress from '@smui/linear-progress';
  import Checkbox from '@smui/checkbox';
  import type { UserDto } from '$lib/api/models';
  import IconButton from '@smui/icon-button';
  import type UserToDelete from '$lib/components/ElementToDelete';

  export let data: UserDto[] | null;
  export let deleteUser: (user: UserToDelete) => void;
</script>

<DataTable table$aria-label="User list" style="width: 100%;">
  <Head>
    <Row>
      <Cell>ID</Cell>
      <Cell>Username</Cell>
      <Cell>First name</Cell>
      <Cell>Last name</Cell>
      <Cell>Email</Cell>
      <Cell>Active</Cell>
      <Cell>Delete</Cell>
    </Row>
  </Head>
  <Body>
    {#if !!data}
      {#if data.length === 0}
        <Row>
          <Cell colspan="7">No data</Cell>
        </Row>
      {/if}

      {#each data as item (item.id)}
        <Row>
          <Cell>{item.id}</Cell>
          <Cell>{item.name}</Cell>
          <Cell>{item.firstName ?? ''}</Cell>
          <Cell>{item.lastName ?? ''}</Cell>
          <Cell>{item.email ?? ''}</Cell>
          <Cell>
            <Checkbox checked={item.active} disabled />
          </Cell>
          <Cell>
            <IconButton
              class="material-icons"
              on:click={() => deleteUser({ id: item.id, name: item.name })}
              >delete</IconButton
            >
          </Cell>
        </Row>
      {/each}
    {:else}
      <Row>
        <Cell colspan="7">Loading...</Cell>
      </Row>
    {/if}
  </Body>

  <LinearProgress
    indeterminate
    closed={!!data}
    aria-label="Data is being loaded..."
    slot="progress"
  />
</DataTable>
