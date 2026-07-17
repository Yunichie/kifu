# kifu

Tenhou riichi mahjong statistics tracker.

## Local development

Use `apps/api/.dev.vars.example` and `apps/web/.env.example` as the local environment templates. The Google client ID must match in both files, and `GOOGLE_REDIRECT_URI` must exactly match the authorized redirect URI.

```bash
bun install
bun run build:api
wrangler d1 migrations apply kifu --local --cwd apps/api --config wrangler.jsonc
bun run dev:api
bun run dev:web
```

The web app runs at `http://localhost:5173` and the API at `http://127.0.0.1:8787`.
