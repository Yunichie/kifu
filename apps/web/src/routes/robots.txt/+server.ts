import { env } from '$env/dynamic/public';
import { canonicalOrigin } from '$lib/seo';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = ({ url }) => {
  const siteUrl = canonicalOrigin(env.PUBLIC_SITE_URL, url.origin);
  const body = [
    'User-agent: *',
    'Content-signal: search=yes, ai-train=no, use=reference',
    'Allow: /',
    `Sitemap: ${new URL('/sitemap.xml', siteUrl)}`,
    ''
  ].join('\n');

  return new Response(body, {
    headers: {
      'cache-control': 'public, max-age=3600',
      'content-type': 'text/plain; charset=utf-8'
    }
  });
};
