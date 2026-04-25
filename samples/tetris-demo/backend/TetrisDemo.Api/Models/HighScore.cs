namespace TetrisDemo.Api.Models;

public sealed class HighScore
{
    public long Id { get; init; }
    public string PlayerName { get; init; } = string.Empty;
    public int Score { get; init; }
    public int Lines { get; init; }
    public int Level { get; init; }
    public DateTime CreatedAtUtc { get; init; }
}
