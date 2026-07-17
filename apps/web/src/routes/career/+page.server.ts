import type { CareerStats, PlayerSearchPage } from '@kifu/api-types';
import { apiRequest } from '$lib/api/client';
import { pageParam, playerQuery } from '$lib/query';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => {
  const { me } = await event.parent();
  const query = playerQuery(event.url.searchParams.get('q'));
  const page = pageParam(event.url.searchParams.get('page'));
  return me
    ? {
        query,
        playerResults: null,
        career: await apiRequest<CareerStats>(event, '/api/me/career')
      }
    : {
        query,
        playerResults: await apiRequest<PlayerSearchPage>(
          event,
          `/api/players?${new URLSearchParams({ q: query, page: String(page) })}`
        ),
        career: null
      };
};
