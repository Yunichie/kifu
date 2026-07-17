import { describe, expect, test } from 'bun:test';
import {
  googleAuthorizationUrl,
  pkceChallenge,
  randomToken,
  validatedExchangeInput
} from './oauth';

describe('OAuth PKCE helpers', () => {
  test('creates URL-safe 256-bit random tokens', () => {
    const first = randomToken();
    const second = randomToken();

    expect(first).toMatch(/^[A-Za-z0-9_-]{43}$/);
    expect(second).not.toBe(first);
  });

  test('matches the RFC 7636 S256 example', async () => {
    const verifier = 'dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk';
    expect(await pkceChallenge(verifier)).toBe('E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM');
  });

  test('builds the minimal Google authorization request', () => {
    const url = new URL(
      googleAuthorizationUrl('client-id', 'https://kifu.example/auth/google/callback', 'state', 'challenge')
    );
    expect(url.origin + url.pathname).toBe('https://accounts.google.com/o/oauth2/v2/auth');
    expect(url.searchParams.get('scope')).toBe('openid profile');
    expect(url.searchParams.get('code_challenge_method')).toBe('S256');
    expect(url.searchParams.get('redirect_uri')).toBe(
      'https://kifu.example/auth/google/callback'
    );
  });

  test('rejects expired or mismatched state before code exchange', () => {
    expect(validatedExchangeInput(undefined, 'state', 'code', 'verifier')).toBeNull();
    expect(validatedExchangeInput('expected', 'other', 'code', 'verifier')).toBeNull();
    expect(validatedExchangeInput('state', 'state', 'code', 'verifier')).toEqual({
      code: 'code',
      codeVerifier: 'verifier'
    });
  });
});
