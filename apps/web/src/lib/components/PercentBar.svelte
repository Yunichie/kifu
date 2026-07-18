<script lang="ts">
  let { label, value, maximum = 100, display = null, tone = 'gold' }: {
    label: string;
    value: number;
    maximum?: number;
    display?: string | null;
    tone?: 'gold' | 'win' | 'loss' | 'ai';
  } = $props();
  let percent = $derived(Math.min(100, Math.max(0, (value / Math.max(1, maximum)) * 100)));
  let width = $derived(`${percent}%`);
</script>

<div class="percent-bar">
  <span class="label" title={label}>{label}</span>
  <div
    class="track"
    role="meter"
    aria-label={label}
    aria-valuemin="0"
    aria-valuemax={maximum}
    aria-valuenow={value}
  >
    <span class={['fill', `tone-${tone}`]} style:--width={width}></span>
  </div>
  <span class="value">{display ?? value}</span>
</div>

<style>
  .percent-bar {
    display: grid;
    grid-template-columns: minmax(5.5rem, 0.8fr) minmax(6rem, 1.6fr) minmax(2.5rem, auto);
    align-items: center;
    gap: 10px;
    min-height: 28px;
    font-size: 12px;
    line-height: 18px;
  }

  .label {
    overflow: hidden;
    color: var(--text-secondary);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .track {
    position: relative;
    height: 10px;
    border-left: 1px solid var(--border-strong);
    background-color: var(--surface-2);
    background-image: linear-gradient(90deg, transparent 24.5%, var(--border-subtle) 25%, transparent 25.5%, transparent 49.5%, var(--border-subtle) 50%, transparent 50.5%, transparent 74.5%, var(--border-subtle) 75%, transparent 75.5%);
  }

  .fill {
    display: block;
    width: var(--width);
    height: 100%;
    background: var(--gold);
    transform-origin: left;
    transition: width var(--motion-base) ease-out;
  }

  .tone-win { background: var(--sou); }
  .tone-loss { background: var(--man); }
  .tone-ai { background: var(--pin); }
  .value {
    color: var(--text-primary);
    font-family: "Roboto Mono", "BIZ UDPGothic", monospace;
    font-variant-numeric: tabular-nums lining-nums;
    font-weight: 600;
    text-align: right;
  }

  @media (max-width: 420px) {
    .percent-bar { grid-template-columns: minmax(4.75rem, 0.8fr) minmax(4rem, 1fr) auto; gap: 8px; }
  }
</style>
