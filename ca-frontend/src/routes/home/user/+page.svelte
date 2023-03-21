<script lang="ts">
	import UserTable from './UserTable.svelte';
	import Button from '@smui/button';
	import Switch from '@smui/switch';
	import FormField from '@smui/form-field';
	import { onMount } from 'svelte';
	import CreateUserDialog from './CreateUserDialog.svelte';
	import type { UserDto } from '$lib/api/models';
	import toast from 'svelte-french-toast';
	import { listUsers } from '$lib/api/users/users';
	import DeleteElementDialog from '$lib/components/DeleteElementDialog.svelte';
	import type ElementToDelete from '$lib/components/ElementToDelete';
	import { deleteUser } from '$lib/api/users/users';
	import { listRoles } from '$lib/api/admin/admin';

	let createDialogOpen = false;
	let deleteDialogUser: ElementToDelete | null = null;
	let userData: UserDto[] | null = null;
	let roles: string[] = [];
	let includeInactive: boolean = false;

	onMount(async () => {
		await Promise.all([
			loadData(),
			(async () => {
				roles = await listRoles();
			})(),
		]);
	});

	const loadData = async () => {
		userData = null;
		try {
			userData = await listUsers({
				includeInactive,
			});
		} catch (e) {
			toast.error('Failed to load data');
			userData = [];
		}
	};

	const onCreateDialogClose = (userCreated: boolean) => {
		createDialogOpen = false;
		if (userCreated) {
			loadData();
		}
	};

	const onDeleteDialogClose = (userDeleted: boolean) => {
		deleteDialogUser = null;
		if (userDeleted) {
			loadData();
		}
	};

	const handleSwitchIncludeInactive = () => {
		includeInactive = !includeInactive;
		loadData();
	};
</script>

<CreateUserDialog
	bind:open={createDialogOpen}
	onClose={onCreateDialogClose}
	{roles}
/>
<DeleteElementDialog
	onClose={onDeleteDialogClose}
	element={deleteDialogUser}
	name="user"
	deleteElement={deleteUser}
/>

<div class="container">
	<div class="include-inactive">
		<FormField>
			<Switch
				checked={includeInactive}
				disabled={!userData}
				on:click={handleSwitchIncludeInactive}
			/>
			<span slot="label">Include inactive</span>
		</FormField>
	</div>
	<div class="button-container">
		<div class="create-user-button-container">
			<Button on:click={() => (createDialogOpen = true)}>Create new user</Button
			>
		</div>
	</div>
</div>
<UserTable data={userData} deleteUser={(id) => (deleteDialogUser = id)} />

<style lang="scss">
	.container {
		display: grid;
		grid-template-columns: 50% 50%;
		grid-template-rows: 100%;

		.include-inactive {
			margin: auto 5px;
		}
	}

	.button-container {
		display: flex;
		justify-content: flex-end;

		.create-user-button-container {
			margin: 15px;
		}
	}
</style>
