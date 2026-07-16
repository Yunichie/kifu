<script lang="ts">
  let { label, value, maximum = 100, display = null, tone = 'gold' }: {
    label: string;
    value: number;
    maximum?: number;
    display?: string | null;
    tone?: 'gold' | 'win' | 'loss' | 'ai';
  } = $props();
  let width = $derived(`${Math.min(100, Math.max(0, (value / Math.max(1, maximum)) * 100))}%`);
</script>

<div class="percent-bar">
  <span class="label" title={label}>{label}</span>
  <div class="track"><span class={['fill', `tone-${tone}`]} style:--width={width}></span></div>
  <span class="value">{display ?? value}</span>
</div>

<style>
  .percent-bar { display: grid; grid-template-columns: minmax(5rem, auto) 1fr minmax(2rem, auto); align-items: center; gap: 12px; font-size: 12px; line-height: 18px; }
  .label { overflow: hidden; color: var(--text-secondary); text-overflow: ellipsis; white-space: nowrap; }
  .track { height: 6px; overflow: hidden; background: var(--surface-3); }
  .fill { display: block; width: var(--width); height: 100%; background: var(--gold); transition: width var(--motion-base) ease-out; }
  .tone-win { background: var(--take); }
  .tone-loss { background: var(--shu); }
  .tone-ai { background: var(--ai); }
  .value { color: var(--text-primary); font-family: "JetBrains Mono", monospace; text-align: right; }
</style>
