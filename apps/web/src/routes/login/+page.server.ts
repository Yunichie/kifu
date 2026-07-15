import type { AuthResponse } from '@kifu/api-types';
import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import type { Actions, PageServerLoad, RequestEvent } from './$types';

export const load: PageServerLoad = async ({ url, parent }) => {
  const { me } = await parent();
  if (me) redirect(303, '/');
  return { mode: url.searchParams.get('mode') === 'signup' ? 'signup' : 'login' };
};

async function authenticate(event: RequestEvent, mode: 'login' | 'signup') {
  const formData = await event.request.formData();
  const username = String(formData.get('username') ?? '').trim();
  const password = String(formData.get('password') ?? '');

  try {
    await apiRequest<AuthResponse>(event, `/api/auth/${mode}`, {
      method: 'POST',
      body: { username, password }
    });
  } catch (error) {
    if (error instanceof ApiError) {
      const message =
        mode === 'login' && error.status === 401
          ? "That username or password doesn't match an account."
          : mode === 'signup' && error.status === 409
            ? 'That username is already in use.'
            : error.message;
      return fail(error.status, { message, mode, username });
    }
    throw error;
  }

  redirect(303, '/');
}

export const actions = {
  login: (event) => authenticate(event, 'login'),
  signup: (event) => authenticate(event, 'signup')
} satisfies Actions;
