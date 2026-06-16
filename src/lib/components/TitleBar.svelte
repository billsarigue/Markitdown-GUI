<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

  async function minimize() { await appWindow.minimize(); }
  async function toggleMaximize() { await appWindow.toggleMaximize(); }
  async function close() { await appWindow.close(); }

  let lastClickTime = 0;
  const DBLCLICK_MS = 300;

  async function onMousedown(e: MouseEvent) {
    if ((e.target as HTMLElement).closest('button')) return;

    const now = Date.now();
    const gap = now - lastClickTime;
    lastClickTime = now;

    if (gap < DBLCLICK_MS) {
      // Duplo clique detectado manualmente
      lastClickTime = 0;
      await toggleMaximize();
      return;
    }

    await appWindow.startDragging();
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="titlebar" on:mousedown={onMousedown}>
  <span class="app-name">Markitdown GUI</span>
  <div class="titlebar-controls">
    <button class="dot minimize" on:click|stopPropagation={minimize} title="Minimizar"></button>
    <button class="dot maximize" on:click|stopPropagation={toggleMaximize} title="Maximizar"></button>
    <button class="dot close" on:click|stopPropagation={close} title="Fechar"></button>
  </div>
</div>

<style>
  .titlebar {
    height: 42px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1rem;
    border-radius: 14px 14px 0 0;
    background: rgba(2, 11, 30, 0.6);
    border-bottom: 1px solid rgba(59, 130, 246, 0.12);
    user-select: none;
    flex-shrink: 0;
    cursor: default;
  }

  .app-name {
    font-size: 0.82rem;
    font-weight: 600;
    color: #475569;
    letter-spacing: 0.04em;
    pointer-events: none;
  }

  .titlebar-controls {
    display: flex;
    gap: 0.45rem;
    align-items: center;
  }

  .dot {
    width: 13px;
    height: 13px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    transition: filter 0.15s, transform 0.1s;
    padding: 0;
    flex-shrink: 0;
  }

  .dot:hover { filter: brightness(1.25); transform: scale(1.12); }
  .dot:active { transform: scale(0.88); }

  .minimize { background: #febc2e; }
  .maximize { background: #28c840; }
  .close    { background: #ff5f57; }
</style>
