<script lang="ts">
  export let active: 'convert' | 'settings' | 'about' = 'convert';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher<{ navigate: typeof active }>();

  function go(page: typeof active) {
    active = page;
    dispatch('navigate', page);
  }
</script>

<aside class="sidebar">
  <div class="logo">
    <div class="logo-icon">M</div>
  </div>

  <nav>
    <button class:active={active === 'convert'} on:click={() => go('convert')} title="Converter">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="9" y1="13" x2="15" y2="13"/>
        <line x1="9" y1="17" x2="12" y2="17"/>
      </svg>
      <span>Converter</span>
    </button>

    <button class:active={active === 'settings'} on:click={() => go('settings')} title="Configurações">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
      <span>Config.</span>
    </button>

    <button class:active={active === 'about'} on:click={() => go('about')} title="Sobre">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="8" x2="12" y2="12"/>
        <line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
      <span>Sobre</span>
    </button>
  </nav>
</aside>

<style>
  .sidebar {
    width: 76px;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0.75rem 0;
    background: rgba(2, 8, 24, 0.7);
    border-right: 1px solid rgba(59, 130, 246, 0.1);
    border-radius: 0 0 0 14px;
    gap: 0.25rem;
    flex-shrink: 0;
  }

  .logo {
    margin-bottom: 1rem;
  }

  .logo-icon {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background: linear-gradient(135deg, #1d4ed8, #3b82f6);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    font-size: 1rem;
    color: white;
    box-shadow: 0 0 12px rgba(59, 130, 246, 0.4);
  }

  nav {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
    align-items: center;
  }

  nav button {
    width: 56px;
    height: 56px;
    border-radius: 12px;
    border: none;
    background: transparent;
    color: #475569;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.2rem;
    transition: background 0.15s, color 0.15s;
    padding: 0;
  }

  nav button svg {
    width: 20px;
    height: 20px;
  }

  nav button span {
    font-size: 0.6rem;
    letter-spacing: 0.02em;
  }

  nav button:hover {
    background: rgba(59, 130, 246, 0.1);
    color: #93c5fd;
  }

  nav button.active {
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    box-shadow: inset 0 0 0 1px rgba(59, 130, 246, 0.25);
  }
</style>
