<script lang="ts">
  import type { MeResponse } from '@kifu/api-types';
  import { page } from '$app/state';
  import ChartNoAxesCombined from 'lucide-svelte/icons/chart-no-axes-combined';
  import ChevronDown from 'lucide-svelte/icons/chevron-down';
  import Files from 'lucide-svelte/icons/files';
  import LogIn from 'lucide-svelte/icons/log-in';
  import LogOut from 'lucide-svelte/icons/log-out';
  import Settings from 'lucide-svelte/icons/settings';
  import ThemeToggle from '$lib/components/ThemeToggle.svelte';

  let { me }: { me: MeResponse | null } = $props();
</script>

<header class="sticky top-0 z-30 border-b border-border-subtle bg-surface-1/95 backdrop-blur-md">
  <div class="mx-auto flex min-h-16 max-w-[1180px] items-center gap-3 px-4 sm:px-5">
    <a class="wordmark mr-auto" href="/" aria-label="Kifu home">
      <span>Kifu</span>
      <span lang="ja">棋譜</span>
    </a>

    <nav class="hidden self-stretch sm:flex" aria-label="Primary navigation">
      <a
        class="nav-link"
        href="/"
        aria-current={page.url.pathname === '/' || page.url.pathname.startsWith('/games/')
          ? 'page'
          : undefined}
      >
        <Files size={16} strokeWidth={1.75} aria-hidden="true" />
        Logs
      </a>
      <a
        class="nav-link"
        href="/career"
        aria-current={page.url.pathname.startsWith('/career') ? 'page' : undefined}
      >
        <ChartNoAxesCombined size={16} strokeWidth={1.75} aria-hidden="true" />
        Career
      </a>
    </nav>

    <ThemeToggle />

    {#if me}
      <details class="account-menu relative">
        <summary class="account-trigger">
          <span class="account-initial" aria-hidden="true">{me.user.displayName.slice(0, 1).toUpperCase()}</span>
          <span class="hidden max-w-28 truncate md:block">{me.user.displayName}</span>
          <span class="menu-chevron"><ChevronDown size={15} strokeWidth={1.75} aria-hidden="true" /></span>
        </summary>
        <div class="account-popover">
          <div class="border-b border-border-subtle px-3 py-2.5 md:hidden">
            <div class="truncate text-[13px] font-bold text-text-primary">{me.user.displayName}</div>
            <div class="mt-0.5 text-[11px] text-text-tertiary">Kifu account</div>
          </div>
          <a class="account-row" href="/account">
            <Settings size={16} strokeWidth={1.75} aria-hidden="true" />
            Settings
          </a>
          <form method="POST" action="/account?/logout">
            <button class="account-row" type="submit">
              <LogOut size={16} strokeWidth={1.75} aria-hidden="true" />
              Log out
            </button>
          </form>
        </div>
      </details>
    {:else}
      <a class="button-secondary px-3 sm:px-4" href="/login">
        <LogIn size={16} strokeWidth={1.75} aria-hidden="true" />
        <span class="hidden min-[390px]:inline">Log in</span>
      </a>
    {/if}
  </div>
</header>

<nav class="mobile-nav sm:hidden" aria-label="Primary navigation">
  <a
    href="/"
    aria-current={page.url.pathname === '/' || page.url.pathname.startsWith('/games/') ? 'page' : undefined}
  >
    <Files size={20} strokeWidth={1.75} aria-hidden="true" />
    <span>Logs</span>
  </a>
  <a href="/career" aria-current={page.url.pathname.startsWith('/career') ? 'page' : undefined}>
    <ChartNoAxesCombined size={20} strokeWidth={1.75} aria-hidden="true" />
    <span>Career</span>
  </a>
</nav>

<style>
  .wordmark {
    display: inline-flex;
    min-height: 44px;
    align-items: baseline;
    gap: 9px;
    color: var(--text-primary);
    font-family: "Zen Old Mincho", "Noto Serif JP", serif;
    font-size: 25px;
    font-weight: 700;
    line-height: 32px;
  }

  .wordmark span:last-child {
    color: var(--gold);
    font-size: 12px;
    line-height: 16px;
  }

  .account-trigger {
    display: flex;
    min-height: 44px;
    cursor: pointer;
    list-style: none;
    align-items: center;
    gap: 7px;
    border-radius: 6px;
    padding: 3px 5px;
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 700;
    transition: color var(--motion-fast), background-color var(--motion-fast);
  }

  .account-trigger:hover { background: var(--surface-2); color: var(--text-primary); }
  .account-initial {
    display: grid;
    width: 32px;
    height: 32px;
    place-items: center;
    border: 1px solid var(--border-default);
    border-radius: 50%;
    background: var(--surface-2);
    color: var(--gold);
    font-family: "Zen Old Mincho", "Noto Serif JP", serif;
    font-size: 13px;
  }

  .menu-chevron { transition: transform var(--motion-fast); }
  .account-menu[open] .menu-chevron { transform: rotate(180deg); }
  .account-popover {
    position: absolute;
    right: 0;
    z-index: 40;
    width: 184px;
    margin-top: 7px;
    overflow: hidden;
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    background: var(--surface-1);
    box-shadow: 0 18px 42px -18px var(--shadow-strong);
    transform-origin: top right;
    animation: menu-in var(--motion-fast) ease-out;
  }

  .mobile-nav {
    position: fixed;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 30;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    border-top: 1px solid var(--border-subtle);
    background: color-mix(in srgb, var(--surface-1) 94%, transparent);
    box-shadow: 0 -12px 32px -24px var(--shadow-strong);
    backdrop-filter: blur(14px);
  }

  .mobile-nav a {
    position: relative;
    display: flex;
    min-height: 64px;
    align-items: center;
    justify-content: center;
    gap: 7px;
    color: var(--text-tertiary);
    font-size: 11px;
    font-weight: 700;
    transition: color var(--motion-fast), background-color var(--motion-fast);
  }

  .mobile-nav a[aria-current='page'] { background: var(--surface-2); color: var(--gold); }
  .mobile-nav a[aria-current='page']::before {
    position: absolute;
    top: -1px;
    left: 50%;
    width: 34px;
    height: 2px;
    background: var(--gold);
    content: '';
    translate: -50%;
  }

  @keyframes menu-in {
    from { opacity: 0; transform: translateY(-3px) scale(0.98); }
  }

</style>
