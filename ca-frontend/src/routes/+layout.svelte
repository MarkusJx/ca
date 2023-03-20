<script lang="ts">
  import { Toaster } from 'svelte-french-toast';
</script>

<slot />

<Toaster toastOptions={{ style: 'font-family: Roboto' }} />
