import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, parent }) => {
  const { me } = await parent();
  if (me) redirect(303, '/');
  const error = url.searchParams.get('error');
  const message =
    error === 'access_denied'
      ? 'Google sign-in was cancelled.'
      : error === 'invalid_state'
        ? 'That sign-in request expired. Please try again.'
        : error === 'exchange_failed'
          ? 'Google sign-in could not be completed. Please try again.'
          : error === 'configuration'
            ? 'Google sign-in is not configured.'
            : null;
  return { message };
};
