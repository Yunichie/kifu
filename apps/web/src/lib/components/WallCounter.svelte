<script lang="ts">
  import TileBack from '$lib/components/TileBack.svelte';

  let { remaining }: { remaining: number } = $props();
  let stacks = $derived(Math.min(5, Math.max(1, Math.ceil(remaining / 14))));
</script>

<div class="wall-counter" aria-label={`${remaining} tiles left in the wall`}>
  <div class="wall" aria-hidden="true">
    {#each Array.from({ length: stacks }) as _, index (index)}
      <span><TileBack size="dora" /><TileBack size="dora" /></span>
    {/each}
  </div>
  <strong>{remaining}</strong><small>left</small>
</div>

<style>
  .wall-counter { display: grid; grid-template-columns: 1fr auto; align-items: center; gap: 0 8px; }
  .wall { display: flex; min-width: 72px; align-items: center; gap: 2px; overflow: hidden; }
  .wall span { display: grid; height: 22px; transform: scale(0.7); transform-origin: left center; }
  .wall span :global(*) { grid-area: 1 / 1; }
  .wall span :global(*:last-child) { translate: 3px -4px; }
  strong { grid-row: span 2; color: var(--text-primary); font-family: "JetBrains Mono", monospace; font-size: 18px; }
  small { color: var(--text-tertiary); font-size: 8px; text-transform: uppercase; }
</style>
