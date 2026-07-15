<script lang="ts">
  import MahjongTile from '$lib/components/MahjongTile.svelte';
  import type { ReplayDiscard } from '$lib/replay/buildSnapshots';

  let { discards }: { discards: ReplayDiscard[] } = $props();
</script>

<div class="grid min-h-[67px] w-[157px] grid-cols-6 content-start gap-1" aria-label="Discards">
  {#each discards as discard (discard.tile)}
    <MahjongTile
      tile={discard.tile}
      size="discard"
      rotated={discard.calledBy !== null}
      tsumogiri={discard.tsumogiri}
      caller={discard.calledBy}
    />
  {/each}
</div>
