<script lang="ts">
  import { isRedFive, tileLabel, tileRank, tileSuit } from '$lib/replay/tiles';

  let {
    tile,
    size = 'hand',
    rotated = false,
    highlighted = false,
    dimmed = false,
    indicator = false
  }: {
    tile: number;
    size?: 'hand' | 'compact' | 'discard' | 'dora';
    rotated?: boolean;
    highlighted?: boolean;
    dimmed?: boolean;
    indicator?: boolean;
  } = $props();

  const honorGlyphs = ['東', '南', '西', '北', '', '發', '中'];
  const pinPatterns = [
    [4], [0, 8], [0, 4, 8], [0, 2, 6, 8], [0, 2, 4, 6, 8],
    [0, 2, 3, 5, 6, 8], [0, 2, 3, 4, 5, 6, 8], [0, 1, 2, 3, 5, 6, 7, 8],
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
  ];
  let suit = $derived(tileSuit(tile));
  let rank = $derived(tileRank(tile));
  let red = $derived(isRedFive(tile));
  let label = $derived(tileLabel(tile));
</script>

<span
  class={['tile-slot', `size-${size}`, rotated && 'is-rotated', highlighted && 'is-highlighted', dimmed && 'is-dimmed']}
  role="img"
  aria-label={label}
  title={label}
