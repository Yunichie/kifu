<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  import '../lib/styles/app.css';
  import type { LayoutProps } from './$types';

  let { data, children }: LayoutProps = $props();
</script>

<Header me={data.me} />
<main class="mx-auto w-full max-w-[1180px] px-5 py-8">
  {@render children()}
</main>
