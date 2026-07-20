import { describe, expect, test } from 'bun:test';
import { hasCareer } from './player';

describe('hasCareer', () => {
  test('excludes only the exact Tenhou guest name', () => {
    expect(hasCareer('NoName')).toBe(false);
    expect(hasCareer('noname')).toBe(true);
    expect(hasCareer('NoName2')).toBe(true);
    expect(hasCareer('CLS')).toBe(true);
  });
});
