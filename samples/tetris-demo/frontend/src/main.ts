import { fetchHighScores, submitHighScore } from './api/highscores.js';
import { BoardRenderer } from './game/board-renderer.js';
import { TetrisGame } from './game/engine.js';
import { requireElement } from './ui/dom.js';
import { HighScoreTable } from './ui/highscore-table.js';

const boardCanvas = requireElement<HTMLCanvasElement>('#game-canvas');
const nextCanvas = requireElement<HTMLCanvasElement>('#next-piece-canvas');
const startButton = requireElement<HTMLButtonElement>('#start-button');
const messageBanner = requireElement<HTMLDivElement>('#game-message');
const scoreValue = requireElement<HTMLSpanElement>('#score-value');
const linesValue = requireElement<HTMLSpanElement>('#lines-value');
const levelValue = requireElement<HTMLSpanElement>('#level-value');
const scoreForm = requireElement<HTMLFormElement>('#score-form');
const playerNameInput = requireElement<HTMLInputElement>('#player-name');
const scoreFormMessage = requireElement<HTMLParagraphElement>('#score-form-message');
const highScoreBody = requireElement<HTMLTableSectionElement>('#highscore-body');

const boardContext = boardCanvas.getContext('2d');
const nextContext = nextCanvas.getContext('2d');

if (!boardContext || !nextContext) {
  throw new Error('Canvas rendering is not supported by this browser.');
}

const game = new TetrisGame();
const renderer = new BoardRenderer(boardContext, nextContext, game.rows, game.columns);
const highScoreTable = new HighScoreTable(highScoreBody);

let pendingSubmission = false;
let currentSessionSaved = false;
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
    setBanner(`Game over. Final score: ${state.score.toLocaleString()}.`, 'error');
    if (state.score > 0 && !currentSessionSaved) {
      scoreForm.classList.remove('hidden');
    }
  }
};

const gameLoop = (timestamp: number): void => {
  const delta = timestamp - previousTimestamp;
  previousTimestamp = timestamp;
  const speedMultiplier = game.state.level >= 5 ? 1.15 : 1;
  game.update(delta, speedMultiplier);
  syncUi();
  requestAnimationFrame(gameLoop);
};

const loadHighScores = async (): Promise<void> => {
  try {
    const scores = await fetchHighScores();
    highScoreTable.render(scores);
  } catch (error) {
    highScoreTable.renderError(error instanceof Error ? error.message : 'Unable to load high scores.');
  }
};

const startGame = (): void => {
  game.reset();
  currentSessionSaved = false;
  scoreForm.classList.add('hidden');
  scoreFormMessage.textContent = '';
  setBanner('Game started. Good luck!', 'success');
  syncUi();
};

const setBanner = (message: string, kind: 'success' | 'error' | '' = ''): void => {
  messageBanner.textContent = message;
  messageBanner.className = kind ? `message-banner ${kind}` : 'message-banner';
};

const setFormMessage = (message: string, kind: 'success' | 'error' | '' = ''): void => {
  scoreFormMessage.textContent = message;
  scoreFormMessage.className = kind ? `form-message ${kind}` : 'form-message';
};

window.addEventListener('keydown', (event) => {
  if (pendingSubmission) {
    return;
  }

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

scoreForm.addEventListener('submit', async (event) => {
  event.preventDefault();
  if (pendingSubmission) {
    return;
  }

  const state = game.state;
  const playerName = playerNameInput.value.trim();
  if (!playerName) {
    setFormMessage('Enter a player name before saving.', 'error');
    return;
  }

  pendingSubmission = true;
  setFormMessage('Saving score...');

  try {
    await submitHighScore({
      playerName,
      score: state.score,
      lines: state.lines,
      level: state.level
    });
    currentSessionSaved = true;
    scoreForm.classList.add('hidden');
    playerNameInput.value = '';
    setFormMessage('High score saved.', 'success');
    await loadHighScores();
  } catch (error) {
    setFormMessage(error instanceof Error ? error.message : 'Failed to save score.', 'error');
  } finally {
    pendingSubmission = false;
  }
});

void loadHighScores();
syncUi();
requestAnimationFrame(gameLoop);
