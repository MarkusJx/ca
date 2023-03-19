<script lang="ts">
  import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
  import IconButton from '@smui/icon-button';
  import Drawer, {
    AppContent,
    Content,
    Header,
    Title as DrawerTitle,
  } from '@smui/drawer';
  import List, { Item, Text } from '@smui/list';
  import { Toaster } from 'svelte-french-toast';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import type LayoutData from '../types/LayoutData';

  export let data: LayoutData;

  onMount(() => {
    if (!data.keycloak) {
      window.location.reload();
    }
  });

  let topAppBar: TopAppBar;
  let open = false;

  let route: string | null;
  $: route = $page.route.id;
</script>

<div>
  <Drawer variant="dismissible" bind:open>
    <Header>
      <DrawerTitle>Drawer</DrawerTitle>
    </Header>
    <Content>
      <List>
        <Item href="/user" activated={route === '/user'}>
          <Text>Users</Text>
        </Item>
        <Item href="/client" activated={route === '/client'}>
          <Text>Clients</Text>
        </Item>
        <Item>
          <Text>Item 3</Text>
        </Item>
      </List>
    </Content>
  </Drawer>

  <AppContent>
    <TopAppBar variant="static" bind:this={topAppBar}>
      <Row>
        <Section>
          <IconButton class="material-icons" on:click={() => (open = !open)}
            >menu
          </IconButton>
          <Title>Certificate authority</Title>
        </Section>
        <Section align="end" toolbar>
          <IconButton class="material-icons" aria-label="Download"
            >file_download
          </IconButton>
          <IconButton class="material-icons" aria-label="Print this page"
            >print
          </IconButton>
          <IconButton class="material-icons" aria-label="Bookmark this page"
            >bookmark
          </IconButton>
        </Section>
      </Row>
    </TopAppBar>
    <main>
      <slot />
    </main>
  </AppContent>
</div>

<Toaster toastOptions={{ style: 'font-family: Roboto' }} />
