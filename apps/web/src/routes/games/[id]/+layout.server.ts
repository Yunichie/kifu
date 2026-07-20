import type { GameDetail, PublicGameDetail } from '@kifu/api-types';
import { error, redirect } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
  try {
    const owner = event.url.searchParams.get('owner');
    if (owner !== null) {
      const ownerId = Number(owner);
      if (!Number.isSafeInteger(ownerId) || ownerId <= 0) error(404, 'Game not found');
      const shared = await apiRequest<PublicGameDetail>(
        event,
        `/api/public-games/${ownerId}/${encodeURIComponent(event.params.id)}`
      );
      return { game: shared.game, publicOwner: shared.owner };
    }
    return {
      game: await apiRequest<GameDetail>(event, `/api/games/${encodeURIComponent(event.params.id)}`),
      publicOwner: null
    };
  } catch (reason) {
    if (reason instanceof ApiError && (reason.status === 400 || reason.status === 404)) {
      error(404, 'Game not found');
    }
    if (reason instanceof ApiError && reason.status === 401) redirect(303, '/login');
    throw reason;
  }
};
