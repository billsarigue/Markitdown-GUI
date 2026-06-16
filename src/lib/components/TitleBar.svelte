<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

  function minimize() { appWindow.minimize(); }
  function toggleMaximize() { appWindow.toggleMaximize(); }
  function close() { appWindow.close(); }
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-left" data-tauri-drag-region>
    <span class="app-name" data-tauri-drag-region>Markitdown GUI</span>
  </div>
  <div class="titlebar-controls">
    <button class="dot close" on:click={close} title="Fechar"></button>
    <button class="dot minimize" on:click={minimize} title="Minimizar"></button>
    <button class="dot maximize" on:click={toggleMaximize} title="Maximizar"></button>
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
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .app-name {
    font-size: 0.85rem;
    font-weight: 600;
    color: #64748b;
    letter-spacing: 0.03em;
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
  }

  .dot:hover { filter: brightness(1.3); transform: scale(1.1); }
  .dot:active { transform: scale(0.9); }

  .close    { background: #ff5f57; }
  .minimize { background: #febc2e; }
  .maximize { background: #28c840; }
</style>
