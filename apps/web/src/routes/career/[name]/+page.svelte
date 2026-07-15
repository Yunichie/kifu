<script lang="ts">
  import CareerView from '$lib/components/CareerView.svelte';
  import type { PageProps } from './$types';

  let { data, params }: PageProps = $props();
</script>

<svelte:head>
  <title>{params.name} | Kifu</title>
</svelte:head>

<CareerView career={data.career} title={params.name} emptyHref="/" emptyLabel="Browse the ledger" />
