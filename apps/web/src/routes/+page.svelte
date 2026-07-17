<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import LogIn from 'lucide-svelte/icons/log-in';
  import Plus from 'lucide-svelte/icons/plus';
  import Users from 'lucide-svelte/icons/users';
  import GameList from '$lib/components/GameList.svelte';
  import Pagination from '$lib/components/Pagination.svelte';
  import PlayerSearch from '$lib/components/PlayerSearch.svelte';
  import type { PageProps } from './$types';

  let { data, form }: PageProps = $props();

  function homeHref(gamePage: number, playerPage: number, view = data.view): string {
    const params = new URLSearchParams();
    if (view === 'all') params.set('view', 'all');
    if (gamePage > 1) params.set('page', String(gamePage));
    if (data.query) params.set('q', data.query);
    if (playerPage > 1) params.set('playerPage', String(playerPage));
    const query = params.toString();
    return query ? `/?${query}` : '/';
  }
</script>

<svelte:head>
  <title>Logs | Kifu</title>
  <meta name="description" content="Tenhou game records and riichi career statistics" />
</svelte:head>

<section id="add-log" class="panel mb-8">
  <h1 class="mb-3 text-[15px] leading-[21px] font-semibold">Add a Tenhou log</h1>
  {#if form?.message}
    <div class="status-error mb-4" role="alert">
      <CircleAlert class="mt-0.5 shrink-0" size={16} strokeWidth={1.75} aria-hidden="true" />
      <span>{form.message}</span>
    </div>
  {/if}
  <form class="flex flex-col gap-3 sm:flex-row" method="POST" action="?/add">
    <label class="min-w-0 flex-1">
      <span class="field-label">Tenhou log ID or URL</span>
      <input
        class="field font-mono"
        name="logId"
        value={form?.logId ?? ''}
        placeholder="2017010100gm-00a9-0000-003dbd5d"
        autocomplete="off"
        required
      />
    </label>
    {#if data.me}
      <button class="button-primary self-end" type="submit">
        <Plus size={16} strokeWidth={1.75} aria-hidden="true" />
        Parse & save
      </button>
    {:else}
      <a class="button-primary self-end" href="/login">
        <LogIn size={16} strokeWidth={1.75} aria-hidden="true" />
        Log in to add a log
      </a>
    {/if}
  </form>
</section>

<section class="mb-8">
  <div class="mb-4 flex items-end justify-between gap-4 border-b border-border-subtle">
    <h2 class="pb-3 font-display text-xl leading-[26px] font-semibold">Game ledger</h2>
    <nav class="flex self-stretch" aria-label="Game list">
      {#if data.me}
        <a
          class="nav-link px-3"
          href={homeHref(1, data.playerResults.page, 'library')}
          aria-current={data.view === 'library' ? 'page' : undefined}
          >Library</a
        >
      {/if}
      <a
        class="nav-link px-3"
        href={homeHref(1, data.playerResults.page, 'all')}
        aria-current={data.view === 'all' ? 'page' : undefined}>All games</a
      >
    </nav>
  </div>

  {#if data.games.items.length > 0}
    <GameList games={data.games.items} removable={data.view === 'library'} />
    <Pagination
      page={data.games.page}
      hasNext={data.games.hasNext}
      previousHref={homeHref(data.games.page - 1, data.playerResults.page)}
      nextHref={homeHref(data.games.page + 1, data.playerResults.page)}
      label="Game ledger pages"
    />
  {:else}
    <div class="flex min-h-40 flex-col items-center justify-center gap-3 text-center text-text-tertiary">
      <span class="empty-tile" aria-hidden="true"><i></i></span>
      <p class="text-[13px] leading-[19px]">
        {data.view === 'library' ? 'No logs saved yet.' : 'No logs added yet.'}
      </p>
      {#if data.me}
        <a class="button-primary" href="#add-log">Add your first log</a>
      {:else}
        <a class="button-primary" href="/login">Log in to add a log</a>
      {/if}
    </div>
  {/if}
</section>

<section>
  <div class="mb-4 flex items-center gap-2">
    <Users size={18} strokeWidth={1.75} class="text-gold" aria-hidden="true" />
    <h2 class="font-display text-xl leading-[26px] font-semibold">Players</h2>
  </div>
  <PlayerSearch
    query={data.query}
    results={data.playerResults}
    basePath="/"
    pageParam="playerPage"
    preservedParams={{ view: data.view, page: data.games.page }}
  />
</section>

<style>
  .empty-tile { display: grid; width: 30px; height: 42px; place-items: center; border: 2px solid color-mix(in srgb, var(--kinari) 42%, transparent); border-radius: 4px; background: var(--surface-3); opacity: 0.48; transform: rotate(-5deg); }
  .empty-tile i { width: 14px; height: 14px; border: 1px solid var(--gold); border-radius: 50%; }
</style>
