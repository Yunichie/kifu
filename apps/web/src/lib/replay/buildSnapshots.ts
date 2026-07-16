import type { CallKind, KyokuResult, TurnEvent } from '@kifu/api-types';

export type ReplayDiscard = {
  tile: number;
  tsumogiri: boolean;
  calledBy: number | null;
  riichi: boolean;
};

export type ReplayMeld = {
  kind: CallKind;
  tiles: number[];
  fromSeat: number;
  calledTile: number | null;
};

export type TableSnapshot = {
  turnIndex: number;
  hands: number[][];
  drawnTiles: Array<number | null>;
  discards: ReplayDiscard[][];
  melds: ReplayMeld[][];
  doraIndicators: number[];
  scores: number[];
  remainingWall: number;
  riichiSticks: number;
  event: TurnEvent | null;
  settled: boolean;
};

export type SnapshotOptions = {
  startScores?: number[];
  endScores?: number[];
  initialDoraIndicators?: number[];
  riichiSticks?: number;
  result?: KyokuResult;
  clearRiichiSticks?: boolean;
};

export function buildSnapshots(
  startHands: number[][],
  events: TurnEvent[],
  options: SnapshotOptions = {}
): TableSnapshot[] {
  const activeSeats = startHands.filter((hand) => hand.length > 0).length;
  const initialWall = (activeSeats === 3 ? 108 : 136) -
    startHands.reduce((total, hand) => total + hand.length, 0) -
    14;
  const initial: TableSnapshot = {
    turnIndex: 0,
    hands: fourSeats(startHands, (hand) => [...hand]),
    drawnTiles: [null, null, null, null],
    discards: fourSeats([], () => []),
    melds: fourSeats([], () => []),
    doraIndicators: [...(options.initialDoraIndicators ?? [])],
    scores: fourSeats(options.startScores ?? [], (_, seat) =>
      options.startScores?.[seat] ?? (startHands[seat]?.length ? (activeSeats === 3 ? 35_000 : 25_000) : 0)
    ),
    remainingWall: Math.max(0, initialWall),
    riichiSticks: options.riichiSticks ?? 0,
    event: null,
    settled: false
  };
  const snapshots = [initial];

  for (const [index, event] of events.entries()) {
    const snapshot = cloneSnapshot(snapshots[index], index + 1, event);
    applyEvent(snapshot, event);
    snapshots.push(snapshot);
  }

  if (options.endScores) {
    const settlement = cloneSnapshot(snapshots.at(-1)!, snapshots.length, null);
    settlement.scores = fourSeats(options.endScores, (score) => score);
    settlement.settled = true;
    applySettlement(settlement, options.result, options.clearRiichiSticks ?? false);
    snapshots.push(settlement);
  }
  return snapshots;
}

function applySettlement(
  snapshot: TableSnapshot,
  result: KyokuResult | undefined,
  clearRiichiSticks: boolean
): void {
  if (result?.type === 'win') {
    for (const win of result.wins) {
      snapshot.hands[win.winner] = [...win.winningTiles];
      snapshot.drawnTiles[win.winner] = null;
    }
  }
  if (result?.type === 'win' || clearRiichiSticks) snapshot.riichiSticks = 0;
}

function applyEvent(snapshot: TableSnapshot, event: TurnEvent): void {
  switch (event.type) {
    case 'Draw':
      snapshot.hands[event.seat].push(event.tile);
      snapshot.drawnTiles[event.seat] = event.tile;
      snapshot.remainingWall = Math.max(0, snapshot.remainingWall - 1);
      return;
    case 'Discard':
      removeTile(snapshot.hands[event.seat], event.tile);
      snapshot.drawnTiles[event.seat] = null;
      snapshot.discards[event.seat].push({ tile: event.tile, tsumogiri: event.tsumogiri, calledBy: null, riichi: false });
      return;
    case 'Call':
      applyCall(snapshot, event);
      return;
    case 'Riichi':
      snapshot.scores[event.seat] -= 1_000;
      snapshot.riichiSticks += 1;
      if (snapshot.discards[event.seat].length > 0) snapshot.discards[event.seat].at(-1)!.riichi = true;
      return;
    case 'NewDora':
      snapshot.doraIndicators.push(event.tile);
  }
}

function applyCall(snapshot: TableSnapshot, event: Extract<TurnEvent, { type: 'Call' }>): void {
  snapshot.drawnTiles[event.seat] = null;
  const sourceDiscards = snapshot.discards[event.from_seat];
  let calledTile: number | null = null;
  for (let index = sourceDiscards.length - 1; index >= 0; index -= 1) {
    const discard = sourceDiscards[index];
    if (discard.calledBy === null && event.tiles.includes(discard.tile)) {
      discard.calledBy = event.seat;
      calledTile = discard.tile;
      break;
    }
  }

  if (event.kind === 'addedKan') {
    const tileType = Math.floor(event.tiles[0] / 4);
    const meld = snapshot.melds[event.seat].find(
      (candidate) => candidate.kind === 'pon' && Math.floor(candidate.tiles[0] / 4) === tileType
    );
    const addedTile = event.tiles.find((tile) => !meld?.tiles.includes(tile));
    if (addedTile !== undefined) removeTile(snapshot.hands[event.seat], addedTile);
    if (meld) {
      meld.kind = 'addedKan';
      meld.tiles = [...event.tiles];
      return;
    }
  }

  const ownedTiles = event.kind === 'closedKan' || event.kind === 'nuki'
    ? event.tiles
    : event.tiles.filter((tile) => tile !== calledTile);
  for (const tile of ownedTiles) removeTile(snapshot.hands[event.seat], tile);
  snapshot.melds[event.seat].push({
    kind: event.kind,
    tiles: [...event.tiles],
    fromSeat: event.from_seat,
    calledTile
  });
}

function cloneSnapshot(previous: TableSnapshot, turnIndex: number, event: TurnEvent | null): TableSnapshot {
  return {
    turnIndex,
    hands: previous.hands.map((hand) => [...hand]),
    drawnTiles: [...previous.drawnTiles],
    discards: previous.discards.map((pile) => pile.map((discard) => ({ ...discard }))),
    melds: previous.melds.map((melds) => melds.map((meld) => ({ ...meld, tiles: [...meld.tiles] }))),
    doraIndicators: [...previous.doraIndicators],
    scores: [...previous.scores],
    remainingWall: previous.remainingWall,
    riichiSticks: previous.riichiSticks,
    event,
    settled: false
  };
}

function removeTile(hand: number[], tile: number): void {
  const index = hand.indexOf(tile);
  if (index >= 0) hand.splice(index, 1);
}

function fourSeats<T, U>(values: T[], map: (value: T, seat: number) => U): U[] {
  return Array.from({ length: 4 }, (_, seat) => map(values[seat], seat));
}
