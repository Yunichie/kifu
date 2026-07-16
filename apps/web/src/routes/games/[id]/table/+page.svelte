<script lang="ts">
  import Grid3X3 from 'lucide-svelte/icons/grid-3x3';
  import Rows3 from 'lucide-svelte/icons/rows-3';
  import PlaybackScrubber from '$lib/components/PlaybackScrubber.svelte';
  import TableBoard from '$lib/components/TableBoard.svelte';
  import { roundLabel } from '$lib/format';
  import { buildSnapshots } from '$lib/replay/buildSnapshots';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
  let kyokuIndex = $state(0);
  let viewedSeat = $state(0);
  let turnIndex = $state(0);
  let compactMode = $state(false);
  let activeSeats = $derived(data.game.players.map((player) => player.seat).toSorted((a, b) => a - b));
  let kyoku = $derived(data.game.kyoku[kyokuIndex]);
  let isLastKyoku = $derived(kyokuIndex === data.game.kyoku.length - 1);
  let endScores = $derived(
    isLastKyoku
      ? data.game.players.toSorted((a, b) => a.seat - b.seat).map((player) => player.finalScore ?? 0)
      : kyoku.endScores
  );
  let snapshots = $derived(
    buildSnapshots(kyoku.startHands, kyoku.events, {
      startScores: kyoku.startScores,
      endScores,
      initialDoraIndicators: kyoku.doraIndicators.slice(0, 1),
      riichiSticks: kyoku.riichiSticks,
      result: kyoku.result,
      clearRiichiSticks: isLastKyoku
    })
  );
  let snapshot = $derived(snapshots[Math.min(turnIndex, snapshots.length - 1)]);
  const seatTabs = ['border-man text-man', 'border-pin text-pin', 'border-sou text-sou', 'border-gold text-gold'];

  function selectKyoku(index: number): void {
    kyokuIndex = index;
    turnIndex = 0;
  }

</script>

<svelte:head>
  <title>Table Replay | Kifu</title>
</svelte:head>

<section>
  <div class="mb-4 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
    <div>
      <label class="field-label" for="kyoku">Hand</label>
      <select
        id="kyoku"
        class="field min-w-52"
        value={kyokuIndex}
        onchange={(event) => selectKyoku(Number(event.currentTarget.value))}
      >
        {#each data.game.kyoku as hand, index (`${hand.roundIndex}-${hand.honba}-${index}`)}
          <option value={index}>{String(index + 1).padStart(2, '0')} · {roundLabel(hand.bakaze, hand.kyokuNumber, hand.honba)}</option>
        {/each}
      </select>
    </div>

    <div class="flex flex-col gap-3 sm:items-end">
      <div class="field-label">Viewed seat</div>
      <div class="max-w-full overflow-x-auto">
        <div class="flex min-w-max border-b border-border-subtle" role="tablist" aria-label="Viewed seat">
          {#each activeSeats as seat (seat)}
            <button
              class={[
                'min-h-10 whitespace-nowrap border-b-2 px-3 text-[13px] font-semibold transition-colors duration-fast hover:bg-surface-2',
                viewedSeat === seat ? (seatTabs[seat] ?? 'border-border text-text-primary') : 'border-transparent text-text-tertiary'
              ]}
              type="button"
              role="tab"
              aria-selected={viewedSeat === seat}
              onclick={() => viewedSeat = seat}
            >S{seat + 1} {data.game.players.find((player) => player.seat === seat)?.name ?? ''}</button>
          {/each}
        </div>
      </div>
      <div class="hidden rounded-sm border border-border-subtle p-0.5 sm:flex" role="group" aria-label="Replay layout">
        <button
          class={['inline-flex min-h-8 items-center gap-1.5 rounded-sm px-2.5 text-[11px] transition-colors duration-fast', !compactMode ? 'bg-surface-3 text-gold' : 'text-text-tertiary hover:text-text-primary']}
          type="button"
          aria-pressed={!compactMode}
          onclick={() => compactMode = false}
        ><Grid3X3 size={14} strokeWidth={1.75} aria-hidden="true" />Table</button>
        <button
          class={['inline-flex min-h-8 items-center gap-1.5 rounded-sm px-2.5 text-[11px] transition-colors duration-fast', compactMode ? 'bg-surface-3 text-gold' : 'text-text-tertiary hover:text-text-primary']}
          type="button"
          aria-pressed={compactMode}
          onclick={() => compactMode = true}
        ><Rows3 size={14} strokeWidth={1.75} aria-hidden="true" />Compact</button>
      </div>
    </div>
  </div>

  <TableBoard
    {snapshot}
    players={data.game.players}
    {viewedSeat}
    dealer={kyoku.dealer}
    bakaze={kyoku.bakaze}
    kyokuNumber={kyoku.kyokuNumber}
    honba={kyoku.honba}
    result={kyoku.result}
    {compactMode}
  />

  {#key kyokuIndex}
    <PlaybackScrubber
      turn={turnIndex}
      maxTurn={snapshots.length - 1}
      event={snapshot.event}
      settled={snapshot.settled}
      onTurnChange={(next) => turnIndex = next}
    />
  {/key}
</section>
