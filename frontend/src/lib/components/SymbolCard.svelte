<script lang="ts">
  import type { DoqoSymbol } from "$lib/bindings/DoqoSymbol";
  import { fqidToPath } from "$lib/utils";
  import { resolve } from "$app/paths";

  let { symbol } = $props<{ 
    symbol: DoqoSymbol 
  }>();

  const name = $derived(symbol.fqid.split("::").pop());
  const path = $derived("/" + fqidToPath(symbol.fqid));
  const shortDoc = $derived(
    symbol.documentation.comments[0]?.replace(/^[\s]*\/\/\/?\!?[ ]?/, '') ?? ""
  );
  const colorClass = $derived("text-slate-600 bg-slate-50 border-slate-200");
</script>

<a 
  href={resolve(path as any)} 
  class="group flex flex-col p-4 rounded-lg border border-slate-200 bg-white shadow-sm 
         hover:border-blue-400 hover:shadow-md transition-all duration-200 h-full"
>
  <div class="flex items-start justify-between mb-2">
    <span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-tight border {colorClass}">
      {symbol.kind}
    </span>
    
    <span class="text-slate-300 group-hover:text-blue-500 transition-colors">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </span>
  </div>

  <div class="mb-3">
    <h4 class="text-sm font-mono font-bold text-slate-900 group-hover:text-blue-600 truncate">
      {name}
    </h4>
    <p class="text-[10px] font-mono text-slate-400 truncate tracking-tight" title={symbol.fqid}>
      {symbol.fqid}
    </p>
  </div>

  {#if shortDoc}
    <p class="text-xs text-slate-500 line-clamp-2 italic leading-relaxed">
      {shortDoc}
    </p>
  {:else}
    <p class="text-xs text-slate-300 italic">No documentation provided</p>
  {/if}
</a>