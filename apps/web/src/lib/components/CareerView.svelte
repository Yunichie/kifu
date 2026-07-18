<script lang="ts">
  import type { CareerStats } from '@kifu/api-types';
  import ArrowRight from 'lucide-svelte/icons/arrow-right';
  import ChartNoAxesCombined from 'lucide-svelte/icons/chart-no-axes-combined';
  import ChartCard from '$lib/components/ChartCard.svelte';
  import DealInMatrix from '$lib/components/DealInMatrix.svelte';
  import PercentBar from '$lib/components/PercentBar.svelte';
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
      count: career.placementDistribution.find((item) => item.value === value)?.count ?? 0
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

  function placementLabel(value: number): string {
    return `${value}${value === 1 ? 'st' : value === 2 ? 'nd' : value === 3 ? 'rd' : 'th'}`;
  }
</script>

{#snippet distribution(items: { label: string; count: number }[], tone: 'gold' | 'win' | 'loss' | 'ai')}
  <div class="distribution-axis" aria-hidden="true">
    <span></span><div><span>0</span><span>{maximum(items)}</span></div><span>Count</span>
  </div>
  <div class="space-y-1.5">
    {#each items as item (item.label)}
      <PercentBar label={item.label} value={item.count} maximum={maximum(items)} display={String(item.count)} {tone} />
    {/each}
  </div>
{/snippet}

<header class="mb-6">
  <p class="section-kicker">Career record</p>
  <h1 class="mt-1 font-display text-[28px] leading-[34px] font-bold">{title}</h1>
  {#if career.playerNames.length > 0}
    <div class="mt-3 flex flex-wrap gap-2">
      {#each career.playerNames as name (name)}
        <span class="stamp-tag">{name}</span>
      {/each}
    </div>
  {/if}
</header>

{#if career.games === 0}
  <section class="panel flex min-h-64 flex-col items-center justify-center gap-4 text-center">
    <span class="empty-plaque" aria-hidden="true"><i></i></span>
    <p class="text-[13px] leading-5 text-text-tertiary">No matching games in the ledger.</p>
    <a class="button-primary" href={emptyHref}>{emptyLabel}</a>
  </section>
{:else}
  <section class="mb-9 grid grid-cols-2 gap-3 sm:grid-cols-4" aria-label="Career headline statistics">
    <StatTile label="Games" value={career.games} />
    <StatTile label="Hands" value={career.stats.hands} />
    <StatTile label="Avg place" value={career.averagePlacement?.toFixed(2) ?? '—'} tone="gold" />
    <StatTile label="Win rate" value={formatPercent(career.stats.rates.winRate)} tone="win" />
    <StatTile label="Deal-in" value={formatPercent(career.stats.rates.dealInRate)} tone="loss" />
    <StatTile label="Riichi" value={formatPercent(career.stats.rates.riichiRate)} tone="gold" />
    <StatTile label="Call rate" value={formatPercent(career.stats.rates.callRate)} />
    <StatTile label="Tenpai" value={formatPercent(career.stats.rates.tenpaiRate)} tone="win" />
  </section>

  <section class="mb-5 grid gap-4 lg:grid-cols-2" aria-label="Career profiles">
    <ChartCard title="Win profile">
      <dl class="profile-list">
        <div><dt>Wins</dt><dd>{career.stats.wins}</dd></div>
        <div><dt>Tsumo share</dt><dd>{career.stats.tsumoWins} · {formatPercent(career.stats.rates.tsumoShare)}</dd></div>
        <div><dt>Ron share</dt><dd>{career.stats.ronWins} · {formatPercent(career.stats.rates.ronShare)}</dd></div>
        <div><dt>Deal-ins</dt><dd class="text-man">{career.stats.dealIns} · loss</dd></div>
      </dl>
    </ChartCard>

    <ChartCard title="Dealer & draw profile">
      <dl class="profile-list">
        <div><dt>Dealer hands</dt><dd>{career.stats.dealerHands}</dd></div>
        <div><dt>Dealer repeats</dt><dd>{career.stats.dealerRepeats} · {formatPercent(career.stats.rates.dealerRepeatRate)}</dd></div>
        <div><dt>Exhaustive draws</dt><dd>{career.stats.exhaustiveDraws}</dd></div>
        <div><dt>Tenpai / noten</dt><dd>{career.stats.tenpaiDraws} / {career.stats.notenDraws}</dd></div>
      </dl>
    </ChartCard>

    <ChartCard title="Call profile">
      <div class="profile-total"><span>Called hands</span><strong>{career.stats.calledHands}</strong></div>
      <dl class="call-grid">
        <div><dt>Chi</dt><dd>{career.stats.calls.chi}</dd></div>
        <div><dt>Pon</dt><dd>{career.stats.calls.pon}</dd></div>
        <div><dt>Open kan</dt><dd>{career.stats.calls.openKan}</dd></div>
        <div><dt>Closed kan</dt><dd>{career.stats.calls.closedKan}</dd></div>
        <div><dt>Added kan</dt><dd>{career.stats.calls.addedKan}</dd></div>
        <div><dt>Nuki</dt><dd>{career.stats.calls.nuki}</dd></div>
      </dl>
    </ChartCard>

    <ChartCard title="Dora profile">
      <div class="table-shell border-0 shadow-none">
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
  </section>

  <section class="mb-5 grid gap-4 lg:grid-cols-2" aria-label="Career distributions">
    <ChartCard title="Placement distribution">{@render distribution(placementItems, 'gold')}</ChartCard>
    <ChartCard title="Yaku frequency">
      {#if yakuItems.length > 0}{@render distribution(yakuItems, 'win')}{:else}<p class="text-[13px] text-text-tertiary">No winning yaku recorded.</p>{/if}
    </ChartCard>
  </section>

  <section class="mb-5 grid gap-4 lg:grid-cols-3" aria-label="Winning hand distributions">
    <ChartCard title="Han distribution">
      {#if hanItems.length > 0}{@render distribution(hanItems, 'ai')}{:else}<p class="text-[13px] text-text-tertiary">No winning hands recorded.</p>{/if}
    </ChartCard>
    <ChartCard title="Fu distribution">
      {#if fuItems.length > 0}{@render distribution(fuItems, 'ai')}{:else}<p class="text-[13px] text-text-tertiary">No winning hands recorded.</p>{/if}
    </ChartCard>
    <ChartCard title="Hand values">
      {#if valueItems.length > 0}{@render distribution(valueItems, 'gold')}{:else}<p class="text-[13px] text-text-tertiary">No winning hands recorded.</p>{/if}
    </ChartCard>
  </section>

  <section class="mb-5">
    <ChartCard title="Score trend">
      <div class="mb-4 flex items-center gap-2 text-[12px] leading-[18px] text-text-secondary">
        <ChartNoAxesCombined size={16} strokeWidth={1.75} aria-hidden="true" />
        Final score by game
      </div>
      <ScoreTrend points={career.scoreTrend} />

      <div class="mt-4 hidden md:block">
        <div class="table-shell">
          <table class="ledger">
            <thead><tr><th>Game</th><th>Player</th><th>Final score</th><th>Place</th></tr></thead>
            <tbody>
              {#each career.scoreTrend as point (`${point.logId}-${point.playerName}`)}
                <tr>
                  <td><a class="hover:underline" href={`/games/${point.logId}`}>{point.logId}</a></td>
                  <td class="font-sans">{point.playerName}</td>
                  <td>{formatScore(point.finalScore)}</td>
                  <td>{placementLabel(point.placement)}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>

      <ol class="mt-4 divide-y divide-border-subtle border-y border-border-subtle md:hidden">
        {#each career.scoreTrend as point (`mobile-${point.logId}-${point.playerName}`)}
          <li class="trend-record">
            <div class="min-w-0">
              <strong class="block truncate">{point.playerName}</strong>
              <span class="block truncate font-mono text-[10px] text-text-tertiary">{point.logId}</span>
            </div>
            <div class="text-right">
              <strong class="block font-mono text-[13px] tabular-nums">{formatScore(point.finalScore)}</strong>
              <span class="text-[10px] font-bold text-gold uppercase">{placementLabel(point.placement)}</span>
            </div>
            <a class="icon-button" href={`/games/${point.logId}`} aria-label={`Open game ${point.logId}`} title="Open game"><ArrowRight size={16} strokeWidth={1.75} aria-hidden="true" /></a>
          </li>
        {/each}
      </ol>
    </ChartCard>
  </section>

  <ChartCard title="Deal-in matrix"><DealInMatrix matrix={career.dealInMatrix} /></ChartCard>
{/if}

<style>
  .distribution-axis { display: grid; grid-template-columns: minmax(5.5rem, 0.8fr) minmax(6rem, 1.6fr) minmax(2.5rem, auto); align-items: end; gap: 10px; margin-bottom: 3px; color: var(--text-tertiary); font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-size: 9px; line-height: 14px; }
  .distribution-axis > div { display: flex; justify-content: space-between; border-bottom: 1px solid var(--border-subtle); }
  .distribution-axis > span:last-child { text-align: right; }
  .profile-list { font-size: 13px; line-height: 20px; }
  .profile-list div { display: flex; min-height: 40px; align-items: center; justify-content: space-between; gap: 12px; border-bottom: 1px solid var(--border-subtle); }
  .profile-list div:last-child { border-bottom: 0; }
  .profile-list dt, .call-grid dt { color: var(--text-secondary); }
  .profile-list dd, .call-grid dd, .profile-total strong { font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-variant-numeric: tabular-nums; font-weight: 600; text-align: right; }
  .profile-total { display: flex; align-items: flex-end; justify-content: space-between; border-bottom: 1px solid var(--border-subtle); padding-bottom: 12px; color: var(--text-secondary); font-size: 12px; }
  .profile-total strong { color: var(--text-primary); font-size: 20px; }
  .call-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 9px 22px; margin-top: 12px; font-size: 12px; }
  .call-grid div { display: flex; justify-content: space-between; gap: 8px; }
  .trend-record { display: grid; min-height: 64px; grid-template-columns: minmax(0, 1fr) auto 44px; align-items: center; gap: 10px; padding-block: 8px; font-size: 12px; }
  .empty-plaque { display: grid; width: 36px; height: 50px; place-items: center; border: 1px solid var(--tile-edge); border-radius: 5px; background: linear-gradient(var(--tile-highlight), var(--tile-bg)); box-shadow: 0 3px 0 var(--tile-edge); transform: rotate(-3deg); }
  .empty-plaque i { width: 14px; height: 3px; border-radius: 2px; background: var(--gold); }
  @media (max-width: 420px) { .distribution-axis { grid-template-columns: minmax(4.75rem, 0.8fr) minmax(4rem, 1fr) auto; gap: 8px; } }
</style>
