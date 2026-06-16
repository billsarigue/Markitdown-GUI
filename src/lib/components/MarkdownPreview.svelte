<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let markdown = '';
  export let isLoading = false;
  export let error = '';
  export let isSaving = false;
  export let savedPath = '';

  const dispatch = createEventDispatcher<{ save: void }>();
</script>

<section class="preview">
  <div class="header">
    <h2>Markdown</h2>
    {#if markdown}
      <button
        class="save-btn"
        disabled={isSaving}
        on:click={() => dispatch('save')}
      >
        {#if isSaving}
          Salvando...
        {:else}
          💾 Salvar .md
        {/if}
      </button>
    {/if}
  </div>

  {#if isLoading}
    <div class="state muted">Convertendo arquivo...</div>
  {:else if error}
    <div class="state error">{error}</div>
  {:else if markdown}
    <textarea readonly value={markdown}></textarea>
    {#if savedPath}
      <p class="saved-notice">✅ Arquivo salvo em: {savedPath}</p>
    {/if}
  {:else}
    <div class="state muted">O conteúdo convertido aparecerá aqui.</div>
  {/if}
</section>

<style>
  .preview {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 16px;
    padding: 1rem;
    min-height: 420px;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .header h2 {
    margin: 0;
    color: #e2e8f0;
    font-size: 1.2rem;
  }

  .save-btn {
    background: #1d4ed8;
    color: #f8fafc;
    border: none;
    border-radius: 8px;
    padding: 0.4rem 1rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .save-btn:hover:not(:disabled) {
    background: #2563eb;
  }

  .save-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  textarea {
    width: 100%;
    min-height: 360px;
    resize: vertical;
    border: none;
    outline: none;
    background: #020617;
    color: #e5e7eb;
    border-radius: 12px;
    padding: 1rem;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    border-radius: 12px;
    padding: 1rem;
    min-height: 360px;
  }

  .muted {
    background: #111827;
    color: #94a3b8;
  }

  .error {
    background: #2a1111;
    color: #fecaca;
  }

  .saved-notice {
    margin-top: 0.75rem;
    color: #86efac;
    font-size: 0.9rem;
    word-break: break-all;
  }
</style>
