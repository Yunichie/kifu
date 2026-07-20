import type { RequestHandler } from './$types';

const verification = 'google-site-verification: googled013cdf22dfee669.html';

export const GET: RequestHandler = () =>
  new Response(`${verification}\n`, {
    headers: {
      'cache-control': 'public, max-age=3600',
      'content-type': 'text/html; charset=utf-8'
    }
  });
