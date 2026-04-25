using System.ComponentModel.DataAnnotations;

namespace TetrisDemo.Api.Models;

public sealed class SubmitHighScoreRequest
{
    [Required]
    [StringLength(24, MinimumLength = 1)]
    public string PlayerName { get; init; } = string.Empty;

    [Range(0, 10_000_000)]
    public int Score { get; init; }

    [Range(0, 10_000)]
    public int Lines { get; init; }

    [Range(1, 99)]
    public int Level { get; init; }
}
