<script lang="ts">
  import type { Snippet } from 'svelte';

  let { title, children }: { title: string; children: Snippet } = $props();
</script>

<section class="border-t border-border-subtle bg-surface-1/45 px-0 py-5 sm:px-5">
  <h2 class="mb-4 font-display text-[16px] leading-[22px] font-semibold text-text-primary">{title}</h2>
  {@render children()}
</section>
