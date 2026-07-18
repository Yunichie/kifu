<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import FilePlus2 from 'lucide-svelte/icons/file-plus-2';
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

<header class="mb-5 sm:mb-6">
  <p class="section-kicker">Tenhou records</p>
  <h1 class="mt-1 font-display text-[28px] leading-[34px] font-bold">Game ledger</h1>
</header>

<section id="add-log" class="add-log panel mb-8">
  <div class="mb-4 flex items-center gap-3">
    <span class="add-icon" aria-hidden="true"><FilePlus2 size={20} strokeWidth={1.6} /></span>
    <h2 class="font-display text-[18px] leading-6 font-bold">Add a Tenhou log</h2>
  </div>
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
        Log in to add
      </a>
    {/if}
  </form>
</section>

<section class="mb-10">
  <div class="mb-4 flex items-end justify-between gap-3 border-b border-border-subtle">
    <h2 class="pb-3 font-display text-[20px] leading-7 font-bold">Saved games</h2>
    <nav class="flex min-w-0 self-stretch" aria-label="Game list">
      {#if data.me}
        <a
          class="nav-link px-2.5 sm:px-3.5"
          href={homeHref(1, data.playerResults.page, 'library')}
          aria-current={data.view === 'library' ? 'page' : undefined}
        >Library</a>
      {/if}
      <a
        class="nav-link px-2.5 sm:px-3.5"
        href={homeHref(1, data.playerResults.page, 'all')}
        aria-current={data.view === 'all' ? 'page' : undefined}
      >All games</a>
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
    <div class="empty-state">
      <span class="empty-plaque" aria-hidden="true"><i></i></span>
      <p>{data.view === 'library' ? 'No logs saved yet.' : 'No logs added yet.'}</p>
      {#if data.me}
        <a class="button-primary" href="#add-log">Add your first log</a>
      {:else}
        <a class="button-primary" href="/login">Log in to add a log</a>
      {/if}
    </div>
  {/if}
</section>

<section class="player-search panel">
  <div class="mb-4 flex items-center gap-3">
    <span class="search-icon" aria-hidden="true"><Users size={18} strokeWidth={1.75} /></span>
    <h2 class="font-display text-[20px] leading-7 font-bold">Players</h2>
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
  .add-log { position: relative; overflow: hidden; }
  .add-log::before { position: absolute; top: 0; bottom: 0; left: 0; width: 3px; background: var(--gold); content: ''; }
  .add-icon, .search-icon { display: grid; width: 36px; height: 36px; flex: 0 0 auto; place-items: center; border-radius: 6px; background: var(--surface-2); color: var(--gold); }
  .search-icon { color: var(--pin); }
  .empty-state { display: flex; min-height: 220px; flex-direction: column; align-items: center; justify-content: center; gap: 14px; border: 1px dashed var(--border-default); border-radius: 6px; color: var(--text-tertiary); text-align: center; }
  .empty-plaque { display: grid; width: 34px; height: 48px; place-items: center; border: 1px solid var(--tile-edge); border-radius: 5px; background: linear-gradient(var(--tile-highlight), var(--tile-bg)); box-shadow: 0 3px 0 var(--tile-edge); transform: rotate(-3deg); }
  .empty-plaque i { width: 13px; height: 3px; border-radius: 2px; background: var(--gold); }
</style>
