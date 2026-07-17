import { describe, expect, test } from 'bun:test';
import { pageParam, playerQuery } from './query';

describe('query parameters', () => {
  test('normalizes page numbers', () => {
    expect(pageParam(null)).toBe(1);
    expect(pageParam('2')).toBe(2);
    expect(pageParam('0')).toBe(1);
    expect(pageParam('2.5')).toBe(1);
  });

  test('normalizes player searches', () => {
    expect(playerQuery('  \u{77f3}\u{6a4b}  ')).toBe('\u{77f3}\u{6a4b}');
    expect(playerQuery('line\nbreak')).toBe('');
    expect(playerQuery('x'.repeat(65))).toBe('');
  });
});
