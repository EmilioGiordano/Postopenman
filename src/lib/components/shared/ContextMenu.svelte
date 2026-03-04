<script lang="ts">
  interface MenuItem {
    label: string;
    action: () => void;
  }

  let { x, y, items, onclose }: { x: number; y: number; items: MenuItem[]; onclose: () => void } = $props();

  function handleClick(action: () => void) {
    action();
    onclose();
  }

  function handleClickOutside(e: MouseEvent) {
    onclose();
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div
  class="context-menu"
  style="left: {x}px; top: {y}px;"
  onclick={(e) => e.stopPropagation()}
  onkeydown={(e) => { if (e.key === 'Escape') onclose(); }}
  role="menu"
  tabindex="-1"
>
  {#each items as item}
    <button
      class="context-item"
      onclick={() => handleClick(item.action)}
      role="menuitem"
    >
      {item.label}
    </button>
  {/each}
</div>

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    min-width: 140px;
    background: var(--color-bg-input);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 4px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  }

  .context-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: var(--color-text-primary);
    font-size: 12px;
    cursor: pointer;
  }

  .context-item:hover {
    background: var(--color-hover);
  }
</style>
