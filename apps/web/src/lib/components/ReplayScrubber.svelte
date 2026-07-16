<script lang="ts">
  import type { TurnEvent } from '@kifu/api-types';
  import { ChevronLeft, ChevronRight, Pause, Play } from 'lucide-svelte';
  import { tileLabel } from '$lib/replay/tiles';

  let {
    turn,
    maxTurn,
    event,
    settled,
    onTurnChange
  }: {
    turn: number;
    maxTurn: number;
    event: TurnEvent | null;
    settled: boolean;
    onTurnChange: (turn: number) => void;
  } = $props();

  let playing = $state(false);
  let description = $derived(settled ? 'Hand settled' : describeEvent(event));

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

<div class="border-t border-border-subtle pt-4">
  <input
    class="h-4 w-full cursor-pointer accent-gold"
    type="range"
    min="0"
    max={maxTurn}
    value={turn}
    aria-label="Replay turn"
    oninput={(event) => setTurn(Number(event.currentTarget.value))}
  />
  <div class="mt-2 flex flex-col items-center justify-between gap-3 sm:flex-row">
    <div class="flex items-center gap-1">
      <button
        class="inline-flex size-9 items-center justify-center rounded-md text-text-secondary transition-colors duration-fast hover:bg-surface-3 hover:text-text-primary disabled:opacity-30"
        type="button"
        onclick={() => setTurn(turn - 1)}
        disabled={turn === 0}
        aria-label="Previous turn"
        title="Previous turn"
      ><ChevronLeft size={18} strokeWidth={1.75} aria-hidden="true" /></button>
      <button
        class="inline-flex size-10 items-center justify-center rounded-md border border-border text-text-primary transition-colors duration-fast hover:border-border-strong disabled:opacity-30"
        type="button"
        onclick={togglePlayback}
        disabled={maxTurn === 0}
        aria-label={playing ? 'Pause replay' : 'Play replay'}
        title={playing ? 'Pause replay' : 'Play replay'}
      >
        {#if playing}<Pause size={18} strokeWidth={1.75} aria-hidden="true" />{:else}<Play size={18} strokeWidth={1.75} aria-hidden="true" />{/if}
      </button>
      <button
        class="inline-flex size-9 items-center justify-center rounded-md text-text-secondary transition-colors duration-fast hover:bg-surface-3 hover:text-text-primary disabled:opacity-30"
        type="button"
        onclick={() => setTurn(turn + 1)}
        disabled={turn === maxTurn}
        aria-label="Next turn"
        title="Next turn"
      ><ChevronRight size={18} strokeWidth={1.75} aria-hidden="true" /></button>
    </div>
    <div class="min-w-0 text-center font-mono text-[13px] leading-[19px] text-text-secondary sm:text-right" aria-live="polite">
      <span class="text-text-primary">Turn {turn} / {maxTurn}</span> &mdash; {description}
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
