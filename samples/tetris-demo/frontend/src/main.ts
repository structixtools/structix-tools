const rootMessage = document.querySelector<HTMLDivElement>('#game-message');

if (!rootMessage) {
  throw new Error('Missing #game-message element.');
}

rootMessage.textContent = 'Tetris demo scaffold is ready. Gameplay arrives in the next commit.';
