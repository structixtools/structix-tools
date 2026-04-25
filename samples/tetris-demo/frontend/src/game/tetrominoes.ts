export type PieceType = 'I' | 'O' | 'T' | 'S' | 'Z' | 'J' | 'L';

export type Matrix = number[][];

export interface PieceDefinition {
  type: PieceType;
  color: string;
  rotations: Matrix[];
}

const rotate = (matrix: Matrix): Matrix =>
  matrix[0].map((_, columnIndex) => matrix.map((row) => row[columnIndex]).reverse());

const makeRotations = (seed: Matrix): Matrix[] => {
  const rotations: Matrix[] = [seed];
  while (rotations.length < 4) {
    rotations.push(rotate(rotations[rotations.length - 1]));
  }
  return rotations;
};

export const PIECES: PieceDefinition[] = [
  { type: 'I', color: '#38bdf8', rotations: makeRotations([[1, 1, 1, 1]]) },
  { type: 'O', color: '#facc15', rotations: makeRotations([[1, 1], [1, 1]]) },
  { type: 'T', color: '#c084fc', rotations: makeRotations([[0, 1, 0], [1, 1, 1]]) },
  { type: 'S', color: '#4ade80', rotations: makeRotations([[0, 1, 1], [1, 1, 0]]) },
  { type: 'Z', color: '#fb7185', rotations: makeRotations([[1, 1, 0], [0, 1, 1]]) },
  { type: 'J', color: '#60a5fa', rotations: makeRotations([[1, 0, 0], [1, 1, 1]]) },
  { type: 'L', color: '#fb923c', rotations: makeRotations([[0, 0, 1], [1, 1, 1]]) }
];

export const getPieceDefinition = (type: PieceType): PieceDefinition => {
  const match = PIECES.find((piece) => piece.type === type);
  if (!match) {
    throw new Error(`Unknown piece type: ${type}`);
  }
  return match;
};

export const randomPieceType = (): PieceType => PIECES[Math.floor(Math.random() * PIECES.length)].type;
