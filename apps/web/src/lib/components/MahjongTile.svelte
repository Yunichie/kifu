<script lang="ts">
  import { tileGlyph, tileLabel } from '$lib/replay/tiles';

  let {
    tile = 0,
    size = 'hand',
    rotated = false,
    tsumogiri = false,
    caller = null,
    back = false
  }: {
    tile?: number;
    size?: 'hand' | 'compact' | 'discard';
    rotated?: boolean;
    tsumogiri?: boolean;
    caller?: number | null;
    back?: boolean;
  } = $props();

  const sizeClasses = {
    hand: 'h-[39px] w-7 text-[28px]',
    compact: 'h-7 w-5 text-xl',
    discard: 'h-[31px] w-[22px] text-[22px]'
  };
  const callerBorders = ['border-man', 'border-pin', 'border-sou', 'border-gold'];
  let label = $derived(back ? 'Face-down tile' : tileLabel(tile));
  let glyph = $derived(back ? String.fromCodePoint(0x1f02b) : tileGlyph(tile));
</script>

<span
  class={[
    'tile-glyph inline-flex shrink-0 items-center justify-center rounded-sm border bg-tile-bg leading-none text-tile-text transition duration-fast',
    sizeClasses[size],
    caller === null ? 'border-tile-text/10' : (callerBorders[caller] ?? 'border-border-strong'),
    rotated && 'rotate-90 scale-[0.72]',
    tsumogiri && 'underline decoration-text-tertiary underline-offset-2'
  ]}
  role="img"
  aria-label={label}
  title={label}
>
  {glyph}
</span>

<style>
  .tile-glyph {
    font-family: "Segoe UI Symbol", "Noto Sans Symbols 2", sans-serif;
    animation: settle var(--motion-fast) ease-out;
  }

  @keyframes settle {
    from {
      opacity: 0;
      translate: 0 -2px;
    }
  }
</style>
