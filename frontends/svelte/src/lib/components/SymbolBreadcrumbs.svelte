<script lang="ts">
	import type { DoqoSymbol } from '$lib/bindings/DoqoSymbol';

	let { symbol } = $props<{
		symbol: DoqoSymbol;
	}>();

	const parts = $derived(symbol.fqid.split('::'));
</script>

<nav class="mb-4 flex flex-wrap items-center gap-y-1 font-mono text-sm text-slate-500">
	{#each parts as part, i}
		{@const currentPath = '/' + parts.slice(0, i + 1).join('/')}
		{@const isLast = i === parts.length - 1}

		<div class="flex items-center">
			{#if isLast}
				<span class="font-bold text-slate-900">{part}</span>
			{:else}
				<a href={currentPath} class="transition-colors hover:text-blue-600 hover:underline">
					{part}
				</a>
				<span class="mx-1 text-slate-300">::</span>
			{/if}
		</div>
	{/each}
</nav>
