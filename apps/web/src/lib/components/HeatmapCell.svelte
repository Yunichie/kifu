<script lang="ts">
  let { value, maximum }: { value: number; maximum: number } = $props();
  let strength = $derived(maximum === 0 ? '0%' : `${Math.round(12 + (value / maximum) * 70)}%`);
</script>

<span class="heatmap-cell" style:--strength={value === 0 ? '0%' : strength}>{value}</span>

<style>
  .heatmap-cell { display: block; margin: -0.625rem -0.75rem; padding: 0.625rem 0.75rem; background: color-mix(in srgb, var(--shu) var(--strength), var(--kinari)); color: color-mix(in srgb, var(--tile-text) 88%, transparent); font-weight: 600; }
</style>
