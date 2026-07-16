import type { MeResponse } from '@kifu/api-types';
import { ApiError, SESSION_COOKIE, apiRequest } from '$lib/api/client';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
  if (!event.cookies.get(SESSION_COOKIE)) return { me: null };

  try {
    return { me: await apiRequest<MeResponse>(event, '/api/me') };
  } catch (error) {
    if (error instanceof ApiError && error.status === 401) return { me: null };
    throw error;
  }
};
