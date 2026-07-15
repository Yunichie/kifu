export function tileGlyph(tile: number): string {
  const type = Math.floor(tile / 4);
  if (type < 9) return String.fromCodePoint(0x1f007 + type);
  if (type < 18) return String.fromCodePoint(0x1f019 + type - 9);
  if (type < 27) return String.fromCodePoint(0x1f010 + type - 18);
  return String.fromCodePoint([0x1f000, 0x1f001, 0x1f002, 0x1f003, 0x1f006, 0x1f005, 0x1f004][type - 27]);
}

export function tileLabel(tile: number): string {
  const type = Math.floor(tile / 4);
  if (type < 9) return `${type + 1}m`;
  if (type < 18) return `${type - 8}p`;
  if (type < 27) return `${type - 17}s`;
  return ['East', 'South', 'West', 'North', 'White', 'Green', 'Red'][type - 27] ?? 'Unknown tile';
}
