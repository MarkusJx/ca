<script lang="ts">
  import Button, { Label } from '@smui/button';
  import Dialog, { Title, Content } from '@smui/dialog';
  import Checkbox from '@smui/checkbox';
  import FormField from '@smui/form-field';
  import FormTextField from '$lib/components/FormTextField.svelte';
  import { form, field } from 'svelte-forms';
  import { required, between, max } from 'svelte-forms/validators';
  import toast from 'svelte-french-toast';
  import { createUser } from '$lib/api/users/users';

  export let open = false;
  export let onClose: (userCreated: boolean) => void;

  let loading = false;
  let passwordTemporary = false;

  const username = field('username', '', [required(), between(3, 20)]);
  const password = field('password', '', [required(), between(3, 20)]);
  const emailAddress = field('email', '', [max(50)]);
  const firstName = field('firstName', '', [max(20)]);
  const lastName = field('lastName', '', [max(20)]);
  const createForm = form(
    username,
    password,
    emailAddress,
    firstName,
    lastName
  );

  const onOk = async () => {
    await createForm.validate();
    if (!$createForm.valid) {
      toast.error('Please fix the errors in the form');
      return;
    }

    try {
      loading = true;
      await toast.promise(
        createUser({
          name: $username.value,
          email: $emailAddress.value || null,
          firstName: $firstName.value || null,
          lastName: $lastName.value || null,
          password: $password.value,
          roles: null,
          isPasswordTemporary: passwordTemporary,
        }),
        {
          success: 'User created',
          error: 'Failed to create user',
          loading: 'Creating user',
        }
      );

      onClose(true);
    } finally {
      loading = false;
    }
  };

  const onCancel = () => {
    createForm.reset();
    onClose(false);
  };
</script>

<Dialog
  bind:open
  aria-labelledby="create-user-title"
  aria-describedby="create-user-content"
>
  <Title id="create-user-title">Create new user</Title>
  <Content id="create-user-content">
    <div class="text-field-container">
      <FormTextField
        label="Username"
        errors={$createForm.errors}
        bind:value={$username.value}
        errorText="Username must be between 3 and 20 characters long"
        required
        disabled={loading}
      />
      <FormTextField
        label="Password"
        errors={$createForm.errors}
        bind:value={$password.value}
        errorText="Password must be between 3 and 20 characters long"
        required
        disabled={loading}
        type="password"
      />
      <FormTextField
        label="Email"
        errors={$createForm.errors}
        bind:value={$emailAddress.value}
        errorText="Email address is invalid"
        disabled={loading}
      />
      <FormTextField
        label="First name"
        name="firstName"
        errors={$createForm.errors}
        bind:value={$firstName.value}
        errorText="Fist name must be between 3 and 20 characters long"
        disabled={loading}
      />
      <FormTextField
        label="Last name"
        name="lastName"
        errors={$createForm.errors}
        bind:value={$lastName.value}
        errorText="Last name must be between 3 and 20 characters long"
        disabled={loading}
      />
      <div>
        <FormField>
          <Checkbox bind:checked={passwordTemporary} />
          <span slot="label">Password temporary</span>
        </FormField>
      </div>
    </div>
  </Content>
  <div class="button-container">
    <Button on:click={onOk} disabled={!$createForm.valid || loading}>
      <Label>Ok</Label>
    </Button>
    <Button on:click={onCancel} disabled={loading}>
      <Label>Cancel</Label>
    </Button>
  </div>
</Dialog>

<style lang="scss">
  .text-field-container {
    margin-top: 10px;
    display: grid;
    grid-auto-rows: max-content;
    grid-row-gap: 10px;
  }

  .button-container {
    display: flex;
    justify-content: flex-end;
    margin: 10px;
  }
</style>
