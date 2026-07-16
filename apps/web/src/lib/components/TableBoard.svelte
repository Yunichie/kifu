<script lang="ts">
  import type { KyokuResult, PlayerSummary, WinResult } from '@kifu/api-types';
  import DiscardPond from '$lib/components/DiscardPond.svelte';
  import DoraTray from '$lib/components/DoraTray.svelte';
  import HandRack from '$lib/components/HandRack.svelte';
  import MeldArea from '$lib/components/MeldArea.svelte';
  import ScorePlaque from '$lib/components/ScorePlaque.svelte';
  import WallCounter from '$lib/components/WallCounter.svelte';
  import WindMarker from '$lib/components/WindMarker.svelte';
  import { formatScore, roundLabel } from '$lib/format';
  import type { TableSnapshot } from '$lib/replay/buildSnapshots';

  let {
    snapshot,
    players,
    viewedSeat,
    dealer,
    bakaze,
    kyokuNumber,
    honba,
    result,
    compactMode = false
  }: {
    snapshot: TableSnapshot;
    players: PlayerSummary[];
    viewedSeat: number;
    dealer: number;
    bakaze: number;
    kyokuNumber: number;
    honba: number;
    result: KyokuResult;
    compactMode?: boolean;
  } = $props();

  const seatTones = ['var(--shu)', 'var(--ai)', 'var(--take)', 'var(--gold)'];
  const positions = {
    top: 'seat-top',
    left: 'seat-left',
    right: 'seat-right',
    bottom: 'seat-bottom'
  };
  let activeSeats = $derived(players.map((player) => player.seat).toSorted((a, b) => a - b));
  let viewedIndex = $derived(Math.max(0, activeSeats.indexOf(viewedSeat)));
  let bottom = $derived(activeSeats[viewedIndex] ?? viewedSeat);
  let right = $derived(activeSeats.length > 1 ? activeSeats[(viewedIndex + 1) % activeSeats.length] : null);
  let top = $derived(activeSeats.length === 4 ? activeSeats[(viewedIndex + 2) % activeSeats.length] : null);
  let left = $derived(activeSeats.length > 2 ? activeSeats[(viewedIndex + activeSeats.length - 1) % activeSeats.length] : null);
  let mobileSeats = $derived([bottom, right, top, left].filter((seat): seat is number => seat !== null));
  let eventSeat = $derived(snapshot.event && 'seat' in snapshot.event ? snapshot.event.seat : -1);
  let roundJapanese = $derived(`${['東', '南', '西', '北'][bakaze] ?? ''}${kyokuNumber}局`);
  let riichiSticks = $derived(Array.from({ length: snapshot.riichiSticks }, (_, index) => index));

  function playerName(seat: number): string {
    return players.find((player) => player.seat === seat)?.name ?? `Seat ${seat + 1}`;
  }

  function seatWind(seat: number): string {
    const dealerIndex = activeSeats.indexOf(dealer);
    const seatIndex = activeSeats.indexOf(seat);
    return ['東', '南', '西', '北'][(seatIndex - dealerIndex + activeSeats.length) % activeSeats.length] ?? '?';
  }

  function winFor(seat: number): WinResult | null {
    if (!snapshot.settled || result.type !== 'win') return null;
    return result.wins.find((win) => win.winner === seat) ?? null;
  }
</script>

