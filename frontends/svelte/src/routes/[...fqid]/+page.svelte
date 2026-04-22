<script lang="ts">
	import { page } from '$app/state';
	import symbolTableJson from '$lib/json/symbol_table.json';
	import type { DoqoSymbolTable } from '$lib/bindings/DoqoSymbolTable';
	import { pathToFqid, symbolName, source } from '$lib/utils';
	import SymbolCard from '$lib/components/SymbolCard.svelte';
	import SymbolBreadcrumbs from '$lib/components/SymbolBreadcrumbs.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	const symbolTable = symbolTableJson as DoqoSymbolTable;
  const languages = $derived(
    [...new Set(Object.values(symbolTable.sources).map(s => s.language))]
  );

	const path = $derived(page.params.fqid ?? '');
	const fqid = $derived(pathToFqid(path));

	const symbol = $derived(Object.values(symbolTable.symbols).find((s) => s.fqid === fqid));
  const language = $derived(symbol ? symbol.language : "");
  const code = $derived(symbol ? source(symbol, symbolTable) : "");

	const childrenList = $derived(
		symbol?.children.map((id) => symbolTable.symbols[id]).filter(Boolean) ?? []
	);
	const groupedChildren = $derived(Object.groupBy(childrenList, (c) => c.kind));
	const childKinds = $derived(Object.keys(groupedChildren).sort());
</script>

{#if symbol}
	<div class="p-8">
    <header>
      <SymbolBreadcrumbs {symbol} />

      <h1 class="mb-4 font-mono text-4xl font-bold">{symbolName(symbol)}</h1>

      <div class="badge mb-6 inline-block rounded bg-purple-100 px-2 py-1 text-purple-800">
        {symbol.kind}
      </div>
    </header>

		<div class="space-y-2 font-sans leading-relaxed text-gray-700">
			{symbol.documentation.comments}
		</div>

		<!--pre class="overflow-x-auto rounded bg-slate-900 p-4 text-white shadow-lg">
      <code>{source(symbol, symbolTable).trim()}</code>
    </pre-->

    <CodeBlock {code} {language} {languages}/>

    {#if childKinds.length > 0}
      <div>
        {#each childKinds as kind}
          <div class="mb-10">
            <h3
              class="mb-4 flex items-center gap-2 text-xs font-bold tracking-widest text-slate-400 uppercase"
            >
              {kind}
            </h3>

            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
              {#each groupedChildren[kind] as child}
                <SymbolCard symbol={child} />
              {/each}
            </div>
          </div>
        {/each}
      </div>
    {/if}
	</div>
{:else}
	<div class="p-20">
		<h1 class="text-2xl font-bold">Symbol not found: {fqid}</h1>
		<a href="/" class="text-blue-500 underline">Back to the front page</a>
	</div>
{/if}
