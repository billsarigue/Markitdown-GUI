<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let open = false;
  export let llmApiKey = '';
  export let llmEndpoint = '';
  export let llmModel = 'gpt-4o';

  const dispatch = createEventDispatcher<{ apply: { llmApiKey: string; llmEndpoint: string; llmModel: string } }>();

  function apply() {
    dispatch('apply', { llmApiKey, llmEndpoint, llmModel });
    open = false;
  }

  function clear() {
    llmApiKey = '';
    llmEndpoint = '';
    llmModel = 'gpt-4o';
    dispatch('apply', { llmApiKey: '', llmEndpoint: '', llmModel: 'gpt-4o' });
    open = false;
  }
</script>

{#if open}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="overlay" on:click|self={() => (open = false)}>
    <div class="panel">
      <h2>Configurações de LLM</h2>
      <p class="hint">
        Necessário para converter <strong>imagens</strong> e <strong>áudios</strong>.
        Compatible com OpenAI e qualquer endpoint Azure OpenAI.
      </p>

      <label>
        Modelo
        <input type="text" bind:value={llmModel} placeholder="gpt-4o" />
      </label>

      <label>
        API Key
        <input type="password" bind:value={llmApiKey} placeholder="sk-..." />
      </label>

      <label>
        Endpoint (opcional — Azure)
        <input type="text" bind:value={llmEndpoint} placeholder="https://...openai.azure.com/" />
      </label>

      <div class="actions">
        <button class="btn-secondary" on:click={clear}>Limpar</button>
        <button class="btn-primary" on:click={apply}>Aplicar</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
  }

  .panel {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 16px;
    padding: 2rem;
    width: 100%;
    max-width: 480px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  h2 {
    margin: 0;
    font-size: 1.2rem;
    color: #e2e8f0;
  }

  .hint {
    margin: 0;
    color: #94a3b8;
    font-size: 0.9rem;
    line-height: 1.5;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    font-size: 0.9rem;
    color: #cbd5e1;
  }

  input {
    background: #020617;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 0.5rem 0.75rem;
    color: #f1f5f9;
    font-size: 0.95rem;
    outline: none;
    transition: border-color 0.15s;
  }

  input:focus {
    border-color: #3b82f6;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .btn-primary {
    background: #1d4ed8;
    color: #f8fafc;
    border: none;
    border-radius: 8px;
    padding: 0.5rem 1.25rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-primary:hover {
    background: #2563eb;
  }

  .btn-secondary {
    background: transparent;
    color: #94a3b8;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 0.5rem 1.25rem;
    font-size: 0.9rem;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
  }

  .btn-secondary:hover {
    color: #f1f5f9;
    border-color: #64748b;
  }
</style>
