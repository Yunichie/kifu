<script lang="ts">
  import type { ScoreTrendPoint } from '@kifu/api-types';

  let { points }: { points: ScoreTrendPoint[] } = $props();
  const width = 640;
  const height = 180;
  const padding = 16;
  let minimum = $derived(Math.min(...points.map((point) => point.finalScore)));
  let maximum = $derived(Math.max(...points.map((point) => point.finalScore)));
  let range = $derived(Math.max(1, maximum - minimum));
  let coordinates = $derived(
    points.map((point, index) => ({
      x: points.length === 1 ? width / 2 : padding + (index / (points.length - 1)) * (width - padding * 2),
      y: height - padding - ((point.finalScore - minimum) / range) * (height - padding * 2),
      point
    }))
  );
  let polyline = $derived(coordinates.map(({ x, y }) => `${x},${y}`).join(' '));
  let area = $derived(`${padding},${height - padding} ${polyline} ${width - padding},${height - padding}`);
</script>

<div class="aspect-[16/5] min-h-44 w-full" aria-hidden="true">
  <svg class="h-full w-full" viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="none">
    <defs>
      <linearGradient id="ink-wash" x1="0" x2="0" y1="0" y2="1">
        <stop offset="0" stop-color="var(--gold)" stop-opacity="0.18" />
        <stop offset="1" stop-color="var(--gold)" stop-opacity="0" />
      </linearGradient>
    </defs>
    {#each [0.25, 0.5, 0.75] as position (position)}
      <line class="grid-line" x1={padding} x2={width - padding} y1={height * position} y2={height * position} />
    {/each}
    {#if points.length > 1}
      <polygon points={area} fill="url(#ink-wash)" />
      <polyline class="trend-line" points={polyline} fill="none" />
    {/if}
    {#each coordinates as coordinate (`${coordinate.point.logId}-${coordinate.point.playerName}`)}
      <circle class="trend-point" cx={coordinate.x} cy={coordinate.y} r="4" />
    {/each}
  </svg>
</div>

<style>
  .grid-line {
    stroke: color-mix(in srgb, var(--text-primary) 5%, transparent);
    stroke-width: 1;
  }

  .trend-line {
    stroke: var(--gold);
    stroke-width: 2;
    vector-effect: non-scaling-stroke;
  }

  .trend-point {
    fill: var(--surface-1);
    stroke: var(--gold);
    stroke-width: 2;
    vector-effect: non-scaling-stroke;
  }
</style>
