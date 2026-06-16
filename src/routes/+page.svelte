<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import DropZone from '$lib/components/DropZone.svelte';
  import MarkdownPreview from '$lib/components/MarkdownPreview.svelte';

  let selectedFiles: string[] = [];
  let markdown = '';
  let isLoading = false;
  let error = '';

  type ConvertResult = {
    success: boolean;
    content?: string;
    output_path?: string;
    error?: string;
  };

  async function handleFilesSelected(event: CustomEvent<{ paths: string[] }>) {
    selectedFiles = event.detail.paths;
    markdown = '';
    error = '';

    if (selectedFiles.length === 0) return;

    isLoading = true;

    try {
      const result = await invoke<ConvertResult>('convert_file', {
        options: {
          input_path: selectedFiles[0],
          enable_plugins: false,
        },
      });

      if (!result.success) {
        error = result.error ?? 'Falha desconhecida na conversão.';
        return;
      }

      markdown = result.content ?? '';
    } catch (e) {
      error = `Erro ao converter arquivo: ${String(e)}`;
    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>Markitdown GUI</title>
  <meta name="description" content="GUI para o Markitdown da Microsoft" />
</svelte:head>

<div class="app-shell">
  <header>
    <h1>Markitdown GUI</h1>
    <p>Converta arquivos para Markdown com uma interface gráfica simples.</p>
  </header>

  <div class="grid">
    <section>
      <DropZone on:filesSelected={handleFilesSelected} />

      {#if selectedFiles.length > 0}
        <div class="file-list">
          <h2>Arquivos selecionados</h2>
          <ul>
            {#each selectedFiles as file}
              <li>{file}</li>
            {/each}
          </ul>
        </div>
      {/if}
    </section>

    <MarkdownPreview {markdown} {isLoading} {error} />
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: Inter, system-ui, sans-serif;
    background: #020617;
    color: #f8fafc;
  }

  .app-shell {
    max-width: 1280px;
    margin: 0 auto;
    padding: 2rem;
  }

  header {
    margin-bottom: 1.5rem;
  }

  header h1 {
    margin: 0 0 0.25rem;
    font-size: 2rem;
  }

  header p {
    margin: 0;
    color: #94a3b8;
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.25rem;
  }

  .file-list {
    margin-top: 1rem;
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 16px;
    padding: 1rem;
  }

  .file-list h2 {
    margin: 0 0 0.75rem;
    font-size: 1rem;
  }

  .file-list ul {
    margin: 0;
    padding-left: 1rem;
    color: #cbd5e1;
    max-height: 180px;
    overflow: auto;
  }

  .file-list li {
    margin-bottom: 0.4rem;
    word-break: break-all;
  }

  @media (max-width: 900px) {
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
