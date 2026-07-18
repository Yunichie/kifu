<script lang="ts">
  let { label, value, tone = 'neutral' }: { label: string; value: string | number; tone?: 'neutral' | 'win' | 'loss' | 'gold' } = $props();
</script>

<div class={['stat-card', `tone-${tone}`]}>
  <span class="tone-mark" aria-hidden="true"></span>
  <div class="value">{value}</div>
  <div class="label">{label}</div>
</div>

<style>
  .stat-card {
    position: relative;
    min-height: 104px;
    overflow: hidden;
    border: 1px solid var(--tile-edge);
    border-radius: 6px;
    background: linear-gradient(180deg, var(--tile-highlight), var(--tile-bg) 78%, var(--tile-bottom));
    color: var(--tile-text);
    padding: 18px 12px 14px;
    text-align: center;
    box-shadow:
      inset 0 1px 0 var(--tile-highlight),
      inset 0 -2px 0 color-mix(in srgb, var(--tile-edge) 50%, transparent),
      0 3px 0 color-mix(in srgb, var(--tile-edge) 70%, var(--surface-0)),
      0 12px 20px -18px var(--shadow-strong);
  }

  .tone-mark {
    position: absolute;
    top: 0;
    left: 50%;
    width: 36px;
    height: 3px;
    border-radius: 0 0 3px 3px;
    background: var(--border-strong);
    translate: -50%;
  }

  .value {
    overflow: hidden;
    font-family: "Roboto Mono", "BIZ UDPGothic", monospace;
    font-size: 30px;
    font-variant-numeric: tabular-nums lining-nums;
    font-weight: 700;
    line-height: 36px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .label {
    margin-top: 6px;
    color: color-mix(in srgb, var(--tile-text) 64%, transparent);
    font-size: 10px;
    font-weight: 700;
    line-height: 16px;
    text-transform: uppercase;
  }

  .tone-win .tone-mark { background: var(--sou); }
  .tone-loss .tone-mark { background: var(--man); }
  .tone-gold .tone-mark { background: var(--gold); }
</style>
