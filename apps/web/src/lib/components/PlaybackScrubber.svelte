<script lang="ts">
  import type { TurnEvent } from '@kifu/api-types';
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';
  import ChevronRight from 'lucide-svelte/icons/chevron-right';
  import Pause from 'lucide-svelte/icons/pause';
  import Play from 'lucide-svelte/icons/play';
  import { tileLabel } from '$lib/replay/tiles';

  let { turn, maxTurn, event, settled, onTurnChange }: {
    turn: number;
    maxTurn: number;
    event: TurnEvent | null;
    settled: boolean;
    onTurnChange: (turn: number) => void;
  } = $props();

  let playing = $state(false);
  let description = $derived(settled ? 'Hand settled' : describeEvent(event));
  let progress = $derived(maxTurn === 0 ? '0%' : `${(turn / maxTurn) * 100}%`);

  $effect(() => {
    if (!playing) return;
    const timer = window.setInterval(() => {
      if (turn >= maxTurn) {
        playing = false;
        return;
      }
      onTurnChange(turn + 1);
    }, 650);
    return () => window.clearInterval(timer);
  });

  function setTurn(next: number): void {
    playing = false;
    onTurnChange(Math.max(0, Math.min(maxTurn, next)));
  }

  function togglePlayback(): void {
    if (!playing && turn >= maxTurn) onTurnChange(0);
    playing = !playing;
  }
</script>

<div class="playback">
  <input
    class="timeline"
    style:--progress={progress}
    type="range"
    min="0"
    max={maxTurn}
    value={turn}
    aria-label="Replay turn"
    oninput={(inputEvent) => setTurn(Number(inputEvent.currentTarget.value))}
  />
  <div class="controls">
    <div class="buttons">
      <button type="button" onclick={() => setTurn(turn - 1)} disabled={turn === 0} aria-label="Previous turn" title="Previous turn">
        <ChevronLeft size={18} strokeWidth={1.75} aria-hidden="true" />
      </button>
      <button class="play" type="button" onclick={togglePlayback} disabled={maxTurn === 0} aria-label={playing ? 'Pause replay' : 'Play replay'} title={playing ? 'Pause replay' : 'Play replay'}>
        {#if playing}<Pause size={18} strokeWidth={1.75} aria-hidden="true" />{:else}<Play size={18} strokeWidth={1.75} aria-hidden="true" />{/if}
      </button>
      <button type="button" onclick={() => setTurn(turn + 1)} disabled={turn === maxTurn} aria-label="Next turn" title="Next turn">
        <ChevronRight size={18} strokeWidth={1.75} aria-hidden="true" />
      </button>
    </div>
    <div class="event" aria-live="polite">
      <span class="turn-stamp">Turn {turn} / {maxTurn}</span>
      <span>{description}</span>
    </div>
  </div>
</div>

<script lang="ts" module>
  function describeEvent(event: TurnEvent | null): string {
    if (!event) return 'Ready';
    switch (event.type) {
      case 'Draw': return `S${event.seat + 1} draws ${tileLabel(event.tile)}`;
      case 'Discard': return `S${event.seat + 1} discards ${tileLabel(event.tile)}${event.tsumogiri ? ' (tsumogiri)' : ''}`;
      case 'Call': return `S${event.seat + 1} ${event.kind}`;
      case 'Riichi': return `S${event.seat + 1} declares riichi`;
      case 'NewDora': return `Dora indicator ${tileLabel(event.tile)}`;
    }
  }
</script>

<style>
  .playback { border-top: 1px solid var(--border-subtle); padding-top: 16px; }
  .timeline { width: 100%; height: 18px; cursor: pointer; appearance: none; background: transparent; }
  .timeline::-webkit-slider-runnable-track { height: 4px; border-radius: 0; background: linear-gradient(90deg, var(--gold) var(--progress), var(--border-default) var(--progress)); box-shadow: 0 -5px 0 -1px var(--surface-0), 0 5px 0 -1px var(--surface-0); }
  .timeline::-moz-range-track { height: 4px; background: var(--border-default); }
  .timeline::-moz-range-progress { height: 4px; background: var(--gold); }
  .timeline::-webkit-slider-thumb { width: 14px; height: 14px; margin-top: -5px; appearance: none; border: 2px solid var(--gold-ink); border-radius: 50%; background: var(--gold); box-shadow: 0 0 0 2px var(--gold); }
  .timeline::-moz-range-thumb { width: 12px; height: 12px; border: 2px solid var(--gold-ink); border-radius: 50%; background: var(--gold); }
  .controls { display: flex; align-items: center; justify-content: space-between; gap: 16px; margin-top: 7px; }
  .buttons { display: flex; align-items: center; gap: 4px; }
  button { display: inline-flex; width: 36px; height: 36px; align-items: center; justify-content: center; border-radius: 50%; color: var(--text-secondary); transition: color var(--motion-fast), background var(--motion-fast), border-color var(--motion-fast); }
  button:hover:not(:disabled) { background: var(--surface-3); color: var(--text-primary); }
  button:disabled { opacity: 0.3; }
  .play { width: 42px; height: 42px; border: 1px solid var(--gold); color: var(--gold); box-shadow: inset 0 0 0 2px var(--surface-0); }
  .event { display: flex; min-width: 0; align-items: center; gap: 10px; color: var(--text-secondary); font-family: "JetBrains Mono", monospace; font-size: 11px; }
  .turn-stamp { flex: 0 0 auto; border: 1px solid var(--gold); border-radius: 3px; padding: 3px 7px; color: var(--gold); font-weight: 600; }
  @media (max-width: 639px) { .controls { flex-direction: column; } .event { flex-direction: column; gap: 5px; text-align: center; } }
</style>
