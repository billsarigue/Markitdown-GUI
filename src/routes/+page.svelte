<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';
  import TitleBar from '$lib/components/TitleBar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import DropZone from '$lib/components/DropZone.svelte';
  import MarkdownPreview from '$lib/components/MarkdownPreview.svelte';
  import LlmSettings from '$lib/components/LlmSettings.svelte';

  let activePage: 'convert' | 'settings' | 'about' = 'convert';
  let selectedFiles: string[] = [];
  let markdown = '';
  let isLoading = false;
  let error = '';
  let isSaving = false;
  let savedPath = '';

  let showLlmModal = false;
  let llmApiKey = '';
  let llmEndpoint = '';
  let llmModel = 'gpt-4o';
  $: hasLlm = llmApiKey.trim().length > 0;

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
    savedPath = '';
    if (selectedFiles.length === 0) return;
    isLoading = true;
    try {
      const options: Record<string, unknown> = { input_path: selectedFiles[0] };
      if (hasLlm) {
        options.llm_api_key = llmApiKey;
        options.llm_model = llmModel;
        if (llmEndpoint.trim()) options.llm_endpoint = llmEndpoint.trim();
      }
      const result = await invoke<ConvertResult>('convert_file', { options });
      if (!result.success) { error = result.error ?? 'Falha desconhecida.'; return; }
      markdown = result.content ?? '';
    } catch (e) {
      error = `Erro ao converter arquivo: ${String(e)}`;
    } finally {
      isLoading = false;
    }
  }

  async function handleSave() {
    if (!markdown) return;
    isSaving = true;
    savedPath = '';
    try {
      const originalName = selectedFiles[0]
        ? selectedFiles[0].split(/[\\/]/).pop()?.replace(/\.[^.]+$/, '') ?? 'documento'
        : 'documento';
      const filePath = await save({
        defaultPath: `${originalName}.md`,
        filters: [{ name: 'Markdown', extensions: ['md'] }],
      });
      if (!filePath) return;
      await writeTextFile(filePath, markdown);
      savedPath = filePath;
    } catch (e) {
      error = `Erro ao salvar: ${String(e)}`;
    } finally {
      isSaving = false;
    }
  }

  function handleLlmApply(event: CustomEvent<{ llmApiKey: string; llmEndpoint: string; llmModel: string }>) {
    llmApiKey = event.detail.llmApiKey;
    llmEndpoint = event.detail.llmEndpoint;
    llmModel = event.detail.llmModel;
  }

  function handleNavigate(event: CustomEvent<'convert' | 'settings' | 'about'>) {
    if (event.detail === 'settings') {
      showLlmModal = true;
      activePage = 'convert';
    } else {
      activePage = event.detail;
    }
  }
</script>

<svelte:head>
  <title>Markitdown GUI</title>
</svelte:head>

<LlmSettings
  bind:open={showLlmModal}
  bind:llmApiKey
  bind:llmEndpoint
  bind:llmModel
  on:apply={handleLlmApply}
/>

<div class="window">
  <TitleBar />

  <div class="window-body">
    <Sidebar bind:active={activePage} on:navigate={handleNavigate} />

    <main class="main">
      {#if activePage === 'convert'}
        <div class="convert-layout">
          <div class="left-col">
            <DropZone on:filesSelected={handleFilesSelected} />
            {#if selectedFiles.length > 0}
              <div class="file-list glass">
                <h3>Arquivos selecionados</h3>
                <ul>
                  {#each selectedFiles as file}
                    <li>{file}</li>
                  {/each}
                </ul>
              </div>
            {/if}
          </div>
          <div class="right-col">
            <MarkdownPreview
              {markdown} {isLoading} {error} {isSaving} {savedPath} {hasLlm}
              currentFile={selectedFiles[0] ?? ''}
              on:save={handleSave}
              on:openSettings={() => (showLlmModal = true)}
            />
          </div>
        </div>

      {:else if activePage === 'about'}
        <div class="about glass">
          <div class="about-logo">M</div>
          <h2>Markitdown GUI</h2>
          <p>Interface gráfica para o <a href="https://github.com/microsoft/markitdown" target="_blank">MarkItDown</a> da Microsoft.</p>
          <p class="built">Zionlux</p>
          <div class="badges">
            <span>Tauri 2</span>
            <span>SvelteKit</span>
            <span>TypeScript</span>
            <span>Python</span>
          </div>
        </div>
      {/if}
    </main>
  </div>
</div>

<style>
  :global(*) { box-sizing: border-box; }

  :global(body) {
    margin: 0;
    font-family: Inter, system-ui, sans-serif;
    background: transparent;
    color: #f1f5f9;
    overflow: hidden;
  }

  .window {
    width: 100vw;
    height: 100vh;
    border-radius: 14px;
    background:
      radial-gradient(ellipse at 20% 0%, rgba(29, 78, 216, 0.18) 0%, transparent 60%),
      radial-gradient(ellipse at 80% 100%, rgba(59, 130, 246, 0.12) 0%, transparent 55%),
      linear-gradient(160deg, #020b1e 0%, #030d24 50%, #020918 100%);
    border: 1px solid rgba(59, 130, 246, 0.2);
    box-shadow:
      0 0 0 1px rgba(59, 130, 246, 0.08),
      0 8px 60px rgba(0, 0, 0, 0.8),
      0 0 80px rgba(29, 78, 216, 0.12);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .window-body {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .main {
    flex: 1;
    padding: 1.25rem;
    overflow: auto;
    display: flex;
    flex-direction: column;
  }

  .convert-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    flex: 1;
  }

  .left-col, .right-col {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-height: 0;
  }

  .glass {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.07);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-radius: 14px;
  }

  .file-list {
    padding: 1rem;
  }

  .file-list h3 {
    margin: 0 0 0.6rem;
    font-size: 0.85rem;
    color: #64748b;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .file-list ul {
    margin: 0;
    padding-left: 1rem;
    color: #94a3b8;
    max-height: 140px;
    overflow: auto;
  }

  .file-list li {
    margin-bottom: 0.35rem;
    font-size: 0.88rem;
    word-break: break-all;
  }

  /* About */
  .about {
    max-width: 480px;
    margin: 2rem auto;
    padding: 2.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
  }

  .about-logo {
    width: 64px;
    height: 64px;
    border-radius: 18px;
    background: linear-gradient(135deg, #1d4ed8, #3b82f6);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    font-size: 1.8rem;
    color: white;
    box-shadow: 0 0 24px rgba(59, 130, 246, 0.4);
    margin-bottom: 0.5rem;
  }

  .about h2 { margin: 0; font-size: 1.4rem; }
  .about p { margin: 0; color: #94a3b8; font-size: 0.95rem; }
  .about a { color: #60a5fa; text-decoration: none; }
  .about a:hover { text-decoration: underline; }
  .built { font-size: 0.85rem !important; color: #475569 !important; }

  .badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    justify-content: center;
    margin-top: 0.5rem;
  }

  .badges span {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 20px;
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
    color: #93c5fd;
  }

  @media (max-width: 860px) {
    .convert-layout { grid-template-columns: 1fr; }
  }
</style>
