export type TileSuit = 'man' | 'pin' | 'sou' | 'honor';

export function tileSuit(tile: number): TileSuit {
  const type = Math.floor(tile / 4);
  if (type < 9) return 'man';
  if (type < 18) return 'pin';
  if (type < 27) return 'sou';
  return 'honor';
}

export function tileRank(tile: number): number {
  const type = Math.floor(tile / 4);
  return type < 27 ? (type % 9) + 1 : type - 27;
}

export function isRedFive(tile: number): boolean {
  return tile === 16 || tile === 52 || tile === 88;
}

export function tileLabel(tile: number): string {
  const suit = tileSuit(tile);
  const rank = tileRank(tile);
  if (suit !== 'honor') {
    const suitName = { man: 'characters', pin: 'circles', sou: 'bamboo' }[suit];
    return `${isRedFive(tile) ? 'Red ' : ''}${rank} of ${suitName}`;
  }
  return ['East wind', 'South wind', 'West wind', 'North wind', 'White dragon', 'Green dragon', 'Red dragon'][rank] ?? 'Unknown tile';
}
