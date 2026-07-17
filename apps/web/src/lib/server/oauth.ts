import type { OAuthExchangeInput } from '@kifu/api-types';

const TOKEN_BYTES = 32;

export function randomToken(): string {
  const bytes = new Uint8Array(TOKEN_BYTES);
  crypto.getRandomValues(bytes);
  return base64Url(bytes);
}

export async function pkceChallenge(verifier: string): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(verifier));
  return base64Url(new Uint8Array(digest));
}

export function googleAuthorizationUrl(
  clientId: string,
  redirectUri: string,
  state: string,
  challenge: string
): string {
  const url = new URL('https://accounts.google.com/o/oauth2/v2/auth');
  url.search = new URLSearchParams({
    client_id: clientId,
    redirect_uri: redirectUri,
    response_type: 'code',
    scope: 'openid profile',
    state,
    code_challenge: challenge,
    code_challenge_method: 'S256'
  }).toString();
  return url.toString();
}

export function validatedExchangeInput(
  expectedState: string | undefined,
  actualState: string | null,
  code: string | null,
  verifier: string | undefined
): OAuthExchangeInput | null {
  if (!expectedState || !actualState || actualState !== expectedState || !code || !verifier) {
    return null;
  }
  return { code, codeVerifier: verifier };
}

function base64Url(bytes: Uint8Array): string {
  let binary = '';
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}
