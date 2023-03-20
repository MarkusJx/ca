<script lang="ts">
  import Center from '$lib/components/Center.svelte';
  import MainPageContainer from '$lib/components/MainPageContainer.svelte';
  import MainPagePaper from '$lib/components/MainPagePaper.svelte';
  import { KeycloakAdapter } from '$lib/keycloak.js';
  import { getCaCertificate } from '$lib/api/certificates/certificates';
  import toast from 'svelte-french-toast';
  import { downloadFile } from '$lib/util';

  const downloadCertificate = async () => {
    const cert = await toast.promise(getCaCertificate(), {
      loading: 'Downloading certificate...',
      success: 'Certificate downloaded',
      error: 'Failed to download certificate',
    });

    downloadFile([cert], 'ca.crt');
  };
</script>

<Center>
  <MainPageContainer>
    <MainPagePaper title="Download certificate" on:click={downloadCertificate}>
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
