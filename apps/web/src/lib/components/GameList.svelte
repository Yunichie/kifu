<script lang="ts">
  import type { GameListItem } from '@kifu/api-types';
  import ArrowRight from 'lucide-svelte/icons/arrow-right';
  import X from 'lucide-svelte/icons/x';
  import { formatDate, formatScore, rulesLabel } from '$lib/format';

  let { games, removable = false }: { games: GameListItem[]; removable?: boolean } = $props();
  const seatBorders = ['border-man', 'border-pin', 'border-sou', 'border-gold'];
</script>

<div class="table-shell">
  <table class="ledger">
    <thead><tr><th>Date</th><th>Rules</th><th>Players</th><th><span class="sr-only">Actions</span></th></tr></thead>
    <tbody>
      {#each games as game (game.logId)}
        <tr class="border-l-[3px] border-l-border">
          <td><time datetime={new Date(game.addedAt).toISOString()}>{formatDate(game.addedAt)}</time></td>
          <td>
            <div>{rulesLabel(game.rules)}</div>
            <div class="mt-1.5 flex flex-wrap gap-1.5 font-sans">
              {#if game.rules.akaDora}<span class="stamp-tag text-gold">Aka</span>{/if}
              {#if game.rules.kuitan}<span class="stamp-tag">Kuitan</span>{/if}
            </div>
          </td>
          <td>
            <div class="flex min-w-[420px] flex-wrap gap-x-4 gap-y-2">
              {#each game.players as player (player.seat)}
                <a
                  class={`inline-flex items-center border-l-2 pl-1.5 font-sans text-[13px] font-medium text-text-primary hover:underline ${seatBorders[player.seat] ?? 'border-border'}`}
                  href={`/career/${encodeURIComponent(player.name)}`}
                >
                  <span>{player.name}</span>
                  <span class="ml-1 font-mono text-text-secondary">{formatScore(player.finalScore)}</span>
                  {#if player.placement}
                    <span class={['hanko ml-1', player.placement === 1 ? 'border-gold bg-gold/10 text-gold' : 'border-border text-text-tertiary']}>#{player.placement}</span>
                  {/if}
                </a>
              {/each}
            </div>
          </td>
          <td>
            <div class="flex items-center justify-end gap-1">
              <a class="inline-flex size-9 items-center justify-center rounded-md text-text-secondary transition-colors duration-fast hover:bg-surface-3 hover:text-text-primary" href={`/games/${game.logId}`} aria-label={`Open game ${game.logId}`} title="Open game">
                <ArrowRight size={16} strokeWidth={1.75} aria-hidden="true" />
              </a>
              {#if removable}
                <form method="POST" action="?/remove">
                  <input type="hidden" name="logId" value={game.logId} />
                  <button class="inline-flex size-9 items-center justify-center rounded-md text-text-tertiary transition-colors duration-fast hover:bg-man/15 hover:text-man" type="submit" aria-label={`Remove game ${game.logId} from library`} title="Remove from library">
                    <X size={16} strokeWidth={1.75} aria-hidden="true" />
                  </button>
                </form>
              {/if}
            </div>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
