<script lang="ts">
  import { page } from '$app/state';
  import ArrowLeft from 'lucide-svelte/icons/arrow-left';
  import { rulesLabel } from '$lib/format';
  import type { LayoutProps } from './$types';

  let { data, children }: LayoutProps = $props();
  let summaryHref = $derived(`/games/${data.game.logId}`);
  let tableHref = $derived(`${summaryHref}/table`);
</script>

<header class="mb-6">
  <a class="mb-3 inline-flex items-center gap-1.5 text-[13px] leading-[19px] text-text-secondary hover:text-text-primary" href="/">
    <ArrowLeft size={16} strokeWidth={1.75} aria-hidden="true" />
    Game ledger
  </a>
  <div class="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
    <div>
      <p class="text-[11px] leading-[15px] font-medium uppercase text-text-tertiary">This Game</p>
      <h1 class="mt-1 font-display text-xl leading-[26px] font-semibold">{rulesLabel(data.game.rules)}</h1>
    </div>
    <code class="break-all font-mono text-[13px] leading-[19px] text-text-secondary">{data.game.logId}</code>
  </div>
  <div class="mt-3 flex flex-wrap gap-2">
    {#if data.game.rules.akaDora}<span class="rounded-sm border border-gold px-2 py-0.5 text-[11px] text-gold">Aka dora</span>{/if}
    {#if data.game.rules.kuitan}<span class="rounded-sm bg-surface-3 px-2 py-0.5 text-[11px] text-text-secondary">Kuitan</span>{/if}
    {#if data.game.rules.fast}<span class="rounded-sm bg-surface-3 px-2 py-0.5 text-[11px] text-text-secondary">Fast</span>{/if}
  </div>
  <nav class="mt-5 flex border-b border-border-subtle" aria-label="Game views">
    <a class="nav-link min-h-10" href={summaryHref} aria-current={page.url.pathname === summaryHref ? 'page' : undefined}>Summary</a>
    <a class="nav-link min-h-10" href={tableHref} aria-current={page.url.pathname === tableHref ? 'page' : undefined}>Table replay</a>
  </nav>
</header>

{@render children()}
