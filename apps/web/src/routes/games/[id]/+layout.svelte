<script lang="ts">
  import { page } from '$app/state';
  import ArrowLeft from 'lucide-svelte/icons/arrow-left';
  import BarChart3 from 'lucide-svelte/icons/bar-chart-3';
  import MonitorPlay from 'lucide-svelte/icons/monitor-play';
  import UserRound from 'lucide-svelte/icons/user-round';
  import { rulesLabel } from '$lib/format';
  import type { LayoutProps } from './$types';

  let { data, children }: LayoutProps = $props();
  let ownerQuery = $derived(data.publicOwner ? `?owner=${data.publicOwner.userId}` : '');
  let summaryHref = $derived(`/games/${data.game.logId}${ownerQuery}`);
  let tableHref = $derived(`/games/${data.game.logId}/table${ownerQuery}`);
</script>

<header class="mb-6">
  <a class="back-link" href={data.publicOwner ? '/?view=all' : '/'}>
    <ArrowLeft size={16} strokeWidth={1.75} aria-hidden="true" />
    Game ledger
  </a>
  <div class="mt-4 flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
    <div>
      <p class="section-kicker">Game record</p>
      <h1 class="mt-1 font-display text-[28px] leading-[34px] font-bold">{rulesLabel(data.game.rules)}</h1>
      <div class="mt-3 flex flex-wrap gap-2">
        {#if data.game.rules.akaDora}<span class="stamp-tag text-gold">Aka dora</span>{/if}
        {#if data.game.rules.kuitan}<span class="stamp-tag">Kuitan</span>{/if}
        {#if data.game.rules.fast}<span class="stamp-tag">Fast</span>{/if}
      </div>
      {#if data.publicOwner}
        <p class="owner-line mt-3">
          <UserRound size={15} strokeWidth={1.75} aria-hidden="true" />
          Shared by <strong>{data.publicOwner.name}</strong>
        </p>
      {/if}
    </div>
    <code class="log-id">{data.game.logId}</code>
  </div>
  <nav class="mt-6 flex border-b border-border-subtle" aria-label="Game views">
    <a class="nav-link min-h-11" href={summaryHref} aria-current={page.url.pathname === summaryHref ? 'page' : undefined}>
      <BarChart3 size={16} strokeWidth={1.75} aria-hidden="true" /> Summary
    </a>
    <a class="nav-link min-h-11" href={tableHref} aria-current={page.url.pathname === tableHref ? 'page' : undefined}>
      <MonitorPlay size={16} strokeWidth={1.75} aria-hidden="true" /> Table replay
    </a>
  </nav>
</header>

{@render children()}

<style>
  .back-link { display: inline-flex; min-height: 44px; align-items: center; gap: 6px; color: var(--text-secondary); font-size: 13px; font-weight: 700; transition: color var(--motion-fast); }
  .back-link:hover { color: var(--text-primary); }
  .log-id { max-width: 100%; overflow: hidden; border: 1px solid var(--border-subtle); border-radius: 4px; background: var(--surface-2); padding: 7px 9px; color: var(--text-secondary); font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-size: 11px; line-height: 18px; text-overflow: ellipsis; white-space: nowrap; }
  .owner-line { display: flex; align-items: center; gap: 5px; color: var(--text-secondary); font-size: 12px; line-height: 18px; }
  .owner-line strong { color: var(--text-primary); }
  @media (max-width: 639px) { .log-id { width: 100%; } }
</style>
