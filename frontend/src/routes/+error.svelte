<script lang="ts">
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
</script>

<div class="flex flex-col items-center justify-center h-screen text-center">
  <h1 class="text-6xl font-bold text-slate-800">{page.status}</h1>
  <p class="text-xl text-slate-600 mt-4">
    {page.error?.message || "Oops! This page doesn't exist."}
  </p>
  <a href="{resolve('/')}" class="mt-6 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
    Back to Home
  </a>
</div>