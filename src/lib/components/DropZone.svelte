<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  const dispatch = createEventDispatcher<{
    filesSelected: { paths: string[] };
  }>();

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

  function normalizeDroppedFiles(fileList: FileList | null): string[] {
    if (!fileList) return [];

    return Array.from(fileList)
      .map((file) => (file as File & { path?: string }).path || file.name)
      .filter(Boolean);
  }

  function validatePaths(paths: string[]) {
    const valid = paths.filter(isSupported);
    const invalid = paths.filter((p) => !isSupported(p));

    if (invalid.length > 0) {
      error = `Alguns arquivos foram ignorados por tipo não suportado: ${invalid.join(', ')}`;
    } else {
      error = '';
    }

    if (valid.length > 0) {
      dispatch('filesSelected', { paths: valid });
    }
  }

  function onDragEnter(event: DragEvent) {
    event.preventDefault();
    isDragging = true;
  }

  function onDragOver(event: DragEvent) {
    event.preventDefault();
    isDragging = true;
  }

  function onDragLeave(event: DragEvent) {
    event.preventDefault();
    isDragging = false;
  }

  function onDrop(event: DragEvent) {
    event.preventDefault();
    isDragging = false;

    const paths = normalizeDroppedFiles(event.dataTransfer?.files ?? null);
    validatePaths(paths);
  }

  async function pickFiles() {
    try {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: 'Supported files',
            extensions: allowedExtensions,
          },
        ],
      });

      if (!selected) return;

      const paths = Array.isArray(selected) ? selected : [selected];
      validatePaths(paths);
    } catch (e) {
      error = `Erro ao abrir seletor de arquivos: ${String(e)}`;
    }
  }
</script>

<div
  class:is-dragging={isDragging}
  class="dropzone"
  on:dragenter={onDragEnter}
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
  role="button"
  tabindex="0"
  on:click={pickFiles}
  on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && pickFiles()}
>
  <div class="content">
    <h2>Arraste arquivos aqui</h2>
    <p>ou clique para selecionar</p>
    <small>
      Suporta PDF, Word, PowerPoint, Excel, HTML, TXT, CSV, JSON, XML, imagens, ZIP e EPUB.
    </small>
  </div>
</div>

{#if error}
  <p class="error">{error}</p>
{/if}

<style>
  .dropzone {
    border: 2px dashed #7c8aa5;
    border-radius: 16px;
    padding: 3rem 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    cursor: pointer;
    background: #111827;
    color: #f3f4f6;
    transition: all 0.2s ease;
    min-height: 240px;
  }

  .dropzone:hover {
    border-color: #60a5fa;
    background: #172033;
  }

  .dropzone.is-dragging {
    border-color: #22c55e;
    background: #13261a;
    transform: scale(1.01);
  }

  .content h2 {
    margin: 0 0 0.5rem;
    font-size: 1.5rem;
  }

  .content p {
    margin: 0 0 0.75rem;
    color: #cbd5e1;
  }

  .content small {
    color: #94a3b8;
    line-height: 1.4;
  }

  .error {
    margin-top: 0.75rem;
    color: #fca5a5;
    font-size: 0.95rem;
  }
</style>
