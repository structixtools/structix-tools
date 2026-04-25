import type { HighScore } from '../api/highscores.js';

export class HighScoreTable {
  constructor(private readonly tbody: HTMLTableSectionElement) {}

  render(scores: HighScore[]): void {
    this.tbody.innerHTML = '';

    if (scores.length === 0) {
      const row = document.createElement('tr');
      row.className = 'empty-row';
      row.innerHTML = '<td colspan="5">No high scores yet. Be the first.</td>';
      this.tbody.appendChild(row);
      return;
    }

    scores.forEach((score, index) => {
      const row = document.createElement('tr');
      row.innerHTML = `
        <td>${index + 1}</td>
        <td>${escapeHtml(score.playerName)}</td>
        <td>${score.score.toLocaleString()}</td>
        <td>${score.lines}</td>
        <td>${score.level}</td>
      `;
      this.tbody.appendChild(row);
    });
  }

  renderError(message: string): void {
    this.tbody.innerHTML = '';
    const row = document.createElement('tr');
    row.className = 'error-row';
    row.innerHTML = `<td colspan="5">${escapeHtml(message)}</td>`;
    this.tbody.appendChild(row);
  }
}

const escapeHtml = (value: string): string =>
  value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
