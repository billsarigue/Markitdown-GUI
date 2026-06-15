<script lang="ts">
  export let markdown = '';
  export let isLoading = false;
  export let error = '';
</script>

<section class="preview">
  <div class="header">
    <h2>Markdown</h2>
  </div>

  {#if isLoading}
    <div class="state muted">Convertendo arquivo...</div>
  {:else if error}
    <div class="state error">{error}</div>
  {:else if markdown}
    <textarea readonly value={markdown}></textarea>
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
    margin-bottom: 0.75rem;
  }

  .header h2 {
    margin: 0;
    color: #e2e8f0;
    font-size: 1.2rem;
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
</style>
