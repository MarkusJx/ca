<script lang="ts">
  import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
  import IconButton from '@smui/icon-button';
  import Drawer, {
    AppContent,
    Content,
    Header,
    Title as DrawerTitle,
  } from '@smui/drawer';
  import List, { Item, Text, Separator } from '@smui/list';
  import Tooltip, { Wrapper } from '@smui/tooltip';
  import Menu from '@smui/menu';
  import { Toaster } from 'svelte-french-toast';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import type LayoutData from '../types/LayoutData';
  import { KeycloakAdapter } from '$lib/keycloak.js';

  export let data: LayoutData;

  onMount(() => {
    if (!data.keycloak) {
      window.location.reload();
    }
  });

  let topAppBar: TopAppBar;
  let open = false;
  let menu: Menu & { setOpen: (open: boolean) => void };

  let route: string | null;
  $: route = $page.route.id;
</script>

<div>
  <Drawer variant="dismissible" bind:open>
    <Header>
      <DrawerTitle>CA</DrawerTitle>
    </Header>
    <Content>
      <List>
        <Item href="/" activated={route === '/'}>
          <Text>Home</Text>
        </Item>
        <Separator />
        <Item href="/user" activated={route === '/user'}>
          <Text>Users</Text>
        </Item>
        <Item href="/client" activated={route === '/client'}>
          <Text>Clients</Text>
        </Item>
      </List>
    </Content>
  </Drawer>

  <AppContent>
    <TopAppBar variant="static" bind:this={topAppBar}>
      <Row>
        <Section>
          <IconButton class="material-icons" on:click={() => (open = !open)}>
            menu
          </IconButton>
          <Title>Certificate authority</Title>
        </Section>
        <Section align="end" toolbar>
          <div>
            <Wrapper>
              <IconButton
                class="material-icons"
                aria-label="User actions"
                on:click={() => menu.setOpen(true)}
              >
                person
              </IconButton>
              <Tooltip>User actions</Tooltip>
            </Wrapper>
            <Menu bind:this={menu}>
              <List>
                <Item on:SMUI:action={() => KeycloakAdapter.logout()}>
                  <Text>Logout</Text>
                </Item>
              </List>
            </Menu>
          </div>
        </Section>
      </Row>
    </TopAppBar>
    <main>
      <slot />
    </main>
  </AppContent>
</div>

<Toaster toastOptions={{ style: 'font-family: Roboto' }} />
