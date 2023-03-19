<script lang="ts">
  import DataTable, { Body, Cell, Head, Row } from '@smui/data-table';
  import type { SigningRequestDto } from '../../../api/models';
  import CustomDate from '$lib/CustomDate.js';

  export let data: SigningRequestDto[];
</script>

<DataTable table$aria-label="Signing requests">
  <Head>
    <Row>
      <Cell>Issued at</Cell>
      <Cell>Serial number</Cell>
      <Cell>Subject name</Cell>
    </Row>
  </Head>
  <Body>
    {#if data.length === 0}
      <Row>
        <Cell colspan="3">No signing requests found</Cell>
      </Row>
    {/if}

    {#each data as item}
      <Row>
        <Cell>{CustomDate.format(item.issuedAt)}</Cell>
        <Cell>{item.serialNumber}</Cell>
        <Cell>{item.subjectName}</Cell>
      </Row>
    {/each}
  </Body>
</DataTable>
