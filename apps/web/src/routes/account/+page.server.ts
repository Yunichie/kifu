import type { PlayerNamesResponse } from '@kifu/api-types';
import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ parent }) => {
  const { me } = await parent();
  if (!me) redirect(303, '/login');
  return { me };
};

export const actions = {
  addName: async (event) => {
    const formData = await event.request.formData();
    const name = String(formData.get('name') ?? '').trim();
    try {
      await apiRequest<PlayerNamesResponse>(event, '/api/me/player-names', {
        method: 'POST',
        body: { name }
      });
    } catch (error) {
      if (error instanceof ApiError) return fail(error.status, { message: error.message, name });
      throw error;
    }
    redirect(303, '/account');
  },
  removeName: async (event) => {
    const formData = await event.request.formData();
    const name = String(formData.get('name') ?? '');
    try {
      await apiRequest<PlayerNamesResponse>(
        event,
        `/api/me/player-names/${encodeURIComponent(name)}`,
        { method: 'DELETE' }
      );
    } catch (error) {
      if (error instanceof ApiError) return fail(error.status, { message: error.message });
      throw error;
    }
    redirect(303, '/account');
  },
  logout: async (event) => {
    try {
      await apiRequest<void>(event, '/api/auth/logout', { method: 'POST' });
    } catch (error) {
      if (!(error instanceof ApiError && error.status === 401)) throw error;
    }
    redirect(303, '/');
  }
} satisfies Actions;
