<script lang="ts">
  import { formatScore } from '$lib/format';
  import type { HandScoreChartData, HandScoreSeries } from '$lib/handLedger';

  let { chart }: { chart: HandScoreChartData } = $props();

  const width = 760;
  const height = 260;
  const left = 64;
  const right = 24;
  const top = 18;
  const bottom = 48;
  const plotWidth = width - left - right;
  const plotHeight = height - top - bottom;
  const seriesColors = ['var(--man)', 'var(--pin)', 'var(--sou)', 'var(--gold)'];

  let points = $derived(chart.series[0]?.points ?? []);
  let allScores = $derived(chart.series.flatMap((series) => series.points.map((point) => point.score)));
  let minimum = $derived(allScores.length ? Math.min(...allScores) : 0);
  let maximum = $derived(allScores.length ? Math.max(...allScores) : 0);
  let scorePadding = $derived(Math.max(1000, (maximum - minimum) * 0.08));
  let plotMinimum = $derived(minimum - scorePadding);
  let plotMaximum = $derived(maximum + scorePadding);
  let scoreRange = $derived(Math.max(1, plotMaximum - plotMinimum));
  let labelStep = $derived(Math.max(1, Math.ceil(points.length / 8)));
  let ticks = $derived(
    Array.from({ length: 5 }, (_, index) => {
      const ratio = index / 4;
      return {
        id: `tick-${index}`,
        y: top + ratio * plotHeight,
        score: Math.round(plotMaximum - ratio * scoreRange)
      };
    })
  );

  function color(seat: number): string {
    return seriesColors[seat] ?? 'var(--text-secondary)';
  }

  function x(index: number): number {
    return points.length === 1 ? left + plotWidth / 2 : left + (index / (points.length - 1)) * plotWidth;
  }

  function y(score: number): number {
    return top + ((plotMaximum - score) / scoreRange) * plotHeight;
  }

  function linePoints(series: HandScoreSeries): string {
    return series.points.map((point, index) => `${x(index)},${y(point.score)}`).join(' ');
  }

  function showLabel(index: number): boolean {
    return index === 0 || index === points.length - 1 || index % labelStep === 0;
  }
</script>

{#if points.length === 0}
  <p class="text-[13px] leading-[19px] text-text-tertiary">No hands recorded.</p>
{:else}
  <ul class="mb-3 flex flex-wrap gap-x-4 gap-y-2 text-[12px] leading-[16px] text-text-secondary" aria-label="Players">
    {#each chart.series as series (series.seat)}
      <li class="flex min-w-0 items-center gap-1.5">
        <span class="series-swatch" style:--series-color={color(series.seat)} aria-hidden="true"></span>
        <span class="max-w-40 truncate">{series.name}</span>
      </li>
    {/each}
  </ul>

  <div class="overflow-x-auto">
    <svg
      class="block h-auto min-w-[640px] w-full"
      viewBox={`0 0 ${width} ${height}`}
      role="img"
      aria-label="Player scores from the start of the game through each hand"
    >
      {#each ticks as tick (tick.id)}
        <line class="grid-line" x1={left} x2={width - right} y1={tick.y} y2={tick.y} />
        <text class="axis-label" x={left - 9} y={tick.y + 4} text-anchor="end">{formatScore(tick.score)}</text>
      {/each}

      {#each points as point, index (point.id)}
        {#if showLabel(index)}
          <text class="axis-label" x={x(index)} y={height - 16} text-anchor="middle">{point.label}</text>
        {/if}
      {/each}

      {#each chart.series as series (series.seat)}
        {#if series.points.length > 1}
          <polyline
            class="series-line"
            style:--series-color={color(series.seat)}
            points={linePoints(series)}
            fill="none"
          />
        {/if}
        {#each series.points as point, index (point.id)}
          <circle
            class="series-point"
            style:--series-color={color(series.seat)}
            cx={x(index)}
            cy={y(point.score)}
            r="3.5"
          >
            <title>{series.name}, {point.label}: {formatScore(point.score)}</title>
          </circle>
        {/each}
      {/each}
    </svg>
  </div>
{/if}

<style>
  .series-swatch {
    width: 18px;
    height: 2px;
    flex: 0 0 auto;
    background: var(--series-color);
  }

  .grid-line {
    stroke: color-mix(in srgb, var(--text-primary) 8%, transparent);
    stroke-width: 1;
    vector-effect: non-scaling-stroke;
  }

  .axis-label {
    fill: var(--text-tertiary);
    font-family: "Roboto Mono", "BIZ UDPGothic", monospace;
    font-size: 10px;
  }

  .series-line {
    stroke: var(--series-color);
    stroke-width: 2;
    stroke-linejoin: round;
    stroke-linecap: round;
    vector-effect: non-scaling-stroke;
  }

  .series-point {
    fill: var(--surface-1);
    stroke: var(--series-color);
    stroke-width: 2;
    vector-effect: non-scaling-stroke;
  }
</style>
