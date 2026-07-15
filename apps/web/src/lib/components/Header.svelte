<script lang="ts">
  import type { MeResponse } from '@kifu/api-types';
  import { page } from '$app/state';
  import { ChevronDown, LogIn, Plus } from 'lucide-svelte';

  let { me }: { me: MeResponse | null } = $props();
</script>

<header class="border-b border-border-subtle bg-surface-1">
  <div class="mx-auto max-w-[960px] px-5">
    <div class="flex min-h-16 items-center justify-between gap-4">
      <a class="font-display text-[28px] leading-[34px] font-semibold text-text-primary" href="/">
        Kifu
      </a>

      <nav class="hidden items-stretch self-stretch sm:flex" aria-label="Primary navigation">
        <a
          class="nav-link"
          href="/"
          aria-current={page.url.pathname === '/' || page.url.pathname.startsWith('/games/')
            ? 'page'
            : undefined}>Logs</a
        >
        <a
          class="nav-link"
          href="/career"
          aria-current={page.url.pathname.startsWith('/career') ? 'page' : undefined}>Career</a
        >
      </nav>

      {#if me}
        <details class="account-menu relative">
          <summary
            class="flex cursor-pointer list-none items-center gap-1.5 text-sm text-text-secondary transition-colors duration-fast hover:text-text-primary"
          >
            <span class="max-w-28 truncate">{me.user.username}</span>
            <ChevronDown size={16} strokeWidth={1.75} aria-hidden="true" />
          </summary>
          <div
            class="absolute right-0 z-20 mt-2 min-w-36 overflow-hidden rounded-md border border-border-subtle bg-surface-2 py-1 shadow-lg"
          >
            <a class="account-row" href="/account">Settings</a>
            <div class="border-t border-border-subtle"></div>
            <form method="POST" action="/account?/logout">
              <button class="account-row w-full text-left" type="submit">Log out</button>
            </form>
          </div>
        </details>
      {:else}
        <a class="button-secondary" href="/login">
          <LogIn size={16} strokeWidth={1.75} aria-hidden="true" />
          Log in
        </a>
      {/if}
    </div>

    <nav class="flex h-11 items-stretch border-t border-border-subtle sm:hidden" aria-label="Primary navigation">
      <a
        class="nav-link flex-1 justify-center"
        href="/"
        aria-current={page.url.pathname === '/' || page.url.pathname.startsWith('/games/')
          ? 'page'
          : undefined}
      >
        <Plus size={16} strokeWidth={1.75} aria-hidden="true" />
        Logs
      </a>
      <a
        class="nav-link flex-1 justify-center"
        href="/career"
        aria-current={page.url.pathname.startsWith('/career') ? 'page' : undefined}>Career</a
      >
    </nav>
  </div>
</header>
