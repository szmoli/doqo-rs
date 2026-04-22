<script lang="ts">
  import Fuse from 'fuse.js';
  import type { DoqoSymbolTable } from '$lib/bindings/DoqoSymbolTable';
  import { fqidToPath, symbolName } from '$lib/utils';
	import type { DoqoSymbol } from '$lib/bindings/DoqoSymbol';
	import SymbolCard from './SymbolCard.svelte';

  let { symbolTable } = $props<{ symbolTable: DoqoSymbolTable }>();

  let query = $state('');
  let isOpen = $state(false);

  const searchList = $derived(
    Object.values(symbolTable.symbols as DoqoSymbol[]).map(s => ({
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
        {#each results as { item }}
        {@const symbol = item.symbol}
          <!--a
            href={item.path}
            onclick={() => { query = ''; isOpen = false; }}
            class="flex items-center justify-between rounded-lg px-3 py-2 hover:bg-blue-50 group transition-colors"
          >
            <div class="flex flex-col min-w-0">
              <span class="text-sm font-mono font-bold text-slate-800 group-hover:text-blue-700 truncate">
                {item.name}
              </span>
              <span class="text-[10px] text-slate-400 truncate font-mono">
                {item.fqid}
              </span>
            </div>
            <span class="ml-2 rounded bg-slate-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-slate-500 border border-slate-200">
              {item.kind}
            </span>
          </a-->
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