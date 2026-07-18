<script lang="ts">
  import type { ScoreTrendPoint } from '@kifu/api-types';
  import { formatScore } from '$lib/format';

  let { points }: { points: ScoreTrendPoint[] } = $props();
  const width = 680;
  const height = 220;
  const left = 54;
  const right = 18;
  const top = 14;
  const bottom = 28;
  let minimum = $derived(points.length ? Math.min(...points.map((point) => point.finalScore)) : 0);
  let maximum = $derived(points.length ? Math.max(...points.map((point) => point.finalScore)) : 0);
  let range = $derived(Math.max(1, maximum - minimum));
  let coordinates = $derived(
    points.map((point, index) => ({
      x: points.length === 1 ? (left + width - right) / 2 : left + (index / (points.length - 1)) * (width - left - right),
      y: height - bottom - ((point.finalScore - minimum) / range) * (height - top - bottom),
      point
    }))
  );
  let polyline = $derived(coordinates.map(({ x, y }) => `${x},${y}`).join(' '));
  let area = $derived(`${left},${height - bottom} ${polyline} ${width - right},${height - bottom}`);
  let ticks = $derived(
    Array.from({ length: 5 }, (_, index) => ({
      ratio: index / 4,
      value: Math.round(maximum - (index / 4) * range),
      y: top + (index / 4) * (height - top - bottom)
    }))
  );
</script>

{#if points.length === 0}
  <p class="text-[13px] leading-5 text-text-tertiary">No score history recorded.</p>
{:else}
  <div class="trend-chart">
    <svg viewBox={`0 0 ${width} ${height}`} role="img" aria-label="Final score by game">
      <defs>
        <linearGradient id="score-wash" x1="0" x2="0" y1="0" y2="1">
          <stop offset="0" stop-color="var(--gold)" stop-opacity="0.22" />
          <stop offset="1" stop-color="var(--gold)" stop-opacity="0" />
        </linearGradient>
      </defs>
      {#each ticks as tick (tick.ratio)}
        <line class="grid-line" x1={left} x2={width - right} y1={tick.y} y2={tick.y} />
        <text class="axis-label" x={left - 8} y={tick.y + 4} text-anchor="end">{formatScore(tick.value)}</text>
      {/each}
      {#if minimum < 0 && maximum > 0}
        <line class="zero-line" x1={left} x2={width - right} y1={height - bottom - ((0 - minimum) / range) * (height - top - bottom)} y2={height - bottom - ((0 - minimum) / range) * (height - top - bottom)} />
      {/if}
      {#if points.length > 1}
        <polygon points={area} fill="url(#score-wash)" />
        <polyline class="trend-line" points={polyline} fill="none" />
      {/if}
      {#each coordinates as coordinate (`${coordinate.point.logId}-${coordinate.point.playerName}`)}
        <circle class="trend-point" cx={coordinate.x} cy={coordinate.y} r="4">
          <title>{coordinate.point.playerName}: {formatScore(coordinate.point.finalScore)}, place {coordinate.point.placement}</title>
        </circle>
      {/each}
      <text class="axis-label" x={left} y={height - 7}>Oldest</text>
      <text class="axis-label" x={width - right} y={height - 7} text-anchor="end">Latest</text>
    </svg>
  </div>
{/if}

<style>
  .trend-chart { width: 100%; overflow-x: auto; }
  svg { display: block; width: 100%; min-width: 560px; height: auto; }
  .grid-line { stroke: var(--border-subtle); stroke-width: 1; vector-effect: non-scaling-stroke; }
  .zero-line { stroke: var(--text-tertiary); stroke-dasharray: 4 4; stroke-width: 1; vector-effect: non-scaling-stroke; }
  .axis-label { fill: var(--text-tertiary); font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-size: 9px; }
  .trend-line { stroke: var(--gold); stroke-width: 2.5; stroke-linecap: round; stroke-linejoin: round; vector-effect: non-scaling-stroke; }
  .trend-point { fill: var(--surface-1); stroke: var(--gold); stroke-width: 2; vector-effect: non-scaling-stroke; }
</style>
