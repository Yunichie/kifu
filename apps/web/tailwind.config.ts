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
          text: 'var(--tile-text)',
          edge: 'var(--tile-edge)',
          back: 'var(--tile-back)'
        }
      },
      fontFamily: {
        display: ['"Zen Old Mincho"', '"Noto Serif JP"', 'serif'],
        sans: ['"BIZ UDPGothic"', '"Noto Sans JP"', '-apple-system', 'sans-serif'],
        mono: ['"Roboto Mono"', '"BIZ UDPGothic"', 'ui-monospace', 'monospace']
      },
      borderRadius: {
        sm: '4px',
        md: '6px',
        lg: '8px'
      },
      transitionDuration: {
        fast: 'var(--motion-fast)',
        base: 'var(--motion-base)',
        slow: 'var(--motion-slow)'
      }
    }
  }
} satisfies Config;
