import type { CareerStats } from '@kifu/api-types';
import { error } from '@sveltejs/kit';
import { ApiError, apiRequest } from '$lib/api/client';
import { hasCareer } from '$lib/player';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
  if (!hasCareer(event.params.name)) error(404, 'Player not found');

  try {
    const career = await apiRequest<CareerStats>(
      event,
      `/api/players/${encodeURIComponent(event.params.name)}/career`
    );
    if (career.games === 0) error(404, 'Player not found');
    return { career };
  } catch (reason) {
    if (reason instanceof ApiError && (reason.status === 400 || reason.status === 404)) {
      error(404, 'Player not found');
    }
    throw reason;
  }
};
