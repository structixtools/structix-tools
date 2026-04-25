import {
  calculateHardDropPoints,
  calculateLineClearPoints,
  calculateSoftDropPoints
} from './scoring.js';
import { getPieceDefinition, randomPieceType, type Matrix, type PieceType } from './tetrominoes.js';

export interface ActivePiece {
  type: PieceType;
  rotation: number;
  row: number;
  column: number;
}

export interface GameState {
  board: string[][];
  currentPiece: ActivePiece | null;
  nextPieceType: PieceType;
  score: number;
  lines: number;
  level: number;
  isGameOver: boolean;
  isPaused: boolean;
}

export class TetrisGame {
  readonly rows = 20;
  readonly columns = 10;
  private dropAccumulator = 0;
  private readonly stateInternal: GameState;

  constructor() {
    this.stateInternal = this.createInitialState();
  }

  get state(): GameState {
    return {
      ...this.stateInternal,
      board: this.stateInternal.board.map((row) => [...row]),
      currentPiece: this.stateInternal.currentPiece ? { ...this.stateInternal.currentPiece } : null
    };
  }

  reset(): void {
    const fresh = this.createInitialState();
    this.stateInternal.board = fresh.board;
    this.stateInternal.currentPiece = fresh.currentPiece;
    this.stateInternal.nextPieceType = fresh.nextPieceType;
    this.stateInternal.score = fresh.score;
    this.stateInternal.lines = fresh.lines;
    this.stateInternal.level = fresh.level;
    this.stateInternal.isGameOver = fresh.isGameOver;
    this.stateInternal.isPaused = fresh.isPaused;
    this.dropAccumulator = 0;
    this.spawnNextPiece();
  }

  update(deltaMs: number, speedMultiplier = 1): void {
    if (this.stateInternal.isGameOver || this.stateInternal.isPaused) {
      return;
    }

    this.dropAccumulator += deltaMs;
    if (this.dropAccumulator >= this.getDropInterval(speedMultiplier)) {
      this.dropAccumulator = 0;
      this.stepDown();
    }
  }

  togglePause(): void {
    if (this.stateInternal.isGameOver) {
      return;
    }
    this.stateInternal.isPaused = !this.stateInternal.isPaused;
  }

  moveLeft(): void {
    this.tryMove(0, -1);
  }

  moveRight(): void {
    this.tryMove(0, 1);
  }

  softDrop(): void {
    if (this.stepDown()) {
      this.stateInternal.score += calculateSoftDropPoints(1);
    }
  }

  hardDrop(): void {
    if (!this.stateInternal.currentPiece || this.stateInternal.isPaused || this.stateInternal.isGameOver) {
      return;
    }

    let droppedRows = 0;
    while (this.tryMove(1, 0)) {
      droppedRows += 1;
    }

    this.stateInternal.score += calculateHardDropPoints(droppedRows);
    this.lockPiece();
  }

  rotate(): void {
    const piece = this.stateInternal.currentPiece;
    if (!piece || this.stateInternal.isPaused || this.stateInternal.isGameOver) {
      return;
    }

    const candidate = { ...piece, rotation: (piece.rotation + 1) % 4 };
    const kicks = [0, -1, 1, -2, 2];

    for (const kick of kicks) {
      const shifted = { ...candidate, column: candidate.column + kick };
      if (!this.collides(shifted)) {
        this.stateInternal.currentPiece = shifted;
        return;
      }
    }
  }

  getMergedBoard(): string[][] {
    const merged = this.stateInternal.board.map((row) => [...row]);
    const piece = this.stateInternal.currentPiece;

    if (!piece) {
      return merged;
    }

    const definition = getPieceDefinition(piece.type);
    const shape = definition.rotations[piece.rotation];

    shape.forEach((row, rowIndex) => {
      row.forEach((cell, columnIndex) => {
        if (!cell) {
          return;
        }

        const targetRow = piece.row + rowIndex;
        const targetColumn = piece.column + columnIndex;
        if (targetRow >= 0 && targetRow < this.rows && targetColumn >= 0 && targetColumn < this.columns) {
          merged[targetRow][targetColumn] = definition.color;
        }
      });
    });

    return merged;
  }

