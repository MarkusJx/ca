<script>
  import CircularProgress from '@smui/circular-progress';
  import './Spinner.scss';
  import Center from './Center.svelte';
</script>

<Center>
  <CircularProgress
    class="multi-color-spinner"
    style="height: 60px; width: 60px;"
    indeterminate
    fourColor
  />
</Center>
