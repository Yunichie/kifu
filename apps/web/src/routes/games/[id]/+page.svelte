<script lang="ts">
  import { ArrowLeft } from 'lucide-svelte';
  import ChartCard from '$lib/components/ChartCard.svelte';
  import DealInMatrix from '$lib/components/DealInMatrix.svelte';
  import { formatPercent, formatScore, formatSigned, resultLabel, roundLabel, rulesLabel } from '$lib/format';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
  const seatBorders = ['border-t-man', 'border-t-pin', 'border-t-sou', 'border-t-gold'];
</script>

<svelte:head>
  <title>This Game | Kifu</title>
</svelte:head>

<header class="mb-6">
  <a class="mb-3 inline-flex items-center gap-1.5 text-[13px] leading-[19px] text-text-secondary hover:text-text-primary" href="/">
    <ArrowLeft size={16} strokeWidth={1.75} aria-hidden="true" />
    Game ledger
  </a>
  <div class="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
    <div>
      <p class="text-[11px] leading-[15px] font-medium uppercase text-text-tertiary">This Game</p>
      <h1 class="mt-1 font-display text-xl leading-[26px] font-semibold">{rulesLabel(data.game.rules)}</h1>
    </div>
    <code class="break-all font-mono text-[13px] leading-[19px] text-text-secondary">{data.game.logId}</code>
  </div>
  <div class="mt-3 flex flex-wrap gap-2">
    {#if data.game.rules.akaDora}<span class="rounded-sm border border-gold px-2 py-0.5 text-[11px] text-gold">Aka dora</span>{/if}
    {#if data.game.rules.kuitan}<span class="rounded-sm bg-surface-3 px-2 py-0.5 text-[11px] text-text-secondary">Kuitan</span>{/if}
    {#if data.game.rules.fast}<span class="rounded-sm bg-surface-3 px-2 py-0.5 text-[11px] text-text-secondary">Fast</span>{/if}
  </div>
</header>

<section class="mb-8 grid grid-cols-2 gap-3 sm:grid-cols-4" aria-label="Final scores">
  {#each data.game.players as player (player.seat)}
    <article class={`rounded-md border-t-[3px] bg-surface-2 p-3 ${seatBorders[player.seat] ?? 'border-t-border'}`}>
      <div class="mb-3 flex min-w-0 items-center gap-2">
        <a class="truncate font-semibold hover:underline" href={`/career/${encodeURIComponent(player.name)}`}>{player.name}</a>
        {#if player.seat === data.game.kyoku[0]?.dealer}
          <span class="shrink-0 rounded-sm bg-gold px-1.5 py-0.5 text-[9px] font-bold text-gold-ink">DEALER</span>
        {/if}
      </div>
      <div class="rounded-md bg-tile-bg px-2 py-2.5 text-center text-tile-text tile-shadow">
        <div class="font-mono text-[19px] leading-none font-semibold">{formatScore(player.finalScore)}</div>
        <div class="mt-1.5 text-[11px] leading-[15px] font-medium uppercase text-tile-text/60">
          {player.placement ? `#${player.placement}` : 'Final'}
        </div>
      </div>
      <dl class="mt-3 grid grid-cols-2 gap-x-2 gap-y-1 text-[11px] leading-[15px]">
        <dt class="text-text-tertiary">Win</dt><dd class="text-right font-mono">{formatPercent(player.stats.rates.winRate)}</dd>
        <dt class="text-text-tertiary">Deal-in</dt><dd class="text-right font-mono">{formatPercent(player.stats.rates.dealInRate)}</dd>
        <dt class="text-text-tertiary">Riichi</dt><dd class="text-right font-mono">{formatPercent(player.stats.rates.riichiRate)}</dd>
      </dl>
    </article>
  {/each}
</section>

<section class="mb-8">
  <h2 class="mb-3 font-display text-xl leading-[26px] font-semibold">Hand ledger</h2>
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
        {#each data.game.kyoku as kyoku (`${kyoku.roundIndex}-${kyoku.honba}`)}
          <tr class={`border-l-[3px] ${kyoku.result.type === 'win' ? 'border-l-sou' : 'border-l-border'}`}>
            <td>{roundLabel(kyoku.bakaze, kyoku.kyokuNumber, kyoku.honba)}</td>
            <td class="font-sans">{resultLabel(kyoku.result, data.game.players)}</td>
            {#each kyoku.endScores as score, seat (seat)}
              <td>
                {formatScore(score)}
                <span class={score - (kyoku.startScores[seat] ?? score) < 0 ? 'text-man' : 'text-sou'}>
                  {formatSigned(score - (kyoku.startScores[seat] ?? score))}
                </span>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

<ChartCard title="Deal-in matrix">
  <DealInMatrix matrix={data.game.dealInMatrix} />
</ChartCard>
