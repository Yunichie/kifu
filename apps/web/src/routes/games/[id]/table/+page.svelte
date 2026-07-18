<script lang="ts">
  import ExternalLink from 'lucide-svelte/icons/external-link';
  import MonitorPlay from 'lucide-svelte/icons/monitor-play';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();
  let initialSeat = $derived(
    data.game.players
      .toSorted((a, b) => a.seat - b.seat)
      .find((player) => data.me?.playerNames.includes(player.name))?.seat ?? 0
  );
  let tenhouReplayUrl = $derived(
    `https://tenhou.net/5/?log=${encodeURIComponent(data.game.logId)}&tw=${initialSeat}`
  );
</script>

<svelte:head>
  <title>Table Replay | Kifu</title>
</svelte:head>

<section>
  <div class="mb-3 flex items-center justify-between gap-3">
    <div class="flex items-center gap-2 text-[12px] font-bold text-text-secondary">
      <span class="grid size-8 place-items-center rounded-md bg-surface-2 text-pin" aria-hidden="true"><MonitorPlay size={16} strokeWidth={1.75} /></span>
      Tenhou replay
    </div>
    <a class="button-secondary min-h-11 px-3 text-[12px]" href={tenhouReplayUrl} target="_blank" rel="noopener noreferrer">
      <ExternalLink size={15} strokeWidth={1.75} aria-hidden="true" />
      Open in Tenhou
    </a>
  </div>

  <div class="replay-frame">
    <iframe
      src={tenhouReplayUrl}
      title={`Tenhou table replay for ${data.game.logId}`}
      loading="eager"
      referrerpolicy="no-referrer"
      allowfullscreen
    ></iframe>
  </div>
</section>

<style>
  .replay-frame { overflow: hidden; border: 1px solid var(--border-subtle); border-radius: 6px; background: var(--surface-0); box-shadow: 0 22px 54px -38px var(--shadow-strong); overscroll-behavior: contain; }
  iframe { display: block; width: 100%; height: clamp(560px, 72vw, 780px); border: 0; background: var(--surface-0); }
  @media (max-width: 639px) {
    .replay-frame { margin-inline: -16px; border-right: 0; border-left: 0; border-radius: 0; }
    iframe { height: max(540px, calc(100svh - 210px)); }
  }
</style>
