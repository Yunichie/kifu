<script lang="ts">
  import MahjongTile from '$lib/components/MahjongTile.svelte';
  import type { ReplayMeld } from '$lib/replay/buildSnapshots';

  let {
    hand,
    melds,
    drawnTile = null,
    compact = false
  }: {
    hand: number[];
    melds: ReplayMeld[];
    drawnTile?: number | null;
    compact?: boolean;
  } = $props();

  let concealed = $derived(
    hand.filter((tile) => tile !== drawnTile).toSorted((a, b) => a - b)
  );
</script>

<div
  class={['flex min-h-[39px] items-end gap-2', compact ? 'max-w-[270px] flex-wrap' : 'max-w-[620px]']}
  aria-label="Hand"
>
  <div class="flex items-end gap-px">
    {#each concealed as tile (tile)}
      <MahjongTile {tile} size={compact ? 'compact' : 'hand'} />
    {/each}
    {#if drawnTile !== null}
      <span class="ml-2 inline-flex"><MahjongTile tile={drawnTile} size={compact ? 'compact' : 'hand'} /></span>
    {/if}
  </div>

  {#if melds.length > 0}
    <div class="flex flex-wrap items-end gap-1.5">
      {#each melds as meld (`${meld.kind}-${meld.tiles.join('-')}`)}
        <div class="flex items-center gap-px rounded-sm bg-surface-3 p-1" title={meld.kind}>
          {#each meld.tiles as tile (tile)}
            <MahjongTile
              {tile}
              size={compact ? 'compact' : 'hand'}
              rotated={tile === meld.calledTile}
            />
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</div>
