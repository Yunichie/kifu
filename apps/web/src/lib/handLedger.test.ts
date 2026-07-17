import { describe, expect, test } from 'bun:test';
import { buildHandLedger } from './handLedger';

describe('hand ledger chart data', () => {
  test('orders players by seat and plots scores from the start through each hand', () => {
    const ledger = buildHandLedger({
      players: [
        { seat: 2, name: 'C', finalScore: 25000 },
        { seat: 0, name: 'A', finalScore: 27000 },
        { seat: 1, name: 'B', finalScore: 23000 }
      ],
      kyoku: [
        {
          roundIndex: 0,
          bakaze: 0,
          kyokuNumber: 1,
          honba: 0,
          startScores: [25000, 25000, 25000],
          endScores: [26000, 24000, 25000]
        },
        {
          roundIndex: 1,
          bakaze: 0,
          kyokuNumber: 2,
          honba: 0,
          startScores: [26000, 24000, 25000],
          endScores: [27000, 23000, 25000]
        }
      ]
    });

    expect(ledger.terminalSettlement).toBe(false);
    expect(ledger.chart.series.map((series) => series.name)).toEqual(['A', 'B', 'C']);
    expect(ledger.chart.series[0].points.map((point) => point.label)).toEqual([
      'Start',
      'East 1',
      'East 2'
    ]);
    expect(ledger.chart.series[0].points.map((point) => point.score)).toEqual([25000, 26000, 27000]);
  });

  test('adds the final settlement as the last chart point', () => {
    const ledger = buildHandLedger({
      players: [
        { seat: 0, name: 'A', finalScore: 31000 },
        { seat: 1, name: 'B', finalScore: 19000 }
      ],
      kyoku: [
        {
          roundIndex: 0,
          bakaze: 0,
          kyokuNumber: 1,
          honba: 0,
          startScores: [25000, 25000],
          endScores: [30000, 20000]
        }
      ]
    });

    expect(ledger.terminalSettlement).toBe(true);
    expect(ledger.finalScores).toEqual([31000, 19000]);
    expect(ledger.chart.series[0].points.at(-1)).toEqual({ id: 'final', label: 'Final', score: 31000 });
    expect(ledger.chart.series[1].points.at(-1)?.score).toBe(19000);
  });

  test('returns empty chart series when no hands were recorded', () => {
    const ledger = buildHandLedger({
      players: [{ seat: 0, name: 'A', finalScore: null }],
      kyoku: []
    });

    expect(ledger.terminalSettlement).toBe(false);
    expect(ledger.chart.series[0].points).toEqual([]);
  });
});
