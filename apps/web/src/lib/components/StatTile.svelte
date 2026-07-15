<script lang="ts">
  let { label, value }: { label: string; value: string | number } = $props();
</script>

<div class="rounded-md bg-tile-bg px-2.5 py-3 text-center text-tile-text tile-shadow">
  <div class="font-mono text-[26px] leading-none font-semibold">{value}</div>
  <div class="mt-2 text-[11px] leading-[15px] font-medium uppercase text-tile-text/60">{label}</div>
</div>
