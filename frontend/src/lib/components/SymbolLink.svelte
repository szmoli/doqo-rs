<script lang="ts">
	import type { DoqoSymbol } from '$lib/bindings/DoqoSymbol';
	import { symbolName, fqidToPath } from '$lib/utils';
	import { page } from '$app/state';
  import { resolve } from '$app/paths';

	let { symbol } = $props<{
		symbol: DoqoSymbol;
	}>();

	const name = $derived(symbolName(symbol));
	const path = $derived('/' + fqidToPath(symbol.fqid));
	const isActive = $derived(page.url.pathname === path);
</script>

<a
	class="block rounded-r-md px-3 py-1.5 font-mono text-sm transition-all duration-150
    {isActive
		? '-ml-[2px] border-l-2 border-blue-600 bg-blue-50 font-semibold text-blue-700'
		: '-ml-[2px] border-l-2 border-transparent text-slate-600 hover:border-slate-300 hover:bg-white hover:text-slate-900'}"
	href={resolve(path as any)}>{name}</a
>
