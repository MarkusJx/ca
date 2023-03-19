<script>
  import CircularProgress from '@smui/circular-progress';
  import './Spinner.scss';
</script>

<div class="progress-container">
  <CircularProgress
    class="multi-color-spinner"
    style="height: 60px; width: 60px;"
    indeterminate
    fourColor
  />
</div>

<style lang="scss">
  .progress-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    position: fixed;
  }
</style>
