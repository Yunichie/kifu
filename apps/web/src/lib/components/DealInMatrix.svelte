<script lang="ts">
  import type { DealInMatrix } from '@kifu/api-types';
  import ArrowRight from 'lucide-svelte/icons/arrow-right';
  import HeatmapCell from '$lib/components/HeatmapCell.svelte';

  let { matrix }: { matrix: DealInMatrix } = $props();
  let maximum = $derived(Math.max(0, ...matrix.counts.flat()));
</script>

<div class="mb-3 flex flex-wrap items-center justify-between gap-2 text-[11px] leading-4 text-text-tertiary">
  <span class="inline-flex items-center gap-1.5">
    Dealer-in <ArrowRight size={13} strokeWidth={1.75} aria-hidden="true" /> Winner
  </span>
  <span>Bars show relative frequency</span>
</div>

<div class="table-shell matrix-shell">
  <table class="ledger text-center">
    <caption class="sr-only">Number of deal-ins from each player to every other player</caption>
    <thead>
      <tr>
        <th class="sticky left-0 z-10 min-w-32 text-left">From / To</th>
        {#each matrix.players as player (player)}
          <th class="max-w-32 min-w-24 truncate text-center" title={player}>{player}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each matrix.players as player, row (player)}
        <tr>
          <th class="sticky left-0 z-10 max-w-32 truncate border-b border-border-subtle bg-surface-2 px-3 py-3 text-left text-[12px] normal-case text-text-primary" title={player}>{player}</th>
          {#each matrix.players as recipient, column (`${row}-${column}`)}
            {#if row === column}
              <td class="bg-surface-2 text-text-tertiary"><span aria-hidden="true">—</span><span class="sr-only">Not applicable</span></td>
            {:else}
              <td aria-label={`${player} dealt into ${recipient} ${matrix.counts[row]?.[column] ?? 0} times`}><HeatmapCell value={matrix.counts[row]?.[column] ?? 0} {maximum} /></td>
            {/if}
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .matrix-shell { isolation: isolate; }
</style>
