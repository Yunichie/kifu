import type { CareerStats } from '@kifu/api-types';
import { apiRequest } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async (event) => ({
  career: await apiRequest<CareerStats>(
    event,
    `/api/players/${encodeURIComponent(event.params.name)}/career`
  )
});
