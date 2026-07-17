import { env } from '$env/dynamic/public';
import { redirect } from '@sveltejs/kit';
import { googleAuthorizationUrl, pkceChallenge, randomToken } from '$lib/server/oauth';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ cookies, url }) => {
  const clientId = env.PUBLIC_GOOGLE_CLIENT_ID;
  if (!clientId) redirect(303, '/login?error=configuration');

  const state = randomToken();
  const verifier = randomToken();
  const redirectUri = new URL('/auth/google/callback', url).toString();
  const cookieOptions = {
    path: '/auth/google',
    httpOnly: true,
    sameSite: 'lax' as const,
    secure: url.protocol === 'https:',
    maxAge: 10 * 60
  };
  cookies.set('kifu_oauth_state', state, cookieOptions);
  cookies.set('kifu_oauth_verifier', verifier, cookieOptions);

  redirect(
    302,
    googleAuthorizationUrl(clientId, redirectUri, state, await pkceChallenge(verifier))
  );
};
