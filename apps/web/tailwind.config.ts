import type { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        surface: {
          0: 'var(--surface-0)',
          1: 'var(--surface-1)',
          2: 'var(--surface-2)',
          3: 'var(--surface-3)'
        },
        border: {
          subtle: 'var(--border-subtle)',
          DEFAULT: 'var(--border-default)',
          strong: 'var(--border-strong)'
        },
        text: {
          primary: 'var(--text-primary)',
          secondary: 'var(--text-secondary)',
          tertiary: 'var(--text-tertiary)',
          disabled: 'var(--text-disabled)'
        },
        man: 'var(--man)',
        pin: 'var(--pin)',
        sou: 'var(--sou)',
        gold: 'var(--gold)',
        'gold-ink': 'var(--gold-ink)',
        tile: {
          bg: 'var(--tile-bg)',
          text: 'var(--tile-text)'
        }
      },
      fontFamily: {
        display: ['"IBM Plex Slab"', 'serif'],
        sans: ['"IBM Plex Sans"', '-apple-system', 'sans-serif'],
        mono: ['"IBM Plex Mono"', 'ui-monospace', 'monospace']
      },
      borderRadius: {
        sm: '6px',
        md: '10px',
        lg: '16px'
      },
      transitionDuration: {
        fast: '120ms',
        base: '180ms',
        slow: '260ms'
      }
    }
  }
} satisfies Config;
