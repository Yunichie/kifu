<script lang="ts">
  import type { Snippet } from 'svelte';

  let { title, children }: { title: string; children: Snippet } = $props();
</script>

<section class="rounded-md border border-border-subtle bg-surface-1 p-5">
  <h2 class="mb-3 text-[15px] leading-[21px] font-semibold text-text-primary">{title}</h2>
  {@render children()}
</section>
