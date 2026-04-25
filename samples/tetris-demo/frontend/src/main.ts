import { TetrisGame } from './game/engine.js';
import { GameRenderer } from './game/renderer.js';

const boardCanvas = mustGet<HTMLCanvasElement>('#game-canvas');
const nextCanvas = mustGet<HTMLCanvasElement>('#next-piece-canvas');
const startButton = mustGet<HTMLButtonElement>('#start-button');
const messageBanner = mustGet<HTMLDivElement>('#game-message');
const scoreValue = mustGet<HTMLSpanElement>('#score-value');
const linesValue = mustGet<HTMLSpanElement>('#lines-value');
const levelValue = mustGet<HTMLSpanElement>('#level-value');

const boardContext = boardCanvas.getContext('2d');
const nextContext = nextCanvas.getContext('2d');

if (!boardContext || !nextContext) {
  throw new Error('Canvas rendering is not supported by this browser.');
}

const game = new TetrisGame();
const renderer = new GameRenderer(boardContext, nextContext, game.rows, game.columns);

let previousTimestamp = performance.now();

const syncUi = (): void => {
  const state = game.state;
  scoreValue.textContent = state.score.toLocaleString();
  linesValue.textContent = state.lines.toString();
  levelValue.textContent = state.level.toString();
  renderer.render(game.getMergedBoard(), state.nextPieceType);

  if (state.isPaused) {
    setBanner('Paused. Press P to resume.');
  } else if (state.isGameOver) {
    setBanner(`Game over. Final score: ${state.score.toLocaleString()}. High scores arrive in the next step.`, 'error');
  }
};

const gameLoop = (timestamp: number): void => {
  const delta = timestamp - previousTimestamp;
  previousTimestamp = timestamp;
  game.update(delta);
  syncUi();
  requestAnimationFrame(gameLoop);
};

const startGame = (): void => {
  game.reset();
  setBanner('Game started. Good luck!', 'success');
  syncUi();
};

const setBanner = (message: string, kind: 'success' | 'error' | '' = ''): void => {
  messageBanner.textContent = message;
  messageBanner.className = kind ? `message-banner ${kind}` : 'message-banner';
};

window.addEventListener('keydown', (event) => {
  switch (event.code) {
    case 'ArrowLeft':
      event.preventDefault();
      game.moveLeft();
      break;
    case 'ArrowRight':
      event.preventDefault();
      game.moveRight();
      break;
    case 'ArrowUp':
      event.preventDefault();
      game.rotate();
      break;
    case 'ArrowDown':
      event.preventDefault();
      game.softDrop();
      break;
    case 'Space':
      event.preventDefault();
      game.hardDrop();
      break;
    case 'KeyP':
      event.preventDefault();
      game.togglePause();
      break;
    default:
      return;
  }

  syncUi();
});

startButton.addEventListener('click', startGame);

syncUi();
requestAnimationFrame(gameLoop);

function mustGet<T extends Element>(selector: string): T {
  const element = document.querySelector<T>(selector);
  if (!element) {
    throw new Error(`Missing element: ${selector}`);
  }
  return element;
}
