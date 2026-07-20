<script lang="ts">
  import CareerView from '$lib/components/CareerView.svelte';
  import { careerCanonical } from '$lib/seo';
  import type { PageProps } from './$types';

  let { data, params }: PageProps = $props();
  let canonical = $derived(careerCanonical(data.siteUrl, params.name));
  let pageTitle = $derived(`${params.name} Tenhou career statistics | Kifu`);
  let description = $derived(
    `${params.name}'s riichi mahjong statistics across ${data.career.games} public Tenhou games.`
  );
</script>

<svelte:head>
  <title>{pageTitle}</title>
  <meta name="description" content={description} />
  <link rel="canonical" href={canonical} />
  <meta property="og:type" content="profile" />
  <meta property="og:site_name" content="Kifu" />
  <meta property="og:title" content={pageTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={canonical} />
  <meta name="twitter:card" content="summary" />
</svelte:head>

<CareerView career={data.career} title={params.name} emptyHref="/" emptyLabel="Browse the ledger" />
