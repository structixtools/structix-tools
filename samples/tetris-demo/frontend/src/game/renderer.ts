import { getPieceDefinition, type PieceType } from './tetrominoes.js';

export class GameRenderer {
  constructor(
    private readonly boardContext: CanvasRenderingContext2D,
    private readonly nextPieceContext: CanvasRenderingContext2D,
    private readonly rows: number,
    private readonly columns: number
  ) {}

  render(board: string[][], nextPieceType: PieceType): void {
    this.drawBoard(board);
    this.drawNextPiece(nextPieceType);
  }

  private drawBoard(board: string[][]): void {
    const { canvas } = this.boardContext;
    const cellWidth = canvas.width / this.columns;
    const cellHeight = canvas.height / this.rows;

    this.boardContext.clearRect(0, 0, canvas.width, canvas.height);
    this.boardContext.fillStyle = '#020617';
    this.boardContext.fillRect(0, 0, canvas.width, canvas.height);

    for (let row = 0; row < this.rows; row += 1) {
      for (let column = 0; column < this.columns; column += 1) {
        this.boardContext.strokeStyle = 'rgba(148, 163, 184, 0.12)';
        this.boardContext.strokeRect(column * cellWidth, row * cellHeight, cellWidth, cellHeight);

        const color = board[row][column];
        if (!color) {
          continue;
        }

        this.boardContext.fillStyle = color;
        this.boardContext.fillRect(column * cellWidth + 1, row * cellHeight + 1, cellWidth - 2, cellHeight - 2);
      }
    }
  }

  private drawNextPiece(type: PieceType): void {
    const definition = getPieceDefinition(type);
    const shape = definition.rotations[0];
    const { canvas } = this.nextPieceContext;
    const cellSize = 24;
    const offsetX = (canvas.width - shape[0].length * cellSize) / 2;
    const offsetY = (canvas.height - shape.length * cellSize) / 2;

    this.nextPieceContext.clearRect(0, 0, canvas.width, canvas.height);
    this.nextPieceContext.fillStyle = '#020617';
    this.nextPieceContext.fillRect(0, 0, canvas.width, canvas.height);

    shape.forEach((row, rowIndex) => {
      row.forEach((cell, columnIndex) => {
        if (!cell) {
          return;
        }

        this.nextPieceContext.fillStyle = definition.color;
        this.nextPieceContext.fillRect(offsetX + columnIndex * cellSize, offsetY + rowIndex * cellSize, cellSize - 2, cellSize - 2);
      });
    });
  }
}
