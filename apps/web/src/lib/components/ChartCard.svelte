<script lang="ts">
  import type { Snippet } from 'svelte';

  let { title, children }: { title: string; children: Snippet } = $props();
</script>

<section class="chart-card">
  <header><h2>{title}</h2><span aria-hidden="true"></span></header>
  <div class="chart-body">{@render children()}</div>
</section>

<style>
  .chart-card {
    overflow: hidden;
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    background: var(--surface-1);
    box-shadow: 0 14px 28px -26px var(--shadow-strong);
  }

  header {
    display: flex;
    min-height: 48px;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    border-bottom: 1px solid var(--border-subtle);
    background: var(--surface-2);
    padding: 10px 16px;
  }

  h2 {
    color: var(--text-primary);
    font-family: "Zen Old Mincho", "Noto Serif JP", serif;
    font-size: 16px;
    font-weight: 700;
    line-height: 24px;
  }

  header span {
    width: 20px;
    height: 3px;
    flex: 0 0 auto;
    border-radius: 2px;
    background: var(--gold);
  }

  .chart-body { padding: 16px; }

  @media (max-width: 639px) {
    .chart-body { padding: 14px; }
  }
</style>
