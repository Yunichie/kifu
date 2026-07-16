<script lang="ts">
  import type { CareerStats } from '@kifu/api-types';
  import ChartNoAxesCombined from 'lucide-svelte/icons/chart-no-axes-combined';
  import FileSearch from 'lucide-svelte/icons/file-search';
  import ChartCard from '$lib/components/ChartCard.svelte';
  import DealInMatrix from '$lib/components/DealInMatrix.svelte';
  import ScoreTrend from '$lib/components/ScoreTrend.svelte';
  import StatTile from '$lib/components/StatTile.svelte';
  import { formatPercent, formatScore } from '$lib/format';

  let {
    career,
    title,
    emptyHref,
    emptyLabel
  }: {
    career: CareerStats;
    title: string;
    emptyHref: string;
    emptyLabel: string;
  } = $props();

  let placementItems = $derived(
    [1, 2, 3, 4].map((value) => ({
      label: `${value}${value === 1 ? 'st' : value === 2 ? 'nd' : value === 3 ? 'rd' : 'th'}`,
      count: career.placementDistribution.find((item) => item.value === value)?.count ?? 0,
      opacity: `${100 - (value - 1) * 20}%`
    }))
  );
  let yakuItems = $derived(
    [...career.stats.yakuFrequency]
      .sort((a, b) => b.count - a.count || a.name.localeCompare(b.name))
      .map((item) => ({ label: item.name, count: item.count }))
  );
  let hanItems = $derived(career.stats.hanDistribution.map((item) => ({ label: `${item.value} han`, count: item.count })));
  let fuItems = $derived(career.stats.fuDistribution.map((item) => ({ label: `${item.value} fu`, count: item.count })));
  let valueItems = $derived(
    career.stats.handValueDistribution.map((item) => ({ label: formatScore(item.value), count: item.count }))
  );

  function maximum(items: { count: number }[]): number {
    return Math.max(1, ...items.map((item) => item.count));
  }
</script>

