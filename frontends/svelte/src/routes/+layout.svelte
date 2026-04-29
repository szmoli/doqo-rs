<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import registryJson from '$lib/json/registry.json';
	import SymbolLink from '$lib/components/SymbolLink.svelte';
	import type { DoqoRegistry } from '$lib/bindings/DoqoRegistry';
	import SearchBar from '$lib/components/SearchBar.svelte';

	const registry = registryJson as DoqoRegistry;

	let { children } = $props();
	let symbolsList = $derived(Object.values(registry.symbols));

	const symbolsByKind = $derived(Object.groupBy(symbolsList, (s) => s.kind));
	const kinds = $derived(Object.keys(symbolsByKind));
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<div class="flex h-screen">
	<aside class="overflow-y-auto border-r border-slate-200 px-2">
		<header class="flex items-center justify-between border-b border-slate-200 py-4 ">
				<SearchBar symbolTable={registry} />
		</header>
		<nav>
			<h2 class="mb-2 mt-4 text-s font-bold tracking-widest uppercase">Symbols</h2>
			{#each kinds as kind (kind)}
				<h3 class="mb-2 flex items-center justify-between text-xs font-semibold text-slate-500">
					{kind}
				</h3>
				<ul class="mb-2">
					{#each symbolsByKind[kind] as symbol (symbol)}
						<li>
							<SymbolLink {symbol} />
						</li>
					{/each}
				</ul>
			{/each}
		</nav>
	</aside>

	<div class="flex-1 overflow-y-auto px-2">
		{@render children()}
	</div>
</div>
