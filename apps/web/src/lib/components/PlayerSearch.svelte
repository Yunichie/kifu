<script lang="ts">
  import type { PlayerSearchPage } from '@kifu/api-types';
  import Search from 'lucide-svelte/icons/search';
  import Pagination from './Pagination.svelte';

  let {
    query,
    results,
    basePath,
    pageParam = 'page',
    preservedParams = {}
  }: {
    query: string;
    results: PlayerSearchPage;
    basePath: string;
    pageParam?: string;
    preservedParams?: Record<string, string | number>;
  } = $props();

  function pageHref(page: number): string {
    const params = new URLSearchParams();
    for (const [name, value] of Object.entries(preservedParams)) {
      if (String(value)) params.set(name, String(value));
    }
    params.set('q', query);
    if (page > 1) params.set(pageParam, String(page));
    return `${basePath}?${params}`;
  }
</script>

<form class="flex flex-col gap-3 sm:flex-row" method="GET" action={basePath}>
  {#each Object.entries(preservedParams) as [name, value] (name)}
    <input type="hidden" {name} {value} />
  {/each}
  <label class="min-w-0 flex-1">
    <span class="field-label">Player name</span>
    <input
      class="field"
      type="search"
      name="q"
      value={query}
      maxlength="64"
      autocomplete="off"
      placeholder="Search players"
    />
  </label>
  <button class="button-secondary self-end" type="submit">
    <Search size={16} strokeWidth={1.75} aria-hidden="true" />
    Search
  </button>
</form>

{#if query}
  {#if results.items.length > 0}
    <div class="mt-5 grid grid-cols-2 gap-2 sm:flex sm:flex-wrap">
      {#each results.items as player (player)}
        <a
          class="inline-flex min-h-11 min-w-0 items-center justify-between gap-2 rounded-md border border-border-subtle bg-surface-2 px-3 text-[12px] font-bold text-text-primary transition-colors duration-fast hover:border-gold"
          href={`/career/${encodeURIComponent(player)}`}>{player}</a
        >
      {/each}
    </div>
    <Pagination
      page={results.page}
      hasNext={results.hasNext}
      previousHref={pageHref(results.page - 1)}
      nextHref={pageHref(results.page + 1)}
      label="Player search pages"
    />
  {:else}
    <p class="mt-5 text-[13px] leading-[19px] text-text-tertiary">No players found.</p>
  {/if}
{/if}
