import type { GameDetail } from '@kifu/api-types';

const chartWinds = ['E', 'S', 'W', 'N'];

function chartRoundLabel(bakaze: number, kyokuNumber: number, honba: number): string {
  const round = `${chartWinds[bakaze] ?? `R${bakaze + 1}`}${kyokuNumber}`;
  return honba > 0 ? `${round}:${honba}H` : round;
}

type LedgerPlayer = Pick<GameDetail['players'][number], 'seat' | 'name' | 'finalScore'>;
type LedgerHand = Pick<
  GameDetail['kyoku'][number],
  'roundIndex' | 'bakaze' | 'kyokuNumber' | 'honba' | 'startScores' | 'endScores'
>;

export type HandScorePoint = {
  id: string;
  label: string;
  score: number;
};

export type HandScoreSeries = {
  seat: number;
  name: string;
  points: HandScorePoint[];
};

export type HandScoreChartData = {
  series: HandScoreSeries[];
};

export function buildHandLedger(game: { players: LedgerPlayer[]; kyoku: LedgerHand[] }) {
  const players = game.players.toSorted((a, b) => a.seat - b.seat);
  const finalScores = players.map((player) => player.finalScore ?? 0);
  const lastHand = game.kyoku.at(-1);
  const terminalSettlement =
    lastHand !== undefined &&
    players.some((player, index) => finalScores[index] !== lastHand.endScores[player.seat]);

  const snapshots = game.kyoku.length
    ? [
        { id: 'start', label: 'Start', scores: game.kyoku[0].startScores },
        ...game.kyoku.map((hand) => ({
          id: `hand-${hand.roundIndex}-${hand.honba}`,
          label: chartRoundLabel(hand.bakaze, hand.kyokuNumber, hand.honba),
          scores: hand.endScores
        })),
        ...(terminalSettlement ? [{ id: 'final', label: 'Final', scores: finalScores }] : [])
      ]
    : [];

  return {
    finalScores,
    terminalSettlement,
    chart: {
      series: players.map((player) => ({
        seat: player.seat,
        name: player.name,
        points: snapshots.map((snapshot) => ({
          id: snapshot.id,
          label: snapshot.label,
          score: snapshot.scores[player.seat] ?? 0
        }))
      }))
    } satisfies HandScoreChartData
  };
}
