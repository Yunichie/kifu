<script lang="ts">
  import { formatScore } from '$lib/format';

  let { seat, name, score, active = false }: { seat: number; name: string; score: number; active?: boolean } = $props();
  const tones = ['var(--shu)', 'var(--ai)', 'var(--take)', 'var(--gold)'];
</script>

<div class:active class="score-plaque" style:--seat-color={tones[seat] ?? 'var(--text-secondary)'}>
  <span class="seat-mark"></span>
  <span class="name" title={name}>{name}</span>
  <strong>{formatScore(score)}</strong>
</div>

<style>
  .score-plaque { display: grid; min-width: 0; grid-template-columns: 6px minmax(0, 1fr); align-items: center; column-gap: 6px; border: 1px solid color-mix(in srgb, var(--kinari) 9%, transparent); border-radius: 3px; background: rgba(12, 17, 14, 0.42); padding: 4px 6px; }
  .seat-mark { width: 5px; height: 5px; border-radius: 50%; background: var(--seat-color); grid-row: span 2; }
  .name { overflow: hidden; color: var(--text-secondary); font-size: 9px; line-height: 12px; text-overflow: ellipsis; white-space: nowrap; }
  strong { color: var(--text-primary); font-family: "JetBrains Mono", monospace; font-size: 11px; line-height: 14px; }
  .active { border-color: color-mix(in srgb, var(--gold) 42%, transparent); }
</style>
