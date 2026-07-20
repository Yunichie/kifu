import type { CareerStats } from '@kifu/api-types';
import { error } from '@sveltejs/kit';
import { apiRequest } from '$lib/api/client';
import { hasCareer } from '$lib/player';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
  if (!hasCareer(event.params.name)) error(404, 'Player not found');

  return {
    career: await apiRequest<CareerStats>(
      event,
      `/api/players/${encodeURIComponent(event.params.name)}/career`
    )
  };
};
