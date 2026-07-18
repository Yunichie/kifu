<script lang="ts">
  import type { GameListItem, GameListPlayer } from '@kifu/api-types';
  import ArrowRight from 'lucide-svelte/icons/arrow-right';
  import Crown from 'lucide-svelte/icons/crown';
  import X from 'lucide-svelte/icons/x';
  import { formatDate, formatScore, rulesLabel } from '$lib/format';

  let { games, removable = false }: { games: GameListItem[]; removable?: boolean } = $props();
  const seatBorders = ['border-man', 'border-pin', 'border-sou', 'border-gold'];
  const seatDots = ['bg-man', 'bg-pin', 'bg-sou', 'bg-gold'];

  function ranked(players: GameListPlayer[]): GameListPlayer[] {
    return players.toSorted((a, b) => (a.placement ?? 9) - (b.placement ?? 9) || a.seat - b.seat);
  }

  function placementLabel(placement: number | null): string {
    if (!placement) return '—';
    return `${placement}${placement === 1 ? 'st' : placement === 2 ? 'nd' : placement === 3 ? 'rd' : 'th'}`;
  }
</script>

<div class="hidden md:block">
  <div class="table-shell">
    <table class="ledger game-ledger">
      <thead>
        <tr><th>Date</th><th>Rules</th><th>Placement</th><th><span class="sr-only">Actions</span></th></tr>
      </thead>
      <tbody>
        {#each games as game (game.logId)}
          <tr>
            <td class="align-top">
              <time datetime={new Date(game.addedAt).toISOString()}>{formatDate(game.addedAt)}</time>
              <div class="mt-1 max-w-32 truncate text-[10px] leading-4 font-normal text-text-tertiary" title={game.logId}>{game.logId}</div>
            </td>
            <td class="align-top">
              <div class="font-sans text-[13px] font-bold">{rulesLabel(game.rules)}</div>
              <div class="mt-2 flex flex-wrap gap-1.5 font-sans">
                {#if game.rules.akaDora}<span class="stamp-tag text-gold">Aka</span>{/if}
                {#if game.rules.kuitan}<span class="stamp-tag">Kuitan</span>{/if}
                {#if game.rules.fast}<span class="stamp-tag">Fast</span>{/if}
              </div>
            </td>
            <td>
              <ol class="grid min-w-[440px] grid-cols-2 gap-x-5 gap-y-2 font-sans">
                {#each ranked(game.players) as player (player.seat)}
                  <li class={['player-line', player.placement === 1 && 'winner']}>
                    <span class={['seat-dot', seatDots[player.seat] ?? 'bg-border']} aria-hidden="true"></span>
                    <span class="placement">
                      {#if player.placement === 1}<Crown size={13} strokeWidth={1.75} aria-hidden="true" />{/if}
                      {placementLabel(player.placement)}
                    </span>
                    <a class="truncate hover:underline" href={`/career/${encodeURIComponent(player.name)}`}>{player.name}</a>
                    <span class="score">{formatScore(player.finalScore)}</span>
                  </li>
                {/each}
              </ol>
            </td>
            <td>
              <div class="flex items-center justify-end gap-1">
                <a class="icon-button" href={`/games/${game.logId}`} aria-label={`Open game ${game.logId}`} title="Open game">
                  <ArrowRight size={17} strokeWidth={1.75} aria-hidden="true" />
                </a>
                {#if removable}
                  <form method="POST" action="?/remove">
                    <input type="hidden" name="logId" value={game.logId} />
                    <button class="icon-button hover:text-man" type="submit" aria-label={`Remove game ${game.logId} from library`} title="Remove from library">
                      <X size={17} strokeWidth={1.75} aria-hidden="true" />
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
</div>

<div class="grid gap-3 md:hidden">
  {#each games as game (game.logId)}
    <article class="mobile-game">
      <header>
        <div class="min-w-0">
          <time class="font-mono text-[12px] font-semibold" datetime={new Date(game.addedAt).toISOString()}>{formatDate(game.addedAt)}</time>
          <p class="mt-0.5 truncate font-mono text-[10px] leading-4 text-text-tertiary">{game.logId}</p>
        </div>
        <div class="flex items-center gap-1">
          {#if game.rules.akaDora}<span class="stamp-tag text-gold">Aka</span>{/if}
          <a class="icon-button" href={`/games/${game.logId}`} aria-label={`Open game ${game.logId}`} title="Open game">
            <ArrowRight size={17} strokeWidth={1.75} aria-hidden="true" />
          </a>
          {#if removable}
            <form method="POST" action="?/remove">
              <input type="hidden" name="logId" value={game.logId} />
              <button class="icon-button hover:text-man" type="submit" aria-label={`Remove game ${game.logId} from library`} title="Remove from library">
                <X size={17} strokeWidth={1.75} aria-hidden="true" />
              </button>
            </form>
          {/if}
        </div>
      </header>
      <div class="border-b border-border-subtle px-3 py-2 text-[11px] font-bold text-text-secondary">
        {rulesLabel(game.rules)}{game.rules.kuitan ? ' · Kuitan' : ''}{game.rules.fast ? ' · Fast' : ''}
      </div>
      <ol class="divide-y divide-border-subtle">
        {#each ranked(game.players) as player (player.seat)}
          <li class={['mobile-player', player.placement === 1 && 'winner']}>
            <span class={['seat-rule', seatBorders[player.seat] ?? 'border-border']} aria-hidden="true"></span>
            <span class="mobile-place">
              {#if player.placement === 1}<Crown size={14} strokeWidth={1.75} aria-label="Winner" />{/if}
              {placementLabel(player.placement)}
            </span>
            <a class="min-w-0 flex-1 truncate font-bold hover:underline" href={`/career/${encodeURIComponent(player.name)}`}>{player.name}</a>
            <span class="font-mono text-[13px] font-semibold tabular-nums">{formatScore(player.finalScore)}</span>
          </li>
        {/each}
      </ol>
    </article>
  {/each}
</div>

<style>
  .game-ledger td { padding-block: 14px; }
  .player-line {
    display: grid;
    min-width: 0;
    grid-template-columns: 5px 44px minmax(0, 1fr) auto;
    align-items: center;
    gap: 7px;
    color: var(--text-secondary);
    font-size: 12px;
    line-height: 18px;
  }

  .player-line.winner { color: var(--text-primary); font-weight: 700; }
  .seat-dot { width: 5px; height: 18px; border-radius: 2px; }
  .placement { display: inline-flex; align-items: center; gap: 3px; color: var(--text-tertiary); font-size: 10px; font-weight: 700; text-transform: uppercase; }
  .winner .placement { color: var(--gold); }
  .score { color: var(--text-primary); font-family: "Roboto Mono", "BIZ UDPGothic", monospace; font-variant-numeric: tabular-nums; font-weight: 600; }

  .mobile-game {
    overflow: hidden;
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    background: var(--surface-1);
    box-shadow: 0 12px 24px -22px var(--shadow-strong);
  }

  .mobile-game > header { display: flex; min-height: 56px; align-items: center; justify-content: space-between; gap: 12px; padding: 6px 7px 6px 12px; }
  .mobile-player { position: relative; display: flex; min-height: 47px; align-items: center; gap: 9px; padding: 8px 12px 8px 15px; color: var(--text-secondary); font-size: 13px; }
  .mobile-player.winner { background: color-mix(in srgb, var(--gold) 8%, transparent); color: var(--text-primary); }
  .seat-rule { position: absolute; top: 9px; bottom: 9px; left: 0; border-left-width: 3px; }
  .mobile-place { display: inline-flex; width: 48px; flex: 0 0 auto; align-items: center; gap: 4px; color: var(--text-tertiary); font-size: 10px; font-weight: 700; text-transform: uppercase; }
  .winner .mobile-place { color: var(--gold); }
</style>
