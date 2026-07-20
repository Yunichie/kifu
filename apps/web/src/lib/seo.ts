export type SitemapEntry = {
  loc: string;
  lastmod?: string;
};

export function canonicalOrigin(configured: string | undefined, fallback: string): string {
  try {
    return new URL(configured || fallback).origin;
  } catch {
    return new URL(fallback).origin;
  }
}

export function homeCanonical(origin: string, page = 1): string {
  const url = new URL('/', origin);
  if (page > 1) url.searchParams.set('page', String(page));
  return url.toString();
}

export function careerCanonical(origin: string, name?: string): string {
  return new URL(name ? `/career/${encodeURIComponent(name)}` : '/career', origin).toString();
}

export function gameCanonical(origin: string, logId: string, ownerId: number): string {
  const url = new URL(`/games/${encodeURIComponent(logId)}`, origin);
  url.searchParams.set('owner', String(ownerId));
  return url.toString();
}

export function renderSitemap(entries: SitemapEntry[]): string {
  const urls = entries
    .map(({ loc, lastmod }) => {
      const modified = lastmod ? `\n    <lastmod>${escapeXml(lastmod)}</lastmod>` : '';
      return `  <url>\n    <loc>${escapeXml(loc)}</loc>${modified}\n  </url>`;
    })
    .join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;
}

export function escapeXml(value: string): string {
  return value.replace(/[&<>"']/g, (character) => {
    const entities: Record<string, string> = {
      '&': '&amp;',
      '<': '&lt;',
      '>': '&gt;',
      '"': '&quot;',
      "'": '&apos;'
    };
    return entities[character] ?? character;
  });
}
