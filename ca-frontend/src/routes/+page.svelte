<script lang="ts">
  import Center from '$lib/components/Center.svelte';
  import MainPageContainer from '$lib/components/MainPageContainer.svelte';
  import MainPagePaper from '$lib/components/MainPagePaper.svelte';
  import { KeycloakAdapter } from '$lib/keycloak.js';
</script>

<Center>
  <MainPageContainer>
    <MainPagePaper title="Download certificate" href="">
      Download the CA certificate
    </MainPagePaper>
    {#if KeycloakAdapter.authenticated}
      <MainPagePaper title="Home" href="/home">
        Go to the home page
      </MainPagePaper>
    {:else}
      <MainPagePaper title="Login" href="/home">
        Login to the application
      </MainPagePaper>
    {/if}
  </MainPageContainer>
</Center>