{#snippet bars(items: { label: string; count: number; opacity?: string }[])}
  <div class="space-y-2">
    {#each items as item (item.label)}
      <div class="grid grid-cols-[minmax(5rem,auto)_1fr_2rem] items-center gap-3 text-[13px] leading-[19px]">
        <span class="truncate text-text-secondary" title={item.label}>{item.label}</span>
        <div class="h-2 overflow-hidden rounded-sm bg-surface-3">
          <div
            class="bar h-full bg-gold"
            style:--bar-width={`${(item.count / maximum(items)) * 100}%`}
            style:--bar-opacity={item.opacity ?? '100%'}
          ></div>
        </div>
        <span class="text-right font-mono text-text-primary">{item.count}</span>
      </div>
    {/each}
  </div>
{/snippet}

<header class="mb-6">
  <p class="text-[11px] leading-[15px] font-medium uppercase text-text-tertiary">Career</p>
  <h1 class="mt-1 font-display text-xl leading-[26px] font-semibold">{title}</h1>
  {#if career.playerNames.length > 0}
    <div class="mt-3 flex flex-wrap gap-2">
      {#each career.playerNames as name (name)}
        <span class="rounded-sm bg-surface-3 px-2.5 py-1 text-[11px] leading-[15px] font-medium text-text-secondary">{name}</span>
      {/each}
    </div>
  {/if}
</header>

{#if career.games === 0}
  <section class="panel flex min-h-56 flex-col items-center justify-center gap-3 text-center">
    <FileSearch class="text-text-tertiary opacity-20" size={40} strokeWidth={1.5} aria-hidden="true" />
    <p class="text-[13px] leading-[19px] text-text-tertiary">No matching games in the ledger.</p>
    <a class="button-primary" href={emptyHref}>{emptyLabel}</a>
  </section>
{:else}
  <section class="mb-8 grid grid-cols-2 gap-3 sm:grid-cols-4" aria-label="Career headline statistics">
    <StatTile label="Games" value={career.games} />
    <StatTile label="Hands" value={career.stats.hands} />
    <StatTile label="Avg place" value={career.averagePlacement?.toFixed(2) ?? '-'} />
    <StatTile label="Win rate" value={formatPercent(career.stats.rates.winRate)} />
    <StatTile label="Deal-in" value={formatPercent(career.stats.rates.dealInRate)} />
    <StatTile label="Riichi" value={formatPercent(career.stats.rates.riichiRate)} />
    <StatTile label="Call rate" value={formatPercent(career.stats.rates.callRate)} />
    <StatTile label="Tenpai" value={formatPercent(career.stats.rates.tenpaiRate)} />
  </section>

  <div class="mb-4 grid gap-4 sm:grid-cols-2">
    <ChartCard title="Win profile">
      <dl class="divide-y divide-border-subtle text-[13px] leading-[19px]">
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Wins</dt><dd class="font-mono">{career.stats.wins}</dd></div>
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Tsumo</dt><dd class="font-mono">{career.stats.tsumoWins} · {formatPercent(career.stats.rates.tsumoShare)}</dd></div>
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Ron</dt><dd class="font-mono">{career.stats.ronWins} · {formatPercent(career.stats.rates.ronShare)}</dd></div>
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Deal-ins</dt><dd class="font-mono text-man">{career.stats.dealIns}</dd></div>
      </dl>
    </ChartCard>

    <ChartCard title="Dealer & draw profile">
      <dl class="divide-y divide-border-subtle text-[13px] leading-[19px]">
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Dealer hands</dt><dd class="font-mono">{career.stats.dealerHands}</dd></div>
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Dealer repeats</dt><dd class="font-mono">{career.stats.dealerRepeats} · {formatPercent(career.stats.rates.dealerRepeatRate)}</dd></div>
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Exhaustive draws</dt><dd class="font-mono">{career.stats.exhaustiveDraws}</dd></div>
        <div class="flex justify-between py-2"><dt class="text-text-secondary">Tenpai / noten</dt><dd class="font-mono">{career.stats.tenpaiDraws} / {career.stats.notenDraws}</dd></div>
      </dl>
    </ChartCard>
  </div>

  <div class="mb-4 grid gap-4 sm:grid-cols-2">
    <ChartCard title="Calls">
      <div class="mb-3 flex items-end justify-between border-b border-border-subtle pb-3">
        <span class="text-text-secondary">Called hands</span>
        <span class="font-mono">{career.stats.calledHands}</span>
      </div>
      <dl class="grid grid-cols-2 gap-x-5 gap-y-2 text-[13px] leading-[19px]">
        <div class="flex justify-between"><dt class="text-text-secondary">Chi</dt><dd class="font-mono">{career.stats.calls.chi}</dd></div>
        <div class="flex justify-between"><dt class="text-text-secondary">Pon</dt><dd class="font-mono">{career.stats.calls.pon}</dd></div>
        <div class="flex justify-between"><dt class="text-text-secondary">Open kan</dt><dd class="font-mono">{career.stats.calls.openKan}</dd></div>
        <div class="flex justify-between"><dt class="text-text-secondary">Closed kan</dt><dd class="font-mono">{career.stats.calls.closedKan}</dd></div>
        <div class="flex justify-between"><dt class="text-text-secondary">Added kan</dt><dd class="font-mono">{career.stats.calls.addedKan}</dd></div>
        <div class="flex justify-between"><dt class="text-text-secondary">Nuki</dt><dd class="font-mono">{career.stats.calls.nuki}</dd></div>
      </dl>
    </ChartCard>

    <ChartCard title="Dora profile">
      <div class="table-shell border-0">
        <table class="ledger min-w-full">
          <thead><tr><th>Type</th><th>Count</th><th>Hit rate</th><th>Wins</th></tr></thead>
          <tbody>
            <tr><td class="font-sans">Dora</td><td>{career.stats.bonuses.dora}</td><td>{formatPercent(career.stats.bonuses.doraHitRate)}</td><td>{career.stats.bonuses.winsWithDora}</td></tr>
            <tr><td class="font-sans">Aka</td><td>{career.stats.bonuses.akaDora}</td><td>{formatPercent(career.stats.bonuses.akaDoraHitRate)}</td><td>{career.stats.bonuses.winsWithAkaDora}</td></tr>
            <tr><td class="font-sans">Ura</td><td>{career.stats.bonuses.uraDora}</td><td>{formatPercent(career.stats.bonuses.uraDoraHitRate)}</td><td>{career.stats.bonuses.winsWithUraDora}</td></tr>
          </tbody>
        </table>
      </div>
    </ChartCard>
  </div>

  <div class="mb-4 grid gap-4 sm:grid-cols-2">
    <ChartCard title="Placement distribution">{@render bars(placementItems)}</ChartCard>
    <ChartCard title="Yaku frequency">
      {#if yakuItems.length > 0}{@render bars(yakuItems)}{:else}<p class="text-[13px] text-text-tertiary">No winning yaku recorded.</p>{/if}
    </ChartCard>
  </div>

  <div class="mb-4 grid gap-4 sm:grid-cols-3">
    <ChartCard title="Han distribution">
      {#if hanItems.length > 0}{@render bars(hanItems)}{:else}<p class="text-[13px] text-text-tertiary">No winning hands recorded.</p>{/if}
    </ChartCard>
    <ChartCard title="Fu distribution">
      {#if fuItems.length > 0}{@render bars(fuItems)}{:else}<p class="text-[13px] text-text-tertiary">No winning hands recorded.</p>{/if}
    </ChartCard>
    <ChartCard title="Hand values">
      {#if valueItems.length > 0}{@render bars(valueItems)}{:else}<p class="text-[13px] text-text-tertiary">No winning hands recorded.</p>{/if}
    </ChartCard>
  </div>

  <ChartCard title="Score trend">
    <div class="mb-4 flex items-center gap-2 text-[13px] text-text-secondary">
      <ChartNoAxesCombined size={16} strokeWidth={1.75} aria-hidden="true" />
      Final score by game
    </div>
    <ScoreTrend points={career.scoreTrend} />
    <div class="table-shell mt-4">
      <table class="ledger">
        <thead><tr><th>Game</th><th>Player</th><th>Final score</th><th>Place</th></tr></thead>
        <tbody>
          {#each career.scoreTrend as point (`${point.logId}-${point.playerName}`)}
            <tr>
              <td><a class="hover:underline" href={`/games/${point.logId}`}>{point.logId}</a></td>
              <td class="font-sans">{point.playerName}</td>
              <td>{formatScore(point.finalScore)}</td>
              <td>#{point.placement}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </ChartCard>

  <div class="mt-4">
    <ChartCard title="Deal-in matrix"><DealInMatrix matrix={career.dealInMatrix} /></ChartCard>
  </div>
{/if}

<style>
  .bar {
    width: var(--bar-width);
    opacity: var(--bar-opacity);
    transition: width var(--motion-base) ease-out;
  }
</style>
