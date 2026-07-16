<script lang="ts">
  import TileFace from '$lib/components/TileFace.svelte';
  import type { ReplayMeld } from '$lib/replay/buildSnapshots';

  let { melds, compact = false }: { melds: ReplayMeld[]; compact?: boolean } = $props();
</script>

{#if melds.length > 0}
  <div class="meld-area" aria-label="Open melds">
    {#each melds as meld, meldIndex (`${meldIndex}-${meld.kind}-${meld.tiles.join('-')}`)}
      <div class="meld" aria-label={meld.kind} title={meld.kind}>
        {#each meld.tiles as tile, index (`${tile}-${index}`)}
          <TileFace {tile} size={compact ? 'compact' : 'hand'} rotated={tile === meld.calledTile} />
        {/each}
      </div>
    {/each}
  </div>
{/if}

<style>
  .meld-area { display: flex; flex-wrap: wrap; align-items: flex-end; gap: 6px; }
  .meld { display: flex; min-height: 38px; align-items: center; gap: 1px; border-bottom: 1px solid color-mix(in srgb, var(--gold) 34%, transparent); padding: 2px 3px 4px; }
</style>
