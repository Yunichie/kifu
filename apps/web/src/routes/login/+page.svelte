<script lang="ts">
  import CircleAlert from 'lucide-svelte/icons/circle-alert';
  import LogIn from 'lucide-svelte/icons/log-in';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>Log in | Kifu</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-12rem)] items-center justify-center py-8">
  <section class="panel w-full max-w-[360px]">
    <h1 class="mb-2 text-[15px] leading-[21px] font-semibold">Log in</h1>
    <p class="mb-5 text-[13px] leading-[19px] text-text-tertiary">
      Use your Google account to continue.
    </p>

    {#if data.message}
      <div class="status-error mb-4" role="alert">
        <CircleAlert class="mt-0.5 shrink-0" size={16} strokeWidth={1.75} aria-hidden="true" />
        <span>{data.message}</span>
      </div>
    {/if}

    <a class="button-primary w-full" href="/auth/google">
      <LogIn size={16} strokeWidth={1.75} aria-hidden="true" />
      Continue with Google
    </a>
  </section>
</div>
