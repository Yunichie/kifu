<script lang="ts">
  import CircleDot from 'lucide-svelte/icons/circle-dot';
  import Crown from 'lucide-svelte/icons/crown';
  import Trophy from 'lucide-svelte/icons/trophy';
  import ChartCard from '$lib/components/ChartCard.svelte';
  import DealInMatrix from '$lib/components/DealInMatrix.svelte';
  import HandScoreChart from '$lib/components/HandScoreChart.svelte';
  import { formatPercent, formatScore, formatSigned, resultLabel, roundLabel } from '$lib/format';
  import { buildHandLedger } from '$lib/handLedger';
  import { hasCareer } from '$lib/player';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
  const seatBorders = ['border-t-man', 'border-t-pin', 'border-t-sou', 'border-t-gold'];
  const seatRules = ['border-man', 'border-pin', 'border-sou', 'border-gold'];
  let handLedger = $derived(buildHandLedger(data.game));

  function deltaClass(value: number): string {
    return value > 0 ? 'delta-positive' : value < 0 ? 'delta-negative' : 'delta-neutral';
  }

  function placementLabel(placement: number | null): string {
    if (!placement) return 'Final';
    return `${placement}${placement === 1 ? 'st' : placement === 2 ? 'nd' : placement === 3 ? 'rd' : 'th'}`;
  }
</script>

