<script lang="ts">
  import Fuse from 'fuse.js';
  import type { DoqoRegistry } from '$lib/bindings/DoqoRegistry';
  import { symbolName } from '$lib/utils';
	import type { DoqoSymbol } from '$lib/bindings/DoqoSymbol';
	import SymbolCard from './SymbolCard.svelte';

  let { registry } = $props<{ registry: DoqoRegistry }>();

  let query = $state('');
  let isOpen = $state(false);

  const searchList = $derived(
    Object.values(registry.symbols as DoqoSymbol[]).map(s => ({
      name: symbolName(s),
      fqid: s.fqid,
      symbol: s,
    }))
  );

  const fuse = $derived(new Fuse(searchList, {
    keys: ['name', 'fqid'],
    threshold: 0.3,
  }));

  const results = $derived(query.length > 1 ? fuse.search(query) : []);

  function handleBlur() {
    setTimeout(() => { isOpen = false; }, 200);
  }
</script>

<div class="relative w-full max-w-md">
  <div class="relative">
    <input
      type="text"
      bind:value={query}
      onfocus={() => isOpen = true}
      onblur={handleBlur}
      placeholder="Search symbols..."
      class="w-full rounded-lg border border-slate-200 bg-slate-50 py-2 pl-10 pr-4 text-sm 
             focus:border-blue-400 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-100 
             transition-all shadow-sm"
    />
    <svg class="absolute left-3 top-2.5 h-4 w-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
    </svg>
  </div>

  {#if isOpen && results.length > 0}
    <div class="absolute z-50 mt-2 w-full overflow-hidden rounded-xl border border-slate-200 bg-white shadow-xl ring-1 ring-black ring-opacity-5">
      <div class="max-h-80 overflow-y-auto p-2">
        {#each results as { item } (item)}
        {@const symbol = item.symbol}
          <SymbolCard {symbol} />
        {/each}
      </div>
      
      <div class="bg-slate-50 px-3 py-2 text-[10px] text-slate-400 border-t border-slate-100">
        Found {results.length} matches for "{query}"
      </div>
    </div>
  {:else if isOpen && query.length > 1}
    <div class="absolute z-50 mt-2 w-full rounded-xl border border-slate-200 bg-white p-4 text-center text-sm text-slate-500 shadow-xl">
      No symbols found matching "{query}"
    </div>
  {/if}
</div>