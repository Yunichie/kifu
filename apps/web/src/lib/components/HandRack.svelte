<script lang="ts">
  import TileFace from '$lib/components/TileFace.svelte';

  let {
    hand,
    drawnTile = null,
    compact = false,
    winningTile = null
  }: {
    hand: number[];
    drawnTile?: number | null;
    compact?: boolean;
    winningTile?: number | null;
  } = $props();

  let concealed = $derived(hand.filter((tile) => tile !== drawnTile).toSorted((a, b) => a - b));
</script>

<div class:winning={winningTile !== null} class="hand-rack" aria-label="Hand tiles">
  <div class="tiles">
    {#each concealed as tile, index (`${tile}-${index}`)}
      <TileFace {tile} size={compact ? 'compact' : 'hand'} highlighted={tile === winningTile} />
    {/each}
  </div>
  {#if drawnTile !== null}
    <span class="drawn"><TileFace tile={drawnTile} size={compact ? 'compact' : 'hand'} highlighted={drawnTile === winningTile} /></span>
  {/if}
</div>

<style>
  .hand-rack { display: flex; min-height: 44px; max-width: 100%; align-items: flex-end; }
  .tiles { display: flex; align-items: flex-end; gap: 1px; }
  .drawn { display: inline-flex; margin-left: 8px; }
  .winning .tiles > :global(*) { transform-origin: center bottom; }
  .winning .tiles > :global(*:nth-child(odd)) { transform: rotate(-1.5deg); }
  .winning .tiles > :global(*:nth-child(even)) { transform: rotate(1.5deg); }
</style>