{#snippet deltaValue(value: number)}
  <span class={deltaClass(value)} aria-label={`${value > 0 ? 'Gained' : value < 0 ? 'Lost' : 'No change'} ${formatScore(Math.abs(value))} points`}>
    {formatSigned(value)}
  </span>
{/snippet}

<svelte:head>
  <title>This Game | Kifu</title>
</svelte:head>

<section class="mb-9 grid grid-cols-2 gap-3 lg:grid-cols-4" aria-label="Final scores">
  {#each data.game.players as player (player.seat)}
    <article class={['player-summary', seatBorders[player.seat] ?? 'border-t-border']}>
      <header>
        {#if hasCareer(player.name)}
          <a class="min-w-0 flex-1 truncate font-bold hover:underline" href={`/career/${encodeURIComponent(player.name)}`}>{player.name}</a>
        {:else}
          <span class="min-w-0 flex-1 truncate font-bold">{player.name}</span>
        {/if}
        {#if player.placement === 1}<Crown class="shrink-0 text-gold" size={17} strokeWidth={1.75} aria-label="Winner" />{/if}
      </header>
      <div class="score-plaque tile-shadow">
        <strong>{formatScore(player.finalScore)}</strong>
        <span>{placementLabel(player.placement)}</span>
      </div>
      <dl>
        <div><dt>Win</dt><dd class="text-sou">{formatPercent(player.stats.rates.winRate)}</dd></div>
        <div><dt>Deal-in</dt><dd class="text-man">{formatPercent(player.stats.rates.dealInRate)}</dd></div>
        <div><dt>Riichi</dt><dd class="text-gold">{formatPercent(player.stats.rates.riichiRate)}</dd></div>
      </dl>
    </article>
  {/each}
</section>

<section class="mb-9">
  <h2 class="mb-4 font-display text-[20px] leading-7 font-bold">Hand ledger</h2>
  <ChartCard title="Score progression">
    <HandScoreChart chart={handLedger.chart} />
  </ChartCard>

  <div class="mt-4 hidden md:block">
    <div class="table-shell">
      <table class="ledger">
        <thead>
          <tr>
            <th>Round</th>
            <th>Result</th>
            {#each data.game.players as player (player.seat)}<th>{player.name}</th>{/each}
          </tr>
        </thead>
        <tbody>
          {#each data.game.kyoku as kyoku, index (`${kyoku.roundIndex}-${kyoku.honba}`)}
            <tr>
              <td class="whitespace-nowrap">{roundLabel(kyoku.bakaze, kyoku.kyokuNumber, kyoku.honba)}</td>
              <td class="max-w-72 font-sans">
                <span class="inline-flex items-center gap-2">
                  {#if kyoku.result.type === 'win'}<Trophy class="shrink-0 text-sou" size={15} strokeWidth={1.75} aria-hidden="true" />{:else}<CircleDot class="shrink-0 text-text-tertiary" size={15} strokeWidth={1.75} aria-hidden="true" />{/if}
                  {resultLabel(kyoku.result, data.game.players)}
                </span>
              </td>
              {#each kyoku.endScores as score, seat (seat)}
                <td>
                  <div class="score-cell"><span>{formatScore(score)}</span>{@render deltaValue(score - (kyoku.startScores[seat] ?? score))}</div>
                </td>
              {/each}
            </tr>
            {#if index === data.game.kyoku.length - 1 && handLedger.terminalSettlement}
              <tr class="final-row">
                <td>Final</td>
                <td class="font-sans font-bold">Final settlement</td>
                {#each handLedger.finalScores as score, seat (seat)}
                  <td><div class="score-cell"><span>{formatScore(score)}</span>{@render deltaValue(score - kyoku.endScores[seat])}</div></td>
                {/each}
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>
    </div>
  </div>

  <div class="mt-4 grid gap-3 md:hidden">
    {#each data.game.kyoku as kyoku, index (`mobile-${kyoku.roundIndex}-${kyoku.honba}`)}
      <article class="hand-record">
        <header>
          <div>
            <span class="section-kicker">Hand {String(index + 1).padStart(2, '0')}</span>
            <h3>{roundLabel(kyoku.bakaze, kyoku.kyokuNumber, kyoku.honba)}</h3>
          </div>
          {#if kyoku.result.type === 'win'}<Trophy class="text-sou" size={18} strokeWidth={1.75} aria-label="Win" />{:else}<CircleDot class="text-text-tertiary" size={18} strokeWidth={1.75} aria-label="Draw" />{/if}
        </header>
        <p class="result-line">{resultLabel(kyoku.result, data.game.players)}</p>
        <dl>
          {#each kyoku.endScores as score, seat (seat)}
            <div>
              <dt class={seatRules[seat] ?? 'border-border'}>{data.game.players[seat]?.name ?? `Seat ${seat + 1}`}</dt>
              <dd><span>{formatScore(score)}</span>{@render deltaValue(score - (kyoku.startScores[seat] ?? score))}</dd>
            </div>
          {/each}
        </dl>
      </article>
      {#if index === data.game.kyoku.length - 1 && handLedger.terminalSettlement}
        <article class="hand-record final-record">
          <header><h3>Final settlement</h3><Crown class="text-gold" size={18} strokeWidth={1.75} aria-hidden="true" /></header>
          <dl>
            {#each handLedger.finalScores as score, seat (seat)}
              <div>
                <dt class={seatRules[seat] ?? 'border-border'}>{data.game.players[seat]?.name ?? `Seat ${seat + 1}`}</dt>
                <dd><span>{formatScore(score)}</span>{@render deltaValue(score - kyoku.endScores[seat])}</dd>
              </div>
            {/each}
          </dl>
        </article>
      {/if}
    {/each}
  </div>
</section>

<ChartCard title="Deal-in matrix">
  <DealInMatrix matrix={data.game.dealInMatrix} />
</ChartCard>

<style>
  .player-summary { overflow: hidden; border-width: 3px 1px 1px; border-right-color: var(--border-subtle); border-bottom-color: var(--border-subtle); border-left-color: var(--border-subtle); border-radius: 6px; background: var(--surface-1); padding: 12px; box-shadow: 0 12px 28px -25px var(--shadow-strong); }
  .player-summary > header { display: flex; min-height: 28px; align-items: center; gap: 8px; font-size: 13px; line-height: 20px; }
  .score-plaque { margin-top: 9px; border-radius: 5px; background: linear-gradient(var(--tile-highlight), var(--tile-bg) 78%, var(--tile-bottom)); color: var(--tile-text); padding: 10px 8px 8px; text-align: center; }
  .score-plaque strong { display: block; font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-size: 24px; font-variant-numeric: tabular-nums; font-weight: 700; line-height: 30px; }
  .score-plaque span { display: block; margin-top: 1px; color: color-mix(in srgb, var(--tile-text) 60%, transparent); font-size: 10px; font-weight: 700; line-height: 16px; text-transform: uppercase; }
  .player-summary dl { display: grid; gap: 4px; margin-top: 13px; font-size: 11px; line-height: 16px; }
  .player-summary dl div { display: flex; justify-content: space-between; gap: 8px; }
  .player-summary dt { color: var(--text-tertiary); }
  .player-summary dd { font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-variant-numeric: tabular-nums; font-weight: 600; }
  .score-cell { display: grid; gap: 2px; white-space: nowrap; }
  .score-cell > span:first-child { color: var(--text-primary); }
  .final-row { background: color-mix(in srgb, var(--gold) 8%, transparent); }
  .hand-record { overflow: hidden; border: 1px solid var(--border-subtle); border-radius: 6px; background: var(--surface-1); }
  .hand-record > header { display: flex; min-height: 56px; align-items: center; justify-content: space-between; gap: 12px; border-bottom: 1px solid var(--border-subtle); background: var(--surface-2); padding: 9px 12px; }
  .hand-record h3 { font-family: "Zen Old Mincho", "Noto Serif JP", serif; font-size: 15px; font-weight: 700; line-height: 22px; }
  .result-line { border-bottom: 1px solid var(--border-subtle); padding: 9px 12px; color: var(--text-secondary); font-size: 12px; line-height: 18px; }
  .hand-record dl { padding: 4px 12px; }
  .hand-record dl div { display: flex; min-height: 42px; align-items: center; justify-content: space-between; gap: 12px; border-bottom: 1px solid var(--border-subtle); font-size: 12px; }
  .hand-record dl div:last-child { border-bottom: 0; }
  .hand-record dt { min-width: 0; overflow: hidden; border-left-width: 3px; padding-left: 8px; text-overflow: ellipsis; white-space: nowrap; }
  .hand-record dd { display: flex; align-items: center; gap: 10px; font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-variant-numeric: tabular-nums; }
  .final-record { border-color: color-mix(in srgb, var(--gold) 45%, var(--border-subtle)); }
  @media (max-width: 420px) { .score-plaque strong { font-size: 20px; } .player-summary { padding: 10px; } }
</style>
