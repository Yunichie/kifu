<script lang="ts">
  import { CircleAlert, LogIn, UserPlus } from 'lucide-svelte';
  import type { PageProps } from './$types';

  let { data, form }: PageProps = $props();
  let mode = $derived(form?.mode ?? data.mode);
  let signup = $derived(mode === 'signup');
</script>

<svelte:head>
  <title>{signup ? 'Sign up' : 'Log in'} | Kifu</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-12rem)] items-center justify-center py-8">
  <section class="panel w-full max-w-[360px]">
    <h1 class="mb-5 text-[15px] leading-[21px] font-semibold">
      {signup ? 'Create an account' : 'Log in'}
    </h1>

    {#if form?.message}
      <div class="status-error mb-4" role="alert">
        <CircleAlert class="mt-0.5 shrink-0" size={16} strokeWidth={1.75} aria-hidden="true" />
        <span>{form.message}</span>
      </div>
    {/if}

    <form class="space-y-4" method="POST" action={signup ? '?/signup' : '?/login'}>
      <label>
        <span class="field-label">Username</span>
        <input
          class="field"
          name="username"
          value={form?.username ?? ''}
          autocomplete="username"
          minlength="3"
          maxlength="32"
          pattern="[A-Za-z0-9_\-]+"
          required
        />
      </label>
      <label>
        <span class="field-label">Password</span>
        <input
          class="field"
          name="password"
          type="password"
          autocomplete={signup ? 'new-password' : 'current-password'}
          minlength="12"
          maxlength="128"
          required
        />
      </label>
      <button class="button-primary w-full" type="submit">
        {#if signup}
          <UserPlus size={16} strokeWidth={1.75} aria-hidden="true" />
          Sign up
        {:else}
          <LogIn size={16} strokeWidth={1.75} aria-hidden="true" />
          Log in
        {/if}
      </button>
    </form>

    <p class="mt-4 text-center text-[13px] leading-[19px] text-text-secondary">
      {#if signup}
        Already have one? <a class="text-pin hover:underline" href="/login">Log in</a>
      {:else}
        Need an account? <a class="text-pin hover:underline" href="/login?mode=signup">Sign up</a>
      {/if}
    </p>
  </section>
</div>