  private createInitialState(): GameState {
    return {
      board: Array.from({ length: this.rows }, () => Array.from({ length: this.columns }, () => '')),
      currentPiece: null,
      nextPieceType: randomPieceType(),
      score: 0,
      lines: 0,
      level: 1,
      isGameOver: false,
      isPaused: false
    };
  }

  private getDropInterval(speedMultiplier: number): number {
    const baseInterval = Math.max(120, 900 - (this.stateInternal.level - 1) * 70);
    const safeMultiplier = speedMultiplier <= 0 ? 1 : speedMultiplier;
    return Math.max(80, Math.floor(baseInterval / safeMultiplier));
  }

  private stepDown(): boolean {
    if (!this.tryMove(1, 0)) {
      this.lockPiece();
      return false;
    }
    return true;
  }

  private tryMove(rowDelta: number, columnDelta: number): boolean {
    const piece = this.stateInternal.currentPiece;
    if (!piece || this.stateInternal.isPaused || this.stateInternal.isGameOver) {
      return false;
    }

    const candidate: ActivePiece = {
      ...piece,
      row: piece.row + rowDelta,
      column: piece.column + columnDelta
    };

    if (this.collides(candidate)) {
      return false;
    }

    this.stateInternal.currentPiece = candidate;
    return true;
  }

  private lockPiece(): void {
    const piece = this.stateInternal.currentPiece;
    if (!piece) {
      return;
    }

    const definition = getPieceDefinition(piece.type);
    const shape = definition.rotations[piece.rotation];

    shape.forEach((row, rowIndex) => {
      row.forEach((cell, columnIndex) => {
        if (!cell) {
          return;
        }

        const boardRow = piece.row + rowIndex;
        const boardColumn = piece.column + columnIndex;
        if (boardRow >= 0) {
          this.stateInternal.board[boardRow][boardColumn] = definition.color;
        }
      });
    });

    this.clearLines();
    this.spawnNextPiece();
  }

  private clearLines(): void {
    const cleared: number[] = [];

    this.stateInternal.board.forEach((row, index) => {
      if (row.every(Boolean)) {
        cleared.push(index);
      }
    });

    if (cleared.length === 0) {
      return;
    }

    for (const rowIndex of cleared.reverse()) {
      this.stateInternal.board.splice(rowIndex, 1);
      this.stateInternal.board.unshift(Array.from({ length: this.columns }, () => ''));
    }

    this.stateInternal.lines += cleared.length;
    this.stateInternal.level = Math.floor(this.stateInternal.lines / 10) + 1;
    this.stateInternal.score += calculateLineClearPoints(cleared.length, this.stateInternal.level);
  }

  private spawnNextPiece(): void {
    const type = this.stateInternal.nextPieceType;
    this.stateInternal.nextPieceType = randomPieceType();

    const definition = getPieceDefinition(type);
    const shape = definition.rotations[0];
    const spawnColumn = Math.floor((this.columns - shape[0].length) / 2);
    const spawnPiece: ActivePiece = {
      type,
      rotation: 0,
      row: 0,
      column: spawnColumn
    };

    if (this.collides(spawnPiece)) {
      this.stateInternal.currentPiece = null;
      this.stateInternal.isGameOver = true;
      return;
    }

    this.stateInternal.currentPiece = spawnPiece;
  }

  private collides(piece: ActivePiece): boolean {
    const definition = getPieceDefinition(piece.type);
    const shape: Matrix = definition.rotations[piece.rotation];

    return shape.some((row, rowIndex) =>
      row.some((cell, columnIndex) => {
        if (!cell) {
          return false;
        }

        const boardRow = piece.row + rowIndex;
        const boardColumn = piece.column + columnIndex;

        if (boardColumn < 0 || boardColumn >= this.columns || boardRow >= this.rows) {
          return true;
        }

        if (boardRow < 0) {
          return false;
        }

        return Boolean(this.stateInternal.board[boardRow][boardColumn]);
      })
    );
  }
}
