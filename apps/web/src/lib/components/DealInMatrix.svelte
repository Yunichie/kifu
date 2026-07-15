<script lang="ts">
  import type { DealInMatrix } from '@kifu/api-types';

  let { matrix }: { matrix: DealInMatrix } = $props();
  let maximum = $derived(Math.max(0, ...matrix.counts.flat()));

  function tint(count: number): string {
    return maximum === 0 ? '0%' : `${Math.round((count / maximum) * 35)}%`;
  }
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
              <td class="bg-surface-1 text-text-tertiary">-</td>
            {:else}
              <td style:--cell-tint={tint(matrix.counts[row]?.[column] ?? 0)}>
                <span class="matrix-cell">{matrix.counts[row]?.[column] ?? 0}</span>
              </td>
            {/if}
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .matrix-cell {
    display: block;
    margin: -0.625rem -0.75rem;
    padding: 0.625rem 0.75rem;
    background: color-mix(in srgb, var(--man) var(--cell-tint), transparent);
  }
</style>
