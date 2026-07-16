<script lang="ts">
  let { wind, dealer = false }: { wind: string; dealer?: boolean } = $props();
</script>

<span class:dealer class="wind-marker" aria-label={`${wind} seat${dealer ? ', dealer' : ''}`}>
  <b lang="ja">{wind}</b>
  {#if dealer}<small lang="ja">親</small>{/if}
</span>

<style>
  .wind-marker { display: inline-grid; width: 31px; height: 31px; place-items: center; border: 1px solid var(--border-strong); border-radius: 50%; background: var(--surface-1); color: var(--text-secondary); box-shadow: 0 2px 5px rgba(0, 0, 0, 0.3); }
  b { font-family: "Shippori Mincho", serif; font-size: 15px; line-height: 1; }
  small { display: none; }
  .dealer { position: relative; border-color: var(--gold); color: var(--gold); }
  .dealer small { position: absolute; right: -7px; bottom: -4px; display: grid; width: 15px; height: 15px; place-items: center; border-radius: 50%; background: var(--gold); color: var(--gold-ink); font-size: 8px; font-weight: 700; }
</style>
