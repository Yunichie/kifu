<script lang="ts">
  let { value, maximum }: { value: number; maximum: number } = $props();
  let level = $derived(value === 0 || maximum === 0 ? 0 : Math.max(1, Math.ceil((value / maximum) * 4)));
</script>

<span class={['heatmap-cell', `level-${level}`]} title={`${value} deal-in${value === 1 ? '' : 's'}`}>
  <strong>{value}</strong>
  <span class="intensity" aria-hidden="true">
    {#each [1, 2, 3, 4] as marker (marker)}<i class={marker <= level ? 'filled' : ''}></i>{/each}
  </span>
</span>

<style>
  .heatmap-cell {
    display: grid;
    min-width: 72px;
    min-height: 54px;
    place-items: center;
    gap: 3px;
    margin: -0.75rem;
    padding: 8px 12px;
    background: var(--heat-0);
    color: var(--heat-ink-dark);
  }

  strong { font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-size: 14px; font-weight: 700; line-height: 18px; }
  .level-1 { background: var(--heat-1); }
  .level-2 { background: var(--heat-2); }
  .level-3 { background: var(--heat-3); color: var(--heat-ink-light); }
  .level-4 { background: var(--heat-4); color: var(--heat-ink-light); }
  .intensity { display: flex; height: 3px; gap: 2px; }
  .intensity i { display: block; width: 6px; height: 2px; background: currentColor; opacity: 0.24; }
  .intensity i.filled { opacity: 0.9; }
</style>
