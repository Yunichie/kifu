<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import LogIn from 'lucide-svelte/icons/log-in';
  import Plus from 'lucide-svelte/icons/plus';
  import Users from 'lucide-svelte/icons/users';
  import GameList from '$lib/components/GameList.svelte';
  import type { PageProps } from './$types';

  let { data, form }: PageProps = $props();
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
        <a class="nav-link px-3" href="/" aria-current={data.view === 'library' ? 'page' : undefined}
          >Library</a
        >
      {/if}
      <a
        class="nav-link px-3"
        href="/?view=all"
        aria-current={data.view === 'all' ? 'page' : undefined}>All games</a
      >
    </nav>
  </div>

  {#if data.games.length > 0}
    <GameList games={data.games} removable={data.view === 'library'} />
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
  {#if data.players.length > 0}
    <div class="flex flex-wrap gap-2">
      {#each data.players as player (player)}
        <a
          class="stamp-tag gap-1.5 transition-colors duration-fast hover:border-gold hover:text-gold"
          href={`/career/${encodeURIComponent(player)}`}><span class="suit-bullet" lang="ja">萬</span>{player}</a
        >
      {/each}
    </div>
  {:else}
    <p class="text-[13px] leading-[19px] text-text-tertiary">Players appear after a log is added.</p>
  {/if}
</section>

<style>
  .empty-tile { display: grid; width: 30px; height: 42px; place-items: center; border: 2px solid color-mix(in srgb, var(--kinari) 42%, transparent); border-radius: 4px; background: var(--surface-3); opacity: 0.48; transform: rotate(-5deg); }
  .empty-tile i { width: 14px; height: 14px; border: 1px solid var(--gold); border-radius: 50%; }
  .suit-bullet { color: var(--shu); font-family: "Shippori Mincho", serif; font-size: 12px; }
</style>
