<script lang="ts">
  import { marked } from 'marked';
  import type { DoqoSymbol } from '$lib/bindings/DoqoSymbol';

  let { symbol } = $props<{ symbol: DoqoSymbol }>();

  const rawMarkdown = $derived(
    symbol.documentation.comments
      .join('\n')
  );

  let renderedHtml = $state('');
  let isParsing = $state(false);

  $effect(() => {
    if (!rawMarkdown) {
      renderedHtml = '';
      return;
    }

    const parseContent = async () => {
      isParsing = true;
      try {
        const html = await marked.parse(rawMarkdown);
        renderedHtml = html;
      } catch (e) {
        console.error("Markdown parse error:", e);
        renderedHtml = '<p class="text-red-500">Error parsing documentation.</p>';
      } finally {
        isParsing = false;
      }
    };

    parseContent();
  });
</script>

<div class="prose prose-slate max-w-none">
  {@html renderedHtml}
</div>