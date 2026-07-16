<script lang="ts">
  let { size = 'hand' }: { size?: 'hand' | 'compact' | 'discard' | 'dora' } = $props();
  const sizes = {
    hand: 'h-11 w-[31px]',
    compact: 'h-[33px] w-[23px]',
    discard: 'h-[31px] w-[22px]',
    dora: 'h-[29px] w-5'
  };
</script>

<span class={['tile-back', sizes[size]]} role="img" aria-label="Face-down tile" title="Face-down tile"></span>

<style>
  .tile-back {
    display: inline-block;
    flex: 0 0 auto;
    border: 2px solid color-mix(in srgb, var(--tile-bg) 74%, var(--wood));
    border-radius: 4px;
    background-color: var(--tile-back);
    background-image:
      linear-gradient(45deg, transparent 43%, rgba(242, 234, 217, 0.16) 44% 56%, transparent 57%),
      linear-gradient(-45deg, transparent 43%, rgba(242, 234, 217, 0.12) 44% 56%, transparent 57%);
    background-size: 7px 7px;
    box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.22), 0 3px 0 rgba(0, 0, 0, 0.3), 0 5px 7px rgba(0, 0, 0, 0.16);
  }
</style>
