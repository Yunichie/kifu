<script lang="ts">
  import type { DealInMatrix } from '@kifu/api-types';
  import HeatmapCell from '$lib/components/HeatmapCell.svelte';

  let { matrix }: { matrix: DealInMatrix } = $props();
  let maximum = $derived(Math.max(0, ...matrix.counts.flat()));

</script>

<div class="table-shell">
  <table class="ledger text-center">
    <thead>
      <tr>
        <th>From / To</th>
        {#each matrix.players as player (player)}
          <th class="max-w-28 truncate text-center" title={player}>{player}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each matrix.players as player, row (player)}
        <tr>
          <th class="max-w-32 truncate text-left" title={player}>{player}</th>
          {#each matrix.players as _recipient, column (`${row}-${column}`)}
            {#if row === column}
              <td class="bg-surface-2 text-text-tertiary">-</td>
            {:else}
              <td><HeatmapCell value={matrix.counts[row]?.[column] ?? 0} {maximum} /></td>
            {/if}
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>
