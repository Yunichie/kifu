import type { AddGameInput, GameDetail, GameListItem } from '@kifu/api-types';
import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
  const { me } = await event.parent();
  const view = me && event.url.searchParams.get('view') !== 'all' ? 'library' : 'all';
  const [games, players, library] = await Promise.all([
    apiRequest<GameListItem[]>(event, '/api/games'),
    apiRequest<string[]>(event, '/api/players'),
    me ? apiRequest<GameListItem[]>(event, '/api/me/library') : Promise.resolve([])
  ]);
  return { games, players, library, view };
};

export const actions = {
  add: async (event) => {
    const formData = await event.request.formData();
    const logId = String(formData.get('logId') ?? '').trim();
    const body = { logId } satisfies AddGameInput;
    try {
      const game = await apiRequest<GameDetail>(event, '/api/games', { method: 'POST', body });
      redirect(303, `/games/${game.logId}`);
    } catch (error) {
      if (error instanceof ApiError) {
        if (error.status === 401) redirect(303, '/login');
        return fail(error.status, { message: error.message, logId });
      }
      throw error;
    }
  },
  remove: async (event) => {
    const formData = await event.request.formData();
    const logId = String(formData.get('logId') ?? '');
    try {
      await apiRequest<void>(event, `/api/games/${encodeURIComponent(logId)}`, {
        method: 'DELETE'
      });
    } catch (error) {
      if (error instanceof ApiError) return fail(error.status, { message: error.message });
      throw error;
    }
    redirect(303, '/');
  }
} satisfies Actions;
