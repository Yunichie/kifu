<script lang="ts">
  import ExternalLink from 'lucide-svelte/icons/external-link';
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
  <div class="mb-3 flex justify-end">
    <a
      class="button-secondary min-h-9 px-3 text-[13px]"
      href={tenhouReplayUrl}
      target="_blank"
      rel="noopener noreferrer"
    >
      <ExternalLink size={15} strokeWidth={1.75} aria-hidden="true" />
      Open in Tenhou
    </a>
  </div>

  <div class="-mx-5 overflow-hidden border-y border-border-subtle bg-surface-0 sm:mx-0 sm:border-x">
    <iframe
      class="block h-[clamp(480px,72vw,760px)] w-full border-0"
      src={tenhouReplayUrl}
      title={`Tenhou table replay for ${data.game.logId}`}
    ></iframe>
  </div>
</section>
