<script lang="ts">
  let { label, value, tone = 'neutral' }: { label: string; value: string | number; tone?: 'neutral' | 'win' | 'loss' | 'gold' } = $props();
</script>

<div class={['stat-card', `tone-${tone}`]}>
  <div class="value">{value}</div>
  <div class="label">{label}</div>
</div>

<style>
  .stat-card { position: relative; border-block: 1px solid var(--border-subtle); padding: 14px 10px 12px; text-align: center; }
  .stat-card::before { position: absolute; top: -1px; left: 50%; width: 28px; height: 2px; background: var(--border-strong); content: ''; translate: -50%; }
  .value { color: var(--text-primary); font-family: "JetBrains Mono", monospace; font-size: 25px; font-weight: 600; line-height: 1; }
  .label { margin-top: 8px; color: var(--text-tertiary); font-size: 10px; font-weight: 600; line-height: 14px; text-transform: uppercase; }
  .tone-win::before { background: var(--take); }
  .tone-loss::before { background: var(--shu); }
  .tone-gold::before { background: var(--gold); }
  .tone-win .value { color: var(--take); }
  .tone-loss .value { color: var(--shu); }
</style>
