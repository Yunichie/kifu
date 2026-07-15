import type { GameDetail } from '@kifu/api-types';
import { error } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
  try {
    return {
      game: await apiRequest<GameDetail>(event, `/api/games/${encodeURIComponent(event.params.id)}`)
    };
  } catch (reason) {
    if (reason instanceof ApiError && reason.status === 404) error(404, 'Game not found');
    throw reason;
  }
};
