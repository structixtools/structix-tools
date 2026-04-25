namespace TetrisDemo.Api.Models.Responses;

public sealed class HighScoreListResponse
{
    public IReadOnlyList<HighScore> Items { get; init; } = Array.Empty<HighScore>();
    public DateTime GeneratedAtUtc { get; init; }
}
