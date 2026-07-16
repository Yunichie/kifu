<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import Plus from 'lucide-svelte/icons/plus';
  import X from 'lucide-svelte/icons/x';
  import type { PageProps } from './$types';

  let { data, form }: PageProps = $props();
</script>

<svelte:head>
  <title>Settings | Kifu</title>
</svelte:head>

<div class="max-w-2xl">
  <header class="mb-6">
    <p class="section-kicker">Settings</p>
    <h1 class="mt-1 font-display text-xl leading-[26px] font-semibold">Your Tenhou names</h1>
    <p class="mt-2 max-w-xl text-[13px] leading-[19px] text-text-tertiary">
      We can't verify Tenhou account ownership. This just tells the app which name is yours when
      combining stats across games.
    </p>
  </header>

  <section class="panel">
    {#if form?.message}
      <div class="status-error mb-4" role="alert">
        <CircleAlert class="mt-0.5 shrink-0" size={16} strokeWidth={1.75} aria-hidden="true" />
        <span>{form.message}</span>
      </div>
    {/if}

    {#if data.me.playerNames.length > 0}
      <div class="mb-5 flex flex-wrap gap-2">
        {#each data.me.playerNames as name (name)}
          <form
            class="stamp-tag inline-flex items-center pl-2.5 text-text-secondary"
            method="POST"
            action="?/removeName"
          >
            <span>{name}</span>
            <input type="hidden" name="name" value={name} />
            <button
              class="ml-1 inline-flex size-8 items-center justify-center text-text-tertiary transition-colors duration-fast hover:text-man"
              type="submit"
              aria-label={`Remove ${name}`}
              title={`Remove ${name}`}
            >
              <X size={14} strokeWidth={1.75} aria-hidden="true" />
            </button>
          </form>
        {/each}
      </div>
    {:else}
      <p class="mb-5 text-[13px] leading-[19px] text-text-tertiary">No Tenhou names claimed.</p>
    {/if}

    <form class="flex flex-col gap-3 sm:flex-row" method="POST" action="?/addName">
      <label class="min-w-0 flex-1">
        <span class="field-label">Tenhou name</span>
        <input
          class="field"
          name="name"
          value={form?.name ?? ''}
          maxlength="64"
          autocomplete="off"
          required
        />
      </label>
      <button class="button-primary self-end" type="submit">
        <Plus size={16} strokeWidth={1.75} aria-hidden="true" />
        Add
      </button>
    </form>
  </section>
</div>
