export interface HighScore {
  id: number;
  playerName: string;
  score: number;
  lines: number;
  level: number;
  createdAtUtc: string;
}

export interface HighScoreResponse {
  items: HighScore[];
}

export interface SubmitHighScoreRequest {
  playerName: string;
  score: number;
  lines: number;
  level: number;
}

export const fetchHighScores = async (): Promise<HighScore[]> => {
  const response = await fetch('/api/highscores?limit=10');
  if (!response.ok) {
    throw new Error('Failed to load high scores.');
  }

  const payload = (await response.json()) as HighScoreResponse;
  return payload.items;
};

export const submitHighScore = async (request: SubmitHighScoreRequest): Promise<HighScore> => {
  const response = await fetch('/api/highscores', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(request)
  });

  if (!response.ok) {
    const errorPayload = (await response.json().catch(() => ({ message: 'Failed to save score.' }))) as {
      message?: string;
    };
    throw new Error(errorPayload.message ?? 'Failed to save score.');
  }

  return (await response.json()) as HighScore;
};
