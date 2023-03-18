<script lang="ts">
  import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
  import LinearProgress from '@smui/linear-progress';
  import Checkbox from '@smui/checkbox';
  import { list } from '../../api/users/users';
  import type { UserDto } from '../../api/models';
  import { toast } from 'svelte-french-toast';
  import { onMount } from 'svelte';

  let items: UserDto[] = [];
  let loaded = false;

  onMount(async () => {
    try {
      items = await list();
    } catch (e) {
      toast.error('Failed to load data');
    } finally {
      loaded = true;
    }
  });
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
    </Row>
  </Head>
  <Body>
    {#each items as item (item.id)}
      <Row>
        <Cell>{item.id}</Cell>
        <Cell>{item.name}</Cell>
        <Cell>{item.firstName ?? ''}</Cell>
        <Cell>{item.lastName ?? ''}</Cell>
        <Cell>{item.email ?? ''}</Cell>
        <Cell>
          <Checkbox checked={item.active} disabled />
        </Cell>
      </Row>
    {/each}
  </Body>

  <LinearProgress
    indeterminate
    bind:closed={loaded}
    aria-label="Data is being loaded..."
    slot="progress"
  />
</DataTable>
