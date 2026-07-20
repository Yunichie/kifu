<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import Plus from 'lucide-svelte/icons/plus';
  import UserRound from 'lucide-svelte/icons/user-round';
  import X from 'lucide-svelte/icons/x';
  import type { PageProps } from './$types';

  let { data, form }: PageProps = $props();
</script>

<svelte:head>
  <title>Settings | Kifu</title>
  <meta name="robots" content="noindex,follow" />
</svelte:head>

<div class="max-w-2xl">
  <header class="mb-6">
    <p class="section-kicker">Account settings</p>
    <h1 class="mt-1 font-display text-[28px] leading-[34px] font-bold">Your Tenhou names</h1>
    <p class="mt-2 max-w-xl text-[13px] leading-5 text-text-secondary">
      Names identify your seats when Kifu combines records across games.
    </p>
  </header>

  <section class="panel">
    {#if form?.message}
      <div class="status-error mb-4" role="alert">
        <CircleAlert class="mt-0.5 shrink-0" size={16} strokeWidth={1.75} aria-hidden="true" />
        <span>{form.message}</span>
      </div>
    {/if}

    <div class="mb-5">
      <h2 class="mb-3 text-[12px] leading-[18px] font-bold text-text-secondary">Claimed names</h2>
      {#if data.me.playerNames.length > 0}
        <div class="grid gap-2">
          {#each data.me.playerNames as name (name)}
            <form class="name-row" method="POST" action="?/removeName">
              <span class="name-avatar" aria-hidden="true"><UserRound size={16} strokeWidth={1.75} /></span>
              <strong title={name}>{name}</strong>
              <input type="hidden" name="name" value={name} />
              <button class="icon-button hover:text-man" type="submit" aria-label={`Remove ${name}`} title={`Remove ${name}`}>
                <X size={16} strokeWidth={1.75} aria-hidden="true" />
              </button>
            </form>
          {/each}
        </div>
      {:else}
        <div class="empty-name">No Tenhou names claimed.</div>
      {/if}
    </div>

    <form class="border-t border-border-subtle pt-5" method="POST" action="?/addName">
      <label class="block">
        <span class="field-label">Tenhou name</span>
        <div class="flex flex-col gap-3 sm:flex-row">
          <input
            class="field min-w-0 flex-1"
            name="name"
            value={form?.name ?? ''}
            maxlength="64"
            autocomplete="off"
            required
          />
          <button class="button-primary self-end" type="submit">
            <Plus size={16} strokeWidth={1.75} aria-hidden="true" />
            Add name
          </button>
        </div>
      </label>
    </form>
  </section>
</div>

<style>
  .name-row { display: grid; min-height: 54px; grid-template-columns: 34px minmax(0, 1fr) 44px; align-items: center; gap: 10px; border: 1px solid var(--border-subtle); border-radius: 6px; background: var(--surface-2); padding: 4px 5px 4px 10px; }
  .name-avatar { display: grid; width: 32px; height: 32px; place-items: center; border-radius: 50%; background: var(--surface-3); color: var(--pin); }
  .name-row strong { overflow: hidden; font-size: 13px; line-height: 20px; text-overflow: ellipsis; white-space: nowrap; }
  .empty-name { display: grid; min-height: 76px; place-items: center; border: 1px dashed var(--border-default); border-radius: 6px; color: var(--text-tertiary); font-size: 13px; }
</style>
