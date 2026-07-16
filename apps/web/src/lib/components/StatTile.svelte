<script lang="ts">
  import StatCard from '$lib/components/StatCard.svelte';
  let { label, value, tone = 'neutral' }: { label: string; value: string | number; tone?: 'neutral' | 'win' | 'loss' | 'gold' } = $props();
</script>

<StatCard {label} {value} {tone} />
