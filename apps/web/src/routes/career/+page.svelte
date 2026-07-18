<script lang="ts">
  import Users from 'lucide-svelte/icons/users';
  import CareerView from '$lib/components/CareerView.svelte';
  import PlayerSearch from '$lib/components/PlayerSearch.svelte';
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
    <p class="section-kicker">Career records</p>
    <h1 class="mt-1 font-display text-[28px] leading-[34px] font-bold">Browse players</h1>
  </header>
  <section class="panel">
    <div class="mb-4 flex items-center gap-3">
      <span class="grid size-9 place-items-center rounded-md bg-surface-2 text-pin"><Users size={18} strokeWidth={1.75} aria-hidden="true" /></span>
      <h2 class="font-display text-[18px] leading-6 font-bold">Players in the ledger</h2>
    </div>
    <PlayerSearch query={data.query} results={data.playerResults!} basePath="/career" />
  </section>
{/if}
