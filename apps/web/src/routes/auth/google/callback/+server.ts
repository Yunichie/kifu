import type { AuthResponse } from '@kifu/api-types';
import { redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import { validatedExchangeInput } from '$lib/server/oauth';
import type { RequestHandler } from './$types';

const COOKIE_PATH = '/auth/google';

export const GET: RequestHandler = async (event) => {
  const expectedState = event.cookies.get('kifu_oauth_state');
  const verifier = event.cookies.get('kifu_oauth_verifier');
  event.cookies.delete('kifu_oauth_state', { path: COOKIE_PATH });
  event.cookies.delete('kifu_oauth_verifier', { path: COOKIE_PATH });

  if (event.url.searchParams.has('error')) redirect(303, '/login?error=access_denied');

  const body = validatedExchangeInput(
    expectedState,
    event.url.searchParams.get('state'),
    event.url.searchParams.get('code'),
    verifier
  );
  if (!body) redirect(303, '/login?error=invalid_state');

  try {
    await apiRequest<AuthResponse>(event, '/api/auth/google', { method: 'POST', body });
  } catch (error) {
    if (error instanceof ApiError) redirect(303, '/login?error=exchange_failed');
    throw error;
  }

  redirect(303, '/');
};
