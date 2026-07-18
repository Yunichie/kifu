export type Theme = 'light' | 'dark';

export const THEME_STORAGE_KEY = 'kifu-theme';

type ThemeRoot = {
  dataset: Record<string, string | undefined>;
  style: { colorScheme: string };
};

type ThemeStorage = {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
};

type ThemeMeta = {
  content: string;
  dataset: Record<string, string | undefined>;
};

type ThemeOptions = {
  root?: ThemeRoot;
  storage?: ThemeStorage | null;
  meta?: ThemeMeta | null;
  prefersDark?: boolean;
};

export function isTheme(value: unknown): value is Theme {
  return value === 'light' || value === 'dark';
}

export function resolveTheme(stored: unknown, prefersDark: boolean): Theme {
  return isTheme(stored) ? stored : prefersDark ? 'dark' : 'light';
}

export function readStoredTheme(storage: ThemeStorage | null = browserStorage()): Theme | null {
  if (!storage) return null;
  try {
    const value = storage.getItem(THEME_STORAGE_KEY);
    return isTheme(value) ? value : null;
  } catch {
    return null;
  }
}

export function readAppliedTheme(
  root: ThemeRoot = browserRoot(),
  prefersDark = browserPrefersDark()
): Theme {
  return resolveTheme(root.dataset.theme, prefersDark);
}

export function applyTheme(
  theme: Theme,
  { root = browserRoot(), storage = browserStorage(), meta = browserMeta() }: ThemeOptions = {},
  persist = true
): Theme {
  root.dataset.theme = theme;
  root.dataset.themeSource = persist ? 'user' : 'system';
  root.style.colorScheme = theme;

  const themeColor = meta?.dataset[theme];
  if (meta && themeColor) meta.content = themeColor;

  if (persist && storage) {
    try {
      storage.setItem(THEME_STORAGE_KEY, theme);
    } catch {}
  }
  return theme;
}

export function toggleTheme(options: ThemeOptions = {}): Theme {
  const root = options.root ?? browserRoot();
  const current = readAppliedTheme(root, options.prefersDark ?? browserPrefersDark());
  return applyTheme(current === 'dark' ? 'light' : 'dark', { ...options, root }, true);
}

export function watchSystemTheme(onChange: (theme: Theme) => void): () => void {
  const query = window.matchMedia('(prefers-color-scheme: dark)');
  const handleChange = (event: MediaQueryListEvent): void => {
    if (browserRoot().dataset.themeSource !== 'system') return;
    onChange(applyTheme(event.matches ? 'dark' : 'light', {}, false));
  };
  query.addEventListener('change', handleChange);
  return () => query.removeEventListener('change', handleChange);
}

function browserRoot(): ThemeRoot {
  return document.documentElement;
}

function browserStorage(): ThemeStorage | null {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
}

function browserMeta(): ThemeMeta | null {
  return document.querySelector<HTMLMetaElement>('meta[name="theme-color"]:not([media])');
}

function browserPrefersDark(): boolean {
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}
