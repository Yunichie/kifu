<script lang="ts">
  import Users from 'lucide-svelte/icons/users';
  import CareerView from '$lib/components/CareerView.svelte';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>Career | Kifu</title>
</svelte:head>

{#if data.career}
  <CareerView
    career={data.career}
    title="Your combined record"
    emptyHref={data.career.playerNames.length === 0 ? '/account' : '/'}
    emptyLabel={data.career.playerNames.length === 0 ? 'Claim a Tenhou name' : 'Add a matching log'}
  />
{:else}
  <header class="mb-6">
    <p class="section-kicker">Career</p>
    <h1 class="mt-1 font-display text-xl leading-[26px] font-semibold">Browse player records</h1>
  </header>
  <section class="panel">
    <div class="mb-4 flex items-center gap-2">
      <Users size={18} strokeWidth={1.75} class="text-text-secondary" aria-hidden="true" />
      <h2 class="text-[15px] leading-[21px] font-semibold">Players in the ledger</h2>
    </div>
    {#if data.players.length > 0}
      <div class="flex flex-wrap gap-2">
        {#each data.players as player (player)}
          <a
            class="stamp-tag transition-colors duration-fast hover:border-gold hover:text-gold"
            href={`/career/${encodeURIComponent(player)}`}>{player}</a
          >
        {/each}
      </div>
    {:else}
      <p class="text-[13px] leading-[19px] text-text-tertiary">Players appear after a log is added.</p>
    {/if}
  </section>
{/if}
