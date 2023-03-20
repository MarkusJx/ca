<script lang="ts">
  import MainPagePaper from '$lib/components/MainPagePaper.svelte';
  import { KeycloakAdapter } from '$lib/keycloak.js';
  import Center from '$lib/components/Center.svelte';
  import MainPageContainer from '$lib/components/MainPageContainer.svelte';
</script>

<Center>
  <MainPageContainer>
    <MainPagePaper title="Clients" href="/home/client">
      View and edit clients
    </MainPagePaper>
    {#if KeycloakAdapter.hasRole('admin')}
      <MainPagePaper title="Users" href="/home/user">
        View and edit users
      </MainPagePaper>
    {/if}
  </MainPageContainer>
</Center>
