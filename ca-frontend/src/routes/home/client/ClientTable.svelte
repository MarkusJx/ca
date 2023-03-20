<script lang="ts">
  import type { ClientDto } from '$lib/api/models';
  import DataTable, { Body, Cell, Head, Row } from '@smui/data-table';
  import Checkbox from '@smui/checkbox';
  import IconButton from '@smui/icon-button';
  import LinearProgress from '@smui/linear-progress';
  import CustomDate from '$lib/CustomDate.js';

  export let data: ClientDto[] | null = null;
</script>

<DataTable table$aria-label="User list" style="width: 100%;">
  <Head>
    <Row>
      <Cell>ID</Cell>
      <Cell>Name</Cell>
      <Cell>Created at</Cell>
      <Cell>Valid until</Cell>
      <Cell>Active</Cell>
      <Cell>Actions</Cell>
    </Row>
  </Head>
  <Body>
    {#if !!data}
      {#if data.length === 0}
        <Row>
          <Cell colspan="6">No clients found</Cell>
        </Row>
      {/if}

      {#each data as item (item.id)}
        <Row>
          <Cell>{item.id}</Cell>
          <Cell>{item.name}</Cell>
          <Cell>{CustomDate.format(item.createdAt)}</Cell>
          <Cell>{CustomDate.format(item.validUntil)}</Cell>
          <Cell>
            <Checkbox checked={item.active} disabled />
          </Cell>
          <Cell>
            <IconButton class="material-icons" href={`/home/client/${item.id}`}>
              preview</IconButton
            >
          </Cell>
        </Row>
      {/each}
    {:else}
      <Row>
        <Cell colspan="6">Loading...</Cell>
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
