<script lang="ts">
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';
  import ChevronRight from 'lucide-svelte/icons/chevron-right';

  let {
    page,
    hasNext,
    previousHref,
    nextHref,
    label
  }: {
    page: number;
    hasNext: boolean;
    previousHref: string;
    nextHref: string;
    label: string;
  } = $props();
</script>

<nav class="mt-5 grid grid-cols-[1fr_auto_1fr] items-center gap-3" aria-label={label}>
  {#if page > 1}
    <a class="button-secondary justify-self-start" href={previousHref}>
      <ChevronLeft size={16} strokeWidth={1.75} aria-hidden="true" />
      Previous
    </a>
  {/if}
  <span class="col-start-2 font-mono text-[12px] text-text-tertiary">Page {page}</span>
  {#if hasNext}
    <a class="button-secondary col-start-3 justify-self-end" href={nextHref}>
      Next
      <ChevronRight size={16} strokeWidth={1.75} aria-hidden="true" />
    </a>
  {/if}
</nav>
