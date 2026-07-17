import type { AddGameInput, GameDetail, GameListPage, PlayerSearchPage } from '@kifu/api-types';
import { fail, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import { pageParam, playerQuery } from '$lib/query';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
  const { me } = await event.parent();
  const view = me && event.url.searchParams.get('view') !== 'all' ? 'library' : 'all';
  const page = pageParam(event.url.searchParams.get('page'));
  const query = playerQuery(event.url.searchParams.get('q'));
  const playerPage = pageParam(event.url.searchParams.get('playerPage'));
  const gamePath = view === 'library' ? '/api/me/library' : '/api/games';
  const [games, playerResults] = await Promise.all([
    apiRequest<GameListPage>(event, `${gamePath}?page=${page}`),
    apiRequest<PlayerSearchPage>(
      event,
      `/api/players?${new URLSearchParams({ q: query, page: String(playerPage) })}`
    )
  ]);
  return { games, playerResults, query, view };
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
