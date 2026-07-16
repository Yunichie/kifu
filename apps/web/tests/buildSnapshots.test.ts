import type { GameDetail } from '@kifu/api-types';
import { describe, expect, test } from 'bun:test';
import fixture from './fixtures/2017010100gm-00a9-0000-003dbd5d.json';
import { buildSnapshots } from '../src/lib/replay/buildSnapshots';

const game = fixture as unknown as GameDetail;

describe('buildSnapshots', () => {
  test('reconstructs the Rust fixture through the final player scores', () => {
    let finalScores: number[] = [];

    for (const [index, kyoku] of game.kyoku.entries()) {
      const isLastKyoku = index === game.kyoku.length - 1;
      const expectedScores = isLastKyoku
        ? game.players.toSorted((a, b) => a.seat - b.seat).map((player) => player.finalScore ?? 0)
        : kyoku.endScores;
      const snapshots = buildSnapshots(kyoku.startHands, kyoku.events, {
        startScores: kyoku.startScores,
        endScores: expectedScores,
        initialDoraIndicators: kyoku.doraIndicators.slice(0, 1),
        riichiSticks: kyoku.riichiSticks,
        result: kyoku.result,
        clearRiichiSticks: isLastKyoku
      });
      const final = snapshots.at(-1)!;

      expect(snapshots).toHaveLength(kyoku.events.length + 2);
      expect(final.turnIndex).toBe(kyoku.events.length + 1);
      expect(final.settled).toBe(true);
      expect(final.scores).toEqual(expectedScores);
      expect(final.doraIndicators).toEqual(kyoku.doraIndicators);
      finalScores = final.scores;

      if (index === 0) {
        const beforeSettlement = snapshots.at(-2)!;
        expect(beforeSettlement.settled).toBe(false);
        expect(beforeSettlement.riichiSticks).toBe(2);
        expect(final.riichiSticks).toBe(0);
        expect(kyoku.result.type).toBe('win');
        if (kyoku.result.type === 'win') {
          expect(sortTiles(final.hands[kyoku.result.wins[0].winner])).toEqual(
            sortTiles(kyoku.result.wins[0].winningTiles)
          );
        }
        expect(final.hands.map(sortTiles)).toEqual([
          [12, 17, 21, 27, 28, 35, 49, 50, 54, 61, 65, 75, 78],
          [18, 19, 22, 25, 29, 41, 44, 51, 63, 67, 71, 81, 85, 91],
          [16, 20, 26, 30, 32, 42, 46, 48, 53, 56, 60, 112, 115],
          [4, 5, 6, 14, 15, 40, 45, 82, 90, 99, 100, 103, 107]
        ]);
        expect(final.discards.map((pile) => pile.map((discard) => discard.tile))).toEqual([
          [37, 104, 114, 118, 124, 38, 93, 111, 135, 88],
          [113, 106, 34, 126, 2, 110, 10, 39, 47, 83],
          [72, 108, 131, 130, 7, 89, 98, 133, 36, 71],
          [121, 73, 123, 33, 69, 62, 64, 23, 43]
        ]);
        expect(final.remainingWall).toBe(31);
      }

      if (index === 3) {
        expect(final.hands.map(sortTiles)).toEqual([
          [17, 21, 25, 77, 79, 99, 100],
          [9, 26, 45, 52, 55, 58, 65, 67, 84, 85, 97, 117, 119],
          [41, 42, 48, 49, 51, 54, 57, 61, 78, 82, 86],
          [2, 3, 4, 6, 15, 39, 40, 44, 60, 64, 89, 96, 107]
        ]);
        expect(final.melds.map((melds) => melds.map((meld) => [meld.kind, meld.calledTile]))).toEqual([
          [['pon', 80], ['pon', 31]],
          [],
          [['pon', 132]],
          []
        ]);
        expect(final.discards.map((pile) => pile.filter((discard) => discard.calledBy !== null).map((discard) => [discard.tile, discard.calledBy]))).toEqual([
          [],
          [[132, 2]],
          [[80, 0]],
          [[31, 0]]
        ]);
      }
    }

    expect(finalScores).toEqual(
      game.players.toSorted((a, b) => a.seat - b.seat).map((player) => player.finalScore ?? 0)
    );
  });
});

function sortTiles(tiles: number[]): number[] {
  return tiles.toSorted((a, b) => a - b);
}
