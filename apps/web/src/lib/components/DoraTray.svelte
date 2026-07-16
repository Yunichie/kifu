<script lang="ts">
  import TileBack from '$lib/components/TileBack.svelte';
  import TileFace from '$lib/components/TileFace.svelte';

  let { indicators }: { indicators: number[] } = $props();
</script>

<div class="dora-tray" aria-label="Dora indicators">
  <span class="tray-label">DORA</span>
  <div class="tiles">
    <TileBack size="dora" />
    {#each indicators as tile, index (`${tile}-${index}`)}<TileFace {tile} size="dora" indicator />{/each}
  </div>
</div>

<style>
  .dora-tray { display: flex; align-items: center; justify-content: space-between; gap: 8px; border-block: 1px solid color-mix(in srgb, var(--kinari) 8%, transparent); padding: 7px 0; }
  .tray-label { color: var(--text-tertiary); font-family: "JetBrains Mono", monospace; font-size: 8px; font-weight: 600; }
  .tiles { display: flex; min-height: 32px; align-items: center; gap: 2px; }
</style>
