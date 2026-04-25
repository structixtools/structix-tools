namespace TetrisDemo.Api.Models;

public sealed class HighScoreResponse
{
    public IReadOnlyList<HighScore> Items { get; init; } = Array.Empty<HighScore>();
}
