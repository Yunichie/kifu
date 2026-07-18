<script lang="ts">
  import { onMount } from 'svelte';
  import Moon from 'lucide-svelte/icons/moon';
  import Sun from 'lucide-svelte/icons/sun';
  import { readAppliedTheme, toggleTheme, watchSystemTheme, type Theme } from '$lib/theme';

  let theme = $state<Theme>('light');
  let label = $derived(theme === 'dark' ? 'Use light theme' : 'Use dark theme');

  onMount(() => {
    theme = readAppliedTheme();
    return watchSystemTheme((next) => theme = next);
  });

  function handleToggle(): void {
    theme = toggleTheme();
  }
</script>

<button
  class="icon-button"
  type="button"
  onclick={handleToggle}
  aria-label={label}
  aria-pressed={theme === 'dark'}
  title={label}
>
  <span class="theme-moon"><Moon size={18} strokeWidth={1.75} aria-hidden="true" /></span>
  <span class="theme-sun"><Sun size={18} strokeWidth={1.75} aria-hidden="true" /></span>
</button>

<style>
  .theme-sun { display: none; }
  :global(html[data-theme='dark']) .theme-moon { display: none; }
  :global(html[data-theme='dark']) .theme-sun { display: block; }
</style>
