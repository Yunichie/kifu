<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import LogIn from 'lucide-svelte/icons/log-in';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>Log in | Kifu</title>
  <meta name="robots" content="noindex,follow" />
</svelte:head>

<div class="auth-shell">
  <section class="auth-panel">
    <div class="auth-mark" aria-hidden="true"><span lang="ja">棋譜</span><i></i></div>
    <p class="section-kicker">Kifu account</p>
    <h1>Log in</h1>
    <p class="subcopy">Continue with your Google account.</p>

    {#if data.message}
      <div class="status-error mt-5" role="alert">
        <CircleAlert class="mt-0.5 shrink-0" size={16} strokeWidth={1.75} aria-hidden="true" />
        <span>{data.message}</span>
      </div>
    {/if}

    <a class="button-primary mt-6 w-full" href="/auth/google">
      <LogIn size={16} strokeWidth={1.75} aria-hidden="true" />
      Continue with Google
    </a>
  </section>
</div>

<style>
  .auth-shell { display: flex; min-height: calc(100vh - 11rem); align-items: center; justify-content: center; padding-block: 28px; }
  .auth-panel { position: relative; width: 100%; max-width: 420px; overflow: hidden; border: 1px solid var(--border-subtle); border-radius: 8px; background: var(--surface-1); padding: 30px; box-shadow: 0 24px 60px -38px var(--shadow-strong); }
  .auth-panel::before { position: absolute; top: 0; right: 0; left: 0; height: 3px; background: var(--gold); content: ''; }
  .auth-mark { display: grid; width: 54px; height: 70px; place-items: center; border: 1px solid var(--tile-edge); border-radius: 6px; background: linear-gradient(var(--tile-highlight), var(--tile-bg) 78%, var(--tile-bottom)); color: var(--tile-text); box-shadow: 0 3px 0 var(--tile-edge); }
  .auth-mark span { font-family: "Zen Old Mincho", "Noto Serif JP", serif; font-size: 18px; font-weight: 700; writing-mode: vertical-rl; }
  .auth-mark i { position: absolute; width: 18px; height: 3px; margin-top: 54px; border-radius: 2px; background: var(--gold); }
  .section-kicker { margin-top: 26px; }
  h1 { margin-top: 4px; font-family: "Zen Old Mincho", "Noto Serif JP", serif; font-size: 28px; font-weight: 700; line-height: 34px; }
  .subcopy { margin-top: 8px; color: var(--text-secondary); font-size: 13px; line-height: 20px; }
  @media (max-width: 639px) { .auth-shell { min-height: calc(100vh - 10rem); padding-block: 12px; } .auth-panel { border-right: 0; border-left: 0; border-radius: 0; padding: 26px 20px; } }
</style>
