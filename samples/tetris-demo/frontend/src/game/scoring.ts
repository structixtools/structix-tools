const LINE_CLEAR_POINTS = [0, 100, 300, 500, 800];

export const calculateLineClearPoints = (clearedLines: number, level: number): number => {
  const safeLineCount = Math.max(0, Math.min(clearedLines, LINE_CLEAR_POINTS.length - 1));
  return LINE_CLEAR_POINTS[safeLineCount] * Math.max(level, 1);
};

export const calculateSoftDropPoints = (steps: number): number => Math.max(steps, 0);

export const calculateHardDropPoints = (distance: number): number => Math.max(distance, 0) * 2;
