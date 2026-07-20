import type { MeResponse } from '@kifu/api-types';
import { env } from '$env/dynamic/public';
import { ApiError, SESSION_COOKIE, apiRequest } from '$lib/api/client';
import { canonicalOrigin } from '$lib/seo';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
  const siteUrl = canonicalOrigin(env.PUBLIC_SITE_URL, event.url.origin);
  event.setHeaders({ vary: 'cookie' });
  if (!event.cookies.get(SESSION_COOKIE)) return { me: null, siteUrl };

  event.setHeaders({ 'cache-control': 'private, no-store' });

  try {
    return { me: await apiRequest<MeResponse>(event, '/api/me'), siteUrl };
  } catch (error) {
    if (error instanceof ApiError && error.status === 401) return { me: null, siteUrl };
    throw error;
  }
};
