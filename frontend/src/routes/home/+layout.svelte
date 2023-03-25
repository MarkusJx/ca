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
	import { page } from '$app/stores';
	import { KeycloakAdapter } from '$lib/keycloak.js';
	import { loadStores, saveStore, sideDrawerOpen } from '$lib/stores.js';
	import { onMount } from 'svelte';

	let menu: Menu & { setOpen: (open: boolean) => void };

	onMount(() => {
		loadStores();
	});

	const logout = () => {
		KeycloakAdapter.logout();
	};

	const elements = [
		{ id: '/home', name: 'Home', separator: true },
		{ id: '/home/user', name: 'Users' },
		{ id: '/home/client', name: 'Clients' },
	];

	let route: string | null;
	$: route = $page.route.id;
	$: saveStore('sideDrawerOpen', $sideDrawerOpen);
</script>

<div>
	<Drawer variant="dismissible" bind:open={$sideDrawerOpen}>
		<Header>
			<DrawerTitle>CA</DrawerTitle>
		</Header>
		<Content>
			<List>
				{#each elements as element (element.id)}
					<Item
						href={element.id}
						activated={route === element.id}
						on:SMUI:action={() => (route = element.id)}
					>
						<Text>{element.name}</Text>
					</Item>
					{#if element.separator}
						<Separator />
					{/if}
				{/each}
			</List>
		</Content>
	</Drawer>

	<AppContent>
		<TopAppBar variant="static">
			<Row>
				<Section>
					<Wrapper>
						<IconButton
							class="material-icons"
							on:click={() => ($sideDrawerOpen = !$sideDrawerOpen)}
						>
							menu
						</IconButton>
						<Tooltip>Menu</Tooltip>
					</Wrapper>
					<Title>Certificate authority</Title>
				</Section>
				<Section align="end" toolbar>
					{#if route !== '/home'}
						<Wrapper>
							<IconButton
								class="material-icons"
								on:click={() => (location.href = '/home')}
							>
								home
							</IconButton>
							<Tooltip>Home</Tooltip>
						</Wrapper>
					{/if}
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
								<Item on:SMUI:action={logout}>
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