>
  <span class={['tile-face', `suit-${suit}`, red && 'is-red']}>
    {#if suit === 'man'}
      <span class="man-rank">{rank}</span>
      <span class="man-unit" lang="ja">萬</span>
    {:else if suit === 'pin'}
      <span class="pin-grid" aria-hidden="true">
        {#each Array.from({ length: 9 }) as _, index (index)}
          {#if pinPatterns[rank - 1].includes(index)}
            <i class:aka={red && index === 4} class:center={index === 4}></i>
          {:else}<i></i>{/if}
        {/each}
      </span>
    {:else if suit === 'sou'}
      {#if rank === 1}
        <span class="sou-bird" aria-hidden="true"><i></i><b></b></span>
      {:else}
        <span class="sou-grid" aria-hidden="true">
          {#each Array.from({ length: rank }) as _, index (index)}<i class:aka={red && index === Math.floor(rank / 2)}></i>{/each}
        </span>
      {/if}
    {:else if rank === 4}
      <span class="white-dragon" aria-hidden="true"></span>
    {:else}
      <span class={['honor-glyph', rank === 5 && 'green', rank === 6 && 'red']} lang="ja">{honorGlyphs[rank]}</span>
    {/if}
    {#if indicator}<span class="indicator-dot" aria-hidden="true"></span>{/if}
  </span>
</span>

<style>
  .tile-slot {
    --tile-w: 31px;
    --tile-h: 44px;
    display: inline-flex;
    width: var(--tile-w);
    height: var(--tile-h);
    flex: 0 0 auto;
    align-items: center;
    justify-content: center;
    transition: filter var(--motion-fast), translate var(--motion-fast);
  }

  .size-compact { --tile-w: 23px; --tile-h: 33px; }
  .size-discard { --tile-w: 22px; --tile-h: 31px; }
  .size-dora { --tile-w: 20px; --tile-h: 29px; }

  .tile-face {
    position: relative;
    display: flex;
    width: var(--tile-w);
    height: var(--tile-h);
    align-items: center;
    justify-content: center;
    overflow: hidden;
    border: 1px solid color-mix(in srgb, var(--tile-text) 18%, transparent);
    border-radius: 4px;
    background: linear-gradient(180deg, var(--tile-highlight) 0%, var(--tile-bg) 78%, var(--tile-bottom) 100%);
    color: var(--tile-text);
    box-shadow: inset 0 1px 0 var(--tile-highlight), inset 0 -2px 0 color-mix(in srgb, var(--wood) 14%, transparent), 0 3px 0 color-mix(in srgb, var(--sumi) 80%, transparent), 0 5px 7px color-mix(in srgb, var(--sumi) 45%, transparent);
    transform-origin: center;
    transition: box-shadow var(--motion-fast), transform var(--motion-fast), filter var(--motion-fast);
    animation: settle var(--motion-base) ease-out;
  }

  .is-rotated {
    width: var(--tile-h);
    height: var(--tile-h);
  }

  .is-rotated .tile-face { transform: rotate(90deg); }
  .is-highlighted { translate: 0 -4px; filter: drop-shadow(0 0 7px color-mix(in srgb, var(--gold) 76%, transparent)); }
  .is-highlighted .tile-face { border-color: var(--gold); box-shadow: inset 0 1px 0 var(--tile-highlight), inset 0 -2px 0 color-mix(in srgb, var(--wood) 12%, transparent), 0 4px 0 color-mix(in srgb, var(--gold) 42%, var(--sumi)), 0 7px 12px color-mix(in srgb, var(--sumi) 66%, transparent); }
  .is-dimmed { opacity: 0.18; }

  .man-rank {
    position: absolute;
    top: 3px;
    color: var(--tile-ink);
    font-family: "Shippori Mincho", serif;
    font-size: calc(var(--tile-w) * 0.58);
    font-weight: 700;
    line-height: 1;
  }

  .man-unit {
    position: absolute;
    bottom: 2px;
    color: var(--shu);
    font-family: "Shippori Mincho", serif;
    font-size: calc(var(--tile-w) * 0.55);
    font-weight: 700;
    line-height: 1;
  }

  .is-red .man-rank { color: var(--shu); }

  .pin-grid {
    display: grid;
    width: 78%;
    height: 80%;
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(3, 1fr);
    align-items: center;
    justify-items: center;
  }

  .pin-grid i:not(:empty), .pin-grid i { width: 68%; aspect-ratio: 1; }
  .pin-grid i[class] {
    border: max(1px, calc(var(--tile-w) * 0.07)) solid var(--ai);
    border-radius: 50%;
    background: color-mix(in srgb, var(--ai) 17%, transparent);
    box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--take) 62%, transparent);
  }
  .pin-grid i.center { border-color: var(--shu); box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--shu) 50%, transparent); }
  .pin-grid i.aka { border-color: var(--shu); background: color-mix(in srgb, var(--shu) 24%, transparent); }

  .sou-grid {
    display: grid;
    width: 70%;
    height: 82%;
    grid-template-columns: repeat(3, 1fr);
    align-content: center;
    gap: 1px;
  }

  .sou-grid i {
    position: relative;
    width: 3px;
    height: calc(var(--tile-h) * 0.18);
    justify-self: center;
    border-radius: 2px;
    background: var(--take);
    transform: rotate(8deg);
    box-shadow: 1px 0 0 color-mix(in srgb, var(--take) 52%, var(--sumi));
  }

  .sou-grid i::after {
    position: absolute;
    top: 42%;
    left: -2px;
    width: 7px;
    height: 1px;
    background: var(--ai);
    content: '';
  }

  .sou-grid i.aka { background: var(--shu); box-shadow: 1px 0 0 color-mix(in srgb, var(--shu) 52%, var(--sumi)); }

  .sou-bird { position: relative; width: 70%; height: 70%; }
  .sou-bird i { position: absolute; left: 40%; top: 25%; width: 24%; height: 42%; border-radius: 50% 50% 35% 35%; background: var(--take); transform: rotate(-12deg); }
  .sou-bird i::before, .sou-bird i::after { position: absolute; top: 16%; width: 90%; height: 35%; border: 2px solid var(--ai); border-bottom: 0; border-radius: 80% 80% 0 0; content: ''; }
  .sou-bird i::before { right: 62%; transform: rotate(-28deg); }
  .sou-bird i::after { left: 62%; transform: rotate(28deg); }
  .sou-bird b { position: absolute; left: 21%; bottom: 13%; width: 60%; height: 2px; background: var(--shu); transform: rotate(-9deg); }

  .honor-glyph {
    font-family: "Shippori Mincho", serif;
    font-size: calc(var(--tile-w) * 0.78);
    font-weight: 700;
    line-height: 1;
  }
  .honor-glyph.green { color: var(--take); }
  .honor-glyph.red { color: var(--shu); }
  .white-dragon { width: 62%; height: 66%; border: 2px double var(--ai); border-radius: 2px; }

  .indicator-dot { position: absolute; right: 2px; bottom: 2px; width: 4px; height: 4px; border-radius: 50%; background: var(--gold); }

  @keyframes settle {
    from { opacity: 0; translate: 0 -3px; }
  }
</style>
