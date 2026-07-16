import type { ErrorResponse } from '@kifu/api-types';
import { env } from '$env/dynamic/public';
import type { RequestEvent } from '@sveltejs/kit';

export const SESSION_COOKIE = 'kifu_session';
const DEFAULT_API_BASE_URL = 'http://localhost:8787';

type ApiInit = Omit<RequestInit, 'body'> & {
  body?: unknown;
};

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    message: string
  ) {
    super(message);
  }
}

export async function apiRequest<T>(
  event: RequestEvent,
  path: string,
  init: ApiInit = {}
): Promise<T> {
  const headers = new Headers(init.headers);
  const session = event.cookies.get(SESSION_COOKIE);
  if (session) headers.set('cookie', `${SESSION_COOKIE}=${session}`);
  if (init.body !== undefined) headers.set('content-type', 'application/json');

  const baseUrl = (env.PUBLIC_API_BASE_URL || DEFAULT_API_BASE_URL).replace(/\/$/, '');
  const response = await event.fetch(`${baseUrl}${path}`, {
    ...init,
    headers,
    body: init.body === undefined ? undefined : JSON.stringify(init.body)
  });

  relaySessionCookie(event, response.headers.get('set-cookie'));

  if (!response.ok) {
    let message = `Request failed with status ${response.status}`;
    try {
      const payload = (await response.json()) as ErrorResponse;
      if (payload.error) message = payload.error;
    } catch {
      // The status remains useful when an upstream proxy returns a non-JSON body.
    }
    if (response.status === 401) event.cookies.delete(SESSION_COOKIE, { path: '/' });
    throw new ApiError(response.status, message);
  }

  if (response.status === 204) return undefined as T;
  return (await response.json()) as T;
}

function relaySessionCookie(event: RequestEvent, header: string | null): void {
  if (!header) return;
  const value = header.match(/^kifu_session=([^;]*)/i)?.[1];
  if (value === undefined) return;

  const maxAge = Number(header.match(/Max-Age=(\d+)/i)?.[1] ?? 0);
  if (!value || maxAge === 0) {
    event.cookies.delete(SESSION_COOKIE, { path: '/' });
    return;
  }

  event.cookies.set(SESSION_COOKIE, value, {
    path: '/',
    httpOnly: true,
    sameSite: 'lax',
    secure: event.url.protocol === 'https:',
    maxAge
  });
}