{#snippet winTag(win: WinResult)}
  <aside class="win-tag" aria-label="Winning hand result">
    <span class="tag-knot"></span>
    <strong>{win.han} han · {win.fu} fu</strong>
    <span>{formatScore(win.points)} pts</span>
    <small>{win.yaku.filter((yaku) => yaku.han > 0 || yaku.yakuman).map((yaku) => yaku.name).join(' · ')}</small>
  </aside>
{/snippet}

{#snippet seatHeader(seat: number)}
  <header class="seat-header">
    <WindMarker wind={seatWind(seat)} dealer={seat === dealer} />
    <div class="seat-name">
      <span>S{seat + 1} · {seatWind(seat)} wind</span>
      <strong title={playerName(seat)}>{playerName(seat)}</strong>
    </div>
    <span class="seat-score">{formatScore(snapshot.scores[seat] ?? 0)}</span>
  </header>
{/snippet}

{#snippet handAndMelds(seat: number, compact: boolean)}
  <div class="rack-line">
    <HandRack
      hand={snapshot.hands[seat]}
      drawnTile={snapshot.drawnTiles[seat]}
      {compact}
      winningTile={winFor(seat)?.wait ?? null}
    />
    <MeldArea melds={snapshot.melds[seat]} {compact} />
  </div>
{/snippet}

{#snippet seatArea(seat: number, position: keyof typeof positions)}
  <section
    class={['seat-zone', positions[position], eventSeat === seat && 'is-active', winFor(seat) && 'is-winner']}
    style:--seat-color={seatTones[seat] ?? 'var(--text-secondary)'}
    aria-label={`${playerName(seat)} table area`}
  >
    {#if position === 'bottom'}
      <DiscardPond discards={snapshot.discards[seat]} />
      {@render seatHeader(seat)}
      {@render handAndMelds(seat, false)}
    {:else if position === 'top'}
      {@render seatHeader(seat)}
      {@render handAndMelds(seat, true)}
      <DiscardPond discards={snapshot.discards[seat]} />
    {:else if position === 'left'}
      <div class="side-rack">{@render seatHeader(seat)}{@render handAndMelds(seat, true)}</div>
      <DiscardPond discards={snapshot.discards[seat]} />
    {:else}
      <DiscardPond discards={snapshot.discards[seat]} />
      <div class="side-rack">{@render seatHeader(seat)}{@render handAndMelds(seat, true)}</div>
    {/if}
    {#if winFor(seat)}{@render winTag(winFor(seat)!)}{/if}
  </section>
{/snippet}

{#snippet consoleContent(compact = false)}
  <section class:compact class="center-console" aria-label="Table state">
    <div class="round-plaque">
      <div><strong lang="ja">{roundJapanese}</strong><span>{honba}本場</span></div>
      <small>{roundLabel(bakaze, kyokuNumber, honba)}</small>
    </div>
    <WallCounter remaining={snapshot.remainingWall} />
    <DoraTray indicators={snapshot.doraIndicators} />
    <div class="riichi-row">
      <span>Riichi</span>
      <div class="sticks" aria-label={`${snapshot.riichiSticks} riichi sticks`}>
        {#each riichiSticks as stick (stick)}<i><b></b></i>{/each}
        {#if snapshot.riichiSticks === 0}<small>0</small>{/if}
      </div>
    </div>
    {#if !compact}
      <div class="score-grid">
        {#each activeSeats as seat (seat)}
          <ScorePlaque {seat} name={playerName(seat)} score={snapshot.scores[seat] ?? 0} active={seat === viewedSeat} />
        {/each}
      </div>
    {/if}
  </section>
{/snippet}

<div class:forced-compact={compactMode} class="board-root">
  <div class="table-view">
    <div class="wood-rail">
      <div class="gold-inlay">
        <div class="felt">
          {#if top !== null}{@render seatArea(top, 'top')}{/if}
          {#if left !== null}{@render seatArea(left, 'left')}{/if}
          {#if right !== null}{@render seatArea(right, 'right')}{/if}
          {@render seatArea(bottom, 'bottom')}
          {@render consoleContent()}
        </div>
      </div>
    </div>
  </div>

  <div class="compact-view">
    {@render consoleContent(true)}
    <div class="compact-seats">
      {#each mobileSeats as seat (seat)}
        <section class={['compact-seat', eventSeat === seat && 'is-active', winFor(seat) && 'is-winner']}>
          {@render seatHeader(seat)}
          <div class="pond-scroll"><DiscardPond discards={snapshot.discards[seat]} compact /></div>
          <div class="rack-scroll">{@render handAndMelds(seat, false)}</div>
          {#if winFor(seat)}{@render winTag(winFor(seat)!)}{/if}
        </section>
      {/each}
    </div>
  </div>
</div>

<style>
  .board-root { width: 100%; }
  .compact-view { display: none; }
  .wood-rail { min-width: 940px; aspect-ratio: 1.34; border: 1px solid var(--wood-dark); border-radius: 42px; background-color: var(--wood); background-image: repeating-linear-gradient(4deg, rgba(242, 234, 217, 0.04) 0 1px, transparent 1px 8px); box-shadow: inset 0 0 0 2px rgba(242, 234, 217, 0.07), 0 18px 45px rgba(0, 0, 0, 0.38); padding: 13px; }
  .gold-inlay { width: 100%; height: 100%; border: 2px solid color-mix(in srgb, var(--gold) 74%, var(--kincha-dark)); border-radius: 33px; padding: 7px; box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.5), 0 0 8px color-mix(in srgb, var(--gold) 15%, transparent); }
  .felt { position: relative; width: 100%; height: 100%; overflow: hidden; border-radius: 25px; background-color: var(--felt); background-image: repeating-linear-gradient(92deg, rgba(242, 234, 217, 0.012) 0 1px, transparent 1px 5px); box-shadow: inset 0 0 34px rgba(0, 0, 0, 0.48); }
  .seat-zone { position: absolute; z-index: 2; display: flex; min-width: 0; gap: 7px; border: 1px solid transparent; border-radius: 7px; padding: 6px; transition: border-color var(--motion-base), box-shadow var(--motion-base), background var(--motion-base); }
  .seat-zone.is-active, .compact-seat.is-active { border-color: color-mix(in srgb, var(--gold) 55%, transparent); background: color-mix(in srgb, var(--gold) 6%, transparent); box-shadow: 0 0 18px color-mix(in srgb, var(--gold) 11%, transparent); }
  .seat-zone.is-winner { border-color: var(--gold); }
  .seat-top { top: 13px; left: 50%; width: 430px; flex-direction: column; align-items: center; transform: translateX(-50%); }
  .seat-bottom { bottom: 14px; left: 50%; width: 510px; flex-direction: column; align-items: center; transform: translateX(-50%); }
  .seat-left { top: 40%; left: 10px; width: 312px; flex-direction: column; align-items: center; transform: translateY(-50%); }
  .seat-right { top: 40%; right: 10px; width: 312px; flex-direction: column; align-items: center; transform: translateY(-50%); }
  .side-rack { display: flex; width: 100%; min-width: 0; flex-direction: column; align-items: center; gap: 7px; }
  .seat-left .rack-line, .seat-right .rack-line { flex-direction: column; align-items: center; }
  .seat-header { display: flex; width: 100%; min-width: 0; align-items: center; gap: 7px; }
  .seat-name { display: flex; min-width: 0; flex: 1; flex-direction: column; }
  .seat-name span { color: var(--seat-color, var(--text-tertiary)); font-family: "JetBrains Mono", monospace; font-size: 8px; line-height: 11px; text-transform: uppercase; }
  .seat-name strong { overflow: hidden; color: var(--text-primary); font-size: 11px; font-weight: 600; line-height: 15px; text-overflow: ellipsis; white-space: nowrap; }
  .seat-score { color: var(--text-secondary); font-family: "JetBrains Mono", monospace; font-size: 10px; }
  .rack-line { display: flex; max-width: 100%; align-items: flex-end; justify-content: center; gap: 7px; }
  .center-console { position: absolute; z-index: 3; top: 49%; left: 50%; width: 250px; border: 1px solid color-mix(in srgb, var(--gold) 28%, var(--border-default)); border-radius: 6px; background: color-mix(in srgb, var(--surface-1) 92%, transparent); box-shadow: inset 0 0 0 2px rgba(242, 234, 217, 0.025), 0 10px 25px rgba(0, 0, 0, 0.34); padding: 11px; transform: translate(-50%, -50%); }
  .round-plaque { display: flex; align-items: flex-end; justify-content: space-between; gap: 8px; border-bottom: 1px solid color-mix(in srgb, var(--gold) 30%, transparent); padding-bottom: 8px; }
  .round-plaque div { display: flex; align-items: baseline; gap: 7px; }
  .round-plaque strong { color: var(--gold); font-family: "Shippori Mincho", serif; font-size: 20px; line-height: 1; }
  .round-plaque span { color: var(--text-secondary); font-family: "JetBrains Mono", monospace; font-size: 9px; }
  .round-plaque small { color: var(--text-tertiary); font-size: 8px; }
  .center-console :global(.wall-counter) { margin-top: 8px; }
  .riichi-row { display: flex; min-height: 26px; align-items: center; justify-content: space-between; gap: 8px; }
  .riichi-row > span { color: var(--text-tertiary); font-family: "JetBrains Mono", monospace; font-size: 8px; text-transform: uppercase; }
  .sticks { display: flex; min-height: 7px; align-items: center; gap: 3px; }
  .sticks i { position: relative; display: block; width: 28px; height: 4px; border-radius: 2px; background: var(--kinari); box-shadow: 0 1px 2px rgba(0, 0, 0, 0.4); }
  .sticks b { position: absolute; top: 1px; left: 50%; width: 3px; height: 3px; border-radius: 50%; background: var(--shu); translate: -50% -1px; }
  .sticks small { color: var(--text-tertiary); font-family: "JetBrains Mono", monospace; font-size: 9px; }
  .score-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 4px; border-top: 1px solid color-mix(in srgb, var(--kinari) 8%, transparent); padding-top: 7px; }
  .win-tag { position: absolute; z-index: 5; right: 6px; bottom: 58px; display: grid; max-width: 225px; grid-template-columns: auto auto; gap: 1px 8px; border-top: 2px solid var(--gold); background: var(--kinari); color: var(--tile-text); box-shadow: 0 6px 18px rgba(0, 0, 0, 0.38); padding: 7px 9px; animation: tag-in var(--motion-slow) ease-out; }
  .win-tag strong { font-family: "Shippori Mincho", serif; font-size: 13px; }
  .win-tag > span:not(.tag-knot) { font-family: "JetBrains Mono", monospace; font-size: 10px; text-align: right; }
  .win-tag small { grid-column: 1 / -1; color: color-mix(in srgb, var(--tile-text) 68%, transparent); font-size: 8px; line-height: 12px; }
  .tag-knot { position: absolute; top: -7px; left: 12px; width: 1px; height: 7px; background: var(--shu); }
  .compact-view > .center-console { position: static; width: 100%; transform: none; }
  .compact-seats { display: grid; gap: 10px; margin-top: 10px; }
  .compact-seat { position: relative; overflow: hidden; border-block: 1px solid var(--border-subtle); padding: 10px 0 13px; }
  .compact-seat .seat-header { padding-inline: 5px; }
  .pond-scroll, .rack-scroll { overflow-x: auto; padding: 8px 5px 6px; }
  .compact-seat .rack-line { width: max-content; justify-content: flex-start; }
  .compact-seat .win-tag { position: relative; right: auto; bottom: auto; margin: 8px 5px 0; }
  @keyframes tag-in { from { opacity: 0; translate: 0 5px; } }
  @media (max-width: 999px) { .table-view { margin-inline: -20px; overflow-x: auto; padding: 0 20px 24px; } }
  @media (max-width: 639px) { .table-view { display: none; } .compact-view { display: block; } }
  .forced-compact .table-view { display: none; }
  .forced-compact .compact-view { display: block; }
</style>
