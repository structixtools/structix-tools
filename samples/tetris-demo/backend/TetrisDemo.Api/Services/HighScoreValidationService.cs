using TetrisDemo.Api.Models.Requests;

namespace TetrisDemo.Api.Services;

public static class HighScoreValidationService
{
    public static bool HasConsistentProgression(RecordHighScoreRequest request)
    {
        if (string.IsNullOrWhiteSpace(request.PlayerName))
        {
            return false;
        }

        if (request.Score < 0 || request.Lines < 0 || request.Level < 1)
        {
            return false;
        }

        var roughMinimum = request.Lines * 40;
        var roughMaximum = Math.Max(100_000, request.Lines * 3_000 + request.Level * 5_000);

        return request.Score >= roughMinimum && request.Score <= roughMaximum;
    }
}
