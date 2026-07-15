<script lang="ts">
  import type { PlayerSummary } from '@kifu/api-types';
  import DiscardPile from '$lib/components/DiscardPile.svelte';
  import HandRow from '$lib/components/HandRow.svelte';
  import MahjongTile from '$lib/components/MahjongTile.svelte';
  import { formatScore } from '$lib/format';
  import type { TableSnapshot } from '$lib/replay/buildSnapshots';

  let {
    snapshot,
    players,
    viewedSeat,
    honba
  }: {
    snapshot: TableSnapshot;
    players: PlayerSummary[];
    viewedSeat: number;
    honba: number;
  } = $props();

  const seatText = ['text-man', 'text-pin', 'text-sou', 'text-gold'];
  const seatDots = ['bg-man', 'bg-pin', 'bg-sou', 'bg-gold'];
  const placement = {
    top: 'col-start-2 row-start-1 items-center',
    left: 'col-start-1 row-start-2 items-start justify-center',
    right: 'col-start-3 row-start-2 items-end justify-center',
    bottom: 'col-start-2 row-start-3 items-center justify-end'
  };
  let bottom = $derived(viewedSeat);
  let right = $derived((viewedSeat + 1) % 4);
  let top = $derived((viewedSeat + 2) % 4);
  let left = $derived((viewedSeat + 3) % 4);
  let sticks = $derived(Array.from({ length: snapshot.riichiSticks }, (_, index) => `stick-${index}`));

  function playerName(seat: number): string {
    return players.find((player) => player.seat === seat)?.name ?? `Seat ${seat + 1}`;
  }
</script>

{#snippet seatArea(seat: number, position: keyof typeof placement)}
  <section class={['flex min-w-0 flex-col gap-2', placement[position]]} aria-label={`${playerName(seat)} table area`}>
    {#if position === 'bottom'}
      <DiscardPile discards={snapshot.discards[seat]} />
    {/if}
    <div class={['text-[11px] leading-[15px] font-semibold', seatText[seat] ?? 'text-text-secondary']}>
      S{seat + 1} &middot; {playerName(seat)}
    </div>
    <HandRow
      hand={snapshot.hands[seat]}
      melds={snapshot.melds[seat]}
      drawnTile={snapshot.drawnTiles[seat]}
      compact={position !== 'bottom'}
    />
    {#if position !== 'bottom'}
      <DiscardPile discards={snapshot.discards[seat]} />
    {/if}
  </section>
{/snippet}

<div
  class="grid min-h-[660px] min-w-[860px] grid-cols-[270px_1fr_270px] grid-rows-[180px_300px_180px] rounded-md border border-border-subtle bg-surface-1 p-4"
  aria-label="Mahjong table replay"
>
  {@render seatArea(top, 'top')}
  {@render seatArea(left, 'left')}
  {@render seatArea(right, 'right')}
  {@render seatArea(bottom, 'bottom')}

  <section class="col-start-2 row-start-2 m-auto w-[244px] rounded-md border border-border bg-surface-2 p-3" aria-label="Table state">
    <div class="grid grid-cols-2 gap-x-4 gap-y-2 border-b border-border-subtle pb-3">
      <div>
        <div class="text-[10px] leading-[14px] font-medium uppercase text-text-tertiary">Tiles left</div>
        <div class="font-mono text-lg leading-6 font-semibold">{snapshot.remainingWall}</div>
      </div>
      <div>
        <div class="text-[10px] leading-[14px] font-medium uppercase text-text-tertiary">Honba</div>
        <div class="font-mono text-lg leading-6 font-semibold">{honba}</div>
      </div>
      <div class="col-span-2">
        <div class="text-[10px] leading-[14px] font-medium uppercase text-text-tertiary">Riichi sticks</div>
        <div class="mt-1 flex min-h-2 flex-wrap gap-1.5" aria-label={`${snapshot.riichiSticks} riichi sticks`}>
          {#each sticks as stick (stick)}<span class="size-2 rounded-full bg-gold"></span>{/each}
        </div>
      </div>
    </div>

    <div class="border-b border-border-subtle py-3">
      <div class="mb-1.5 text-[10px] leading-[14px] font-medium uppercase text-text-tertiary">Dora indicators</div>
      <div class="flex flex-wrap gap-2">
        {#each snapshot.doraIndicators as tile (tile)}
          <span class="flex gap-px"><MahjongTile back size="compact" /><MahjongTile {tile} size="compact" /></span>
        {/each}
      </div>
    </div>

    <div class="grid grid-cols-2 gap-x-3 gap-y-1.5 pt-3">
      {#each [0, 1, 2, 3] as seat (seat)}
        <div class={['min-w-0 border-l-2 pl-2', seat === viewedSeat ? 'text-text-primary' : 'text-text-secondary']}>
          <div class="flex items-center gap-1.5 truncate text-[10px] leading-[14px]">
            <span class={['size-1.5 shrink-0 rounded-full', seatDots[seat] ?? 'bg-border']}></span>
            S{seat + 1}
          </div>
          <div class="font-mono text-[13px] leading-[19px] font-semibold">{formatScore(snapshot.scores[seat] ?? 0)}</div>
        </div>
      {/each}
    </div>
  </section>
</div>
