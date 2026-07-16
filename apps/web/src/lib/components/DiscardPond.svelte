<script lang="ts">
  import TileFace from '$lib/components/TileFace.svelte';
  import type { ReplayDiscard } from '$lib/replay/buildSnapshots';

  let { discards, compact = false }: { discards: ReplayDiscard[]; compact?: boolean } = $props();
</script>

<div class:compact class="discard-pond" aria-label="Discard pond">
  {#each discards as discard, index (`${index}-${discard.tile}`)}
    <span class="discard-cell">
      <TileFace
        tile={discard.tile}
        size="discard"
        rotated={discard.riichi}
        dimmed={discard.calledBy !== null}
      />
    </span>
  {/each}
</div>

<style>
  .discard-pond {
    display: grid;
    width: 204px;
    min-height: 68px;
    grid-template-columns: repeat(6, 34px);
    grid-auto-rows: 34px;
    align-content: start;
  }

  .discard-cell {
    display: flex;
    width: 34px;
    height: 34px;
    align-items: center;
    justify-content: center;
  }

  .compact {
    width: max-content;
    min-height: 34px;
    grid-auto-flow: column;
    grid-template-columns: none;
    grid-template-rows: 34px;
  }
</style>
