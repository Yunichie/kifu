const SCORE_QUANTUM = 5_000;
const MAX_INTERVALS = 6;

export type ScoreAxis = {
  minimum: number;
  maximum: number;
  step: number;
  ticks: number[];
};

export function buildScoreAxis(scores: number[]): ScoreAxis {
  const lowestScore = scores.length ? Math.min(...scores) : 0;
  const highestScore = scores.length ? Math.max(...scores) : 0;
  const lowestValue = Math.min(0, lowestScore);
  const highestValue = Math.max(0, highestScore);
  const range = highestValue - lowestValue;
  let step = Math.max(SCORE_QUANTUM, Math.ceil(range / MAX_INTERVALS / SCORE_QUANTUM) * SCORE_QUANTUM);
  let minimum = Math.floor(lowestValue / step) * step;
  let maximum = Math.ceil(highestValue / step) * step;

  while ((maximum - minimum) / step > MAX_INTERVALS) {
    step += SCORE_QUANTUM;
    minimum = Math.floor(lowestValue / step) * step;
    maximum = Math.ceil(highestValue / step) * step;
  }

  if (minimum === maximum) maximum += step;

  return {
    minimum,
    maximum,
    step,
    ticks: Array.from({ length: (maximum - minimum) / step + 1 }, (_, index) => maximum - index * step)
  };
}
