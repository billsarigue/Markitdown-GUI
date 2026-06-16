<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  const dispatch = createEventDispatcher<{ filesSelected: { paths: string[] } }>();

  let isDragging = false;
  let error = '';

  const allowedExtensions = [
    'pdf', 'docx', 'doc', 'pptx', 'ppt', 'xlsx', 'xls',
    'html', 'htm', 'txt', 'csv', 'json', 'xml',
    'jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp',
    'zip', 'epub'
  ];

  function isSupported(path: string) {
    const ext = path.split('.').pop()?.toLowerCase();
    return !!ext && allowedExtensions.includes(ext);
  }

  function validatePaths(paths: string[]) {
    const valid = paths.filter(isSupported);
    const invalid = paths.filter((p) => !isSupported(p));
    error = invalid.length > 0
      ? `Tipo n\u00e3o suportado: ${invalid.map(p => p.split(/[\\/]/).pop()).join(', ')}`
      : '';
    if (valid.length > 0) dispatch('filesSelected', { paths: valid });
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer) e.dataTransfer.dropEffect = 'copy';
    isDragging = true;
  }

  function onDragLeave(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;
  }

  function onDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    isDragging = false;
    if (!e.dataTransfer) return;
    const paths = Array.from(e.dataTransfer.files).map((f) => (f as any).path ?? f.name);
    if (paths.length > 0) validatePaths(paths);
  }

  async function pickFiles() {
    try {
      const selected = await open({
        multiple: true,
        filters: [{ name: 'Supported files', extensions: allowedExtensions }],
      });
      if (!selected) return;
      validatePaths(Array.isArray(selected) ? selected : [selected]);
    } catch (e) {
      error = `Erro ao abrir seletor: ${String(e)}`;
    }
  }
</script>

<div
  class="dropzone"
  class:is-dragging={isDragging}
  role="button"
  tabindex="0"
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
  on:click={pickFiles}
  on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && pickFiles()}
>
  <div class="content">
    <div class="icon">📄</div>
    <h2>Arraste arquivos aqui</h2>
    <p>ou clique para selecionar</p>
    <small>PDF, Word, PowerPoint, Excel, HTML, TXT, CSV, JSON, XML, imagens, ZIP, EPUB</small>
  </div>
</div>

{#if error}
  <p class="drop-error">{error}</p>
{/if}

<style>
  .dropzone {
    border: 2px dashed rgba(99, 130, 180, 0.35);
    border-radius: 16px;
    padding: 3rem 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    cursor: pointer;
    background: rgba(255, 255, 255, 0.02);
    color: #f3f4f6;
    transition: all 0.2s ease;
    min-height: 240px;
  }

  .dropzone:hover {
    border-color: rgba(96, 165, 250, 0.6);
    background: rgba(59, 130, 246, 0.05);
  }

  .dropzone.is-dragging {
    border-color: #22c55e;
    background: rgba(34, 197, 94, 0.06);
    transform: scale(1.01);
  }

  .content { display: flex; flex-direction: column; align-items: center; gap: 0.4rem; }
  .icon { font-size: 2.5rem; margin-bottom: 0.25rem; }
  .content h2 { margin: 0; font-size: 1.3rem; font-weight: 600; }
  .content p { margin: 0; color: #94a3b8; font-size: 0.95rem; }
  .content small { color: #64748b; line-height: 1.5; font-size: 0.82rem; }

  .drop-error {
    margin-top: 0.5rem;
    color: #fca5a5;
    font-size: 0.88rem;
  }
</style>
