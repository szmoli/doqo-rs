<script lang="ts">
  import { createHighlighter, type Highlighter } from 'shiki';

  let { code, language, languages } = $props<{ 
    code: string; 
    language: string 
    languages: string[]
  }>();

  let highlighter = $state<Highlighter | null>(null);
  let highlightedHtml = $state<string>('');
  let isLoading = $state(true);

  async function initHighlighter() {
    highlighter = await createHighlighter({
      themes: ['github-dark'],
      langs: languages
    });
  }

  $effect(() => {
    if (!code) return;

    const highlight = async () => {
      if (!highlighter) await initHighlighter();
      if (!highlighter) return;

      isLoading = true;
      const lang = language.toLowerCase() === 'rs' ? 'rust' : language.toLowerCase();

      try {
        await highlighter.loadLanguage(lang as any);
        
        highlightedHtml = highlighter.codeToHtml(code.trim(), {
          lang: lang,
          theme: 'github-dark'
        });
      } catch (e) {
        console.warn(`Shiki error loading language: ${lang}`, e);
        highlightedHtml = `<pre class="shiki"><code>${code}</code></pre>`;
      } finally {
        isLoading = false;
      }
    };

    highlight();
  });
</script>

<div class="relative group rounded-lg border border-slate-800 bg-[#0d1117] font-mono text-sm overflow-hidden">
  <div class="absolute right-3 top-2 text-[10px] font-bold uppercase text-slate-500 select-none z-10">
    {language}
  </div>

  {#if isLoading && !highlightedHtml}
    <div class="p-4 text-slate-500 animate-pulse">Loading highlighter...</div>
  {/if}

  <div class="shiki-container">
    {@html highlightedHtml}
  </div>
</div>

<style>
  :global(.shiki-container pre) {
    margin: 0 !important;
    padding: 1rem !important;
    background-color: transparent !important;
    overflow-x: auto;
  }

  :global(.shiki-container code) {
    background: none !important;
    padding: 0 !important;
  }

  :global(.shiki-container pre::-webkit-scrollbar) {
    height: 8px;
  }
  :global(.shiki-container pre::-webkit-scrollbar-thumb) {
    background: #30363d;
    border-radius: 4px;
  }
</style>