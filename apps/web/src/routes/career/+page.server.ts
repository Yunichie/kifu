import type { CareerStats } from '@kifu/api-types';
import { apiRequest } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
  const { me } = await event.parent();
  const players = await apiRequest<string[]>(event, '/api/players');
  return {
    players,
    career: me ? await apiRequest<CareerStats>(event, '/api/me/career') : null
  };
};
