const MAX_PAGE = 100_000;

export function pageParam(value: string | null): number {
  if (!value || !/^\d+$/.test(value)) return 1;
  const page = Number(value);
  return Number.isSafeInteger(page) && page >= 1 && page <= MAX_PAGE ? page : 1;
}

export function playerQuery(value: string | null): string {
  if (!value) return '';
  const query = value.trim();
  return query.length <= 64 && !/\p{Cc}/u.test(query) ? query : '';
}
