import type { KyokuResult, PlayerSummary, Ruleset } from '@kifu/api-types';

export function formatPercent(value: number): string {
  return `${(value * 100).toFixed(1)}%`;
}

export function formatScore(value: number | null): string {
  if (value === null) return '-';
  return new Intl.NumberFormat('en-US').format(value);
}

export function formatSigned(value: number): string {
  return `${value > 0 ? '+' : ''}${new Intl.NumberFormat('en-US').format(value)}`;
}

export function formatDate(timestamp: number): string {
  return new Date(timestamp).toISOString().slice(0, 10);
}

export function rulesLabel(rules: Ruleset): string {
  const length = rules.hanchan ? 'Hanchan' : 'Tonpuu';
  return `${rules.sanma ? '3-player' : '4-player'} ${length}`;
}

export function roundLabel(bakaze: number, kyokuNumber: number, honba: number): string {
  const winds = ['East', 'South', 'West', 'North'];
  const round = `${winds[bakaze] ?? `Round ${bakaze + 1}`} ${kyokuNumber}`;
  return honba > 0 ? `${round}, ${honba} honba` : round;
}

export function resultLabel(result: KyokuResult, players: PlayerSummary[]): string {
  if (result.type === 'draw') {
    if (result.reason.type === 'exhaustive') return 'Exhaustive draw';
    return result.reason.type.replace(/([A-Z])/g, ' $1').toLowerCase();
  }

  return result.wins
    .map((win) => {
      const winner = players[win.winner]?.name ?? `Seat ${win.winner + 1}`;
      if (win.tsumo) return `${winner} tsumo`;
      const source = players[win.fromSeat]?.name ?? `Seat ${win.fromSeat + 1}`;
      return `${winner} ron from ${source}`;
    })
    .join(' / ');
}
