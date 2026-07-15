import type { MeResponse } from '@kifu/api-types';
import { ApiError, apiRequest } from '$lib/api/client';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
  try {
    return { me: await apiRequest<MeResponse>(event, '/api/me') };
  } catch (error) {
    if (error instanceof ApiError && error.status === 401) return { me: null };
    throw error;
  }
};
