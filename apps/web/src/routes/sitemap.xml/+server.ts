import type { GameListPage } from '@kifu/api-types';
import { error } from '@sveltejs/kit';
import { env } from '$env/dynamic/public';
import { apiRequest } from '$lib/api/client';
import { hasCareer } from '$lib/player';
import {
  canonicalOrigin,
  careerCanonical,
  gameCanonical,
  homeCanonical,
  renderSitemap,
  type SitemapEntry
} from '$lib/seo';
import type { RequestHandler } from './$types';

const MAX_SITEMAP_URLS = 50_000;

export const GET: RequestHandler = async (event) => {
  const siteUrl = canonicalOrigin(env.PUBLIC_SITE_URL, event.url.origin);
  const entries = new Map<string, SitemapEntry>();
  addEntry(entries, { loc: homeCanonical(siteUrl) });
  addEntry(entries, { loc: careerCanonical(siteUrl) });

  for (let page = 1; ; page += 1) {
    const games = await apiRequest<GameListPage>(event, `/api/public-games?page=${page}`);
    if (page > 1 && games.items.length > 0) {
      addEntry(entries, { loc: homeCanonical(siteUrl, page) });
    }

    for (const game of games.items) {
      addEntry(entries, {
        loc: gameCanonical(siteUrl, game.logId, game.owner.userId),
        lastmod: new Date(game.addedAt).toISOString()
      });
      for (const player of game.players) {
        if (hasCareer(player.name)) {
          addEntry(entries, { loc: careerCanonical(siteUrl, player.name) });
        }
      }
    }

    if (entries.size > MAX_SITEMAP_URLS) error(503, 'Sitemap capacity exceeded');
    if (!games.hasNext) break;
  }

  return new Response(renderSitemap([...entries.values()]), {
    headers: {
      'cache-control': 'public, max-age=300',
      'content-type': 'application/xml; charset=utf-8'
    }
  });
};

function addEntry(entries: Map<string, SitemapEntry>, entry: SitemapEntry): void {
  if (!entries.has(entry.loc)) entries.set(entry.loc, entry);
}
