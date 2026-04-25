export interface HighScore {
  id: number;
  playerName: string;
  score: number;
  lines: number;
  level: number;
  createdAtUtc: string;
}

export interface HighScoreListResponse {
  items: HighScore[];
  generatedAtUtc: string;
}

export interface LatestHighScoreResponse {
  item: HighScore | null;
  generatedAtUtc: string;
}

export interface RecordHighScoreRequest {
  playerName: string;
  score: number;
  lines: number;
  level: number;
}

export const fetchHighScoreBoard = async (): Promise<HighScoreListResponse> => {
  const response = await fetch('/api/highscores?limit=10&newestFirst=true');
  if (!response.ok) {
    throw new Error('Failed to load high scores.');
  }

  return (await response.json()) as HighScoreListResponse;
};

export const fetchLatestHighScore = async (): Promise<HighScore | null> => {
  const response = await fetch('/api/highscores/latest');
  if (!response.ok) {
    throw new Error('Failed to load the latest saved score.');
  }

  const payload = (await response.json()) as LatestHighScoreResponse;
  return payload.item;
};

export const recordHighScore = async (request: RecordHighScoreRequest): Promise<HighScore> => {
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
