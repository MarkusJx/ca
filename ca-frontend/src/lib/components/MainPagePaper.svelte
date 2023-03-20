<script lang="ts">
  import Paper, { Title, Content } from '@smui/paper';
  import Ripple from '@smui/ripple';

  export let href: string | undefined = undefined;
  export let title: string;

  const onClick = () => {
    if (href) {
      location.href = href;
    }
  };
</script>

<Paper style="padding: 0" on:click={onClick}>
  <div use:Ripple={{ surface: true }} class="paper-ripple-container">
    <Title style="text-align: center">{title}</Title>
    <Content style="text-align: center">
      <slot />
    </Content>
  </div>
</Paper>

<style lang="scss">
  .paper-ripple-container {
    padding: 24px 16px;
    -webkit-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
</style>
