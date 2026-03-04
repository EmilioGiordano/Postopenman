<script lang="ts">
  import UrlBar from './UrlBar.svelte';
  import RequestTabs from './RequestTabs.svelte';
  import ResponsePanel from './ResponsePanel.svelte';
</script>

<div class="request-panel">
  <div class="request-top">
    <UrlBar />
  </div>
  <div class="request-middle">
    <RequestTabs />
  </div>
  <div class="request-bottom">
    <ResponsePanel />
  </div>
</div>

<style>
  .request-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .request-top {
    padding: 12px 16px;
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .request-middle {
    flex: 1;
    overflow-y: auto;
    min-height: 0;
    border-bottom: 1px solid var(--color-border);
  }

  .request-bottom {
    flex: 1;
    overflow: hidden;
    min-height: 120px;
  }
</style>
