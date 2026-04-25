namespace TetrisDemo.Api.Models.Responses;

public sealed class LatestHighScoreResponse
{
    public HighScore? Item { get; init; }
    public DateTime GeneratedAtUtc { get; init; }
}
