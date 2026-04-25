using System.ComponentModel.DataAnnotations;
using TetrisDemo.Api.Data;
using TetrisDemo.Api.Models.Requests;
using TetrisDemo.Api.Models.Responses;
using TetrisDemo.Api.Services;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSingleton<HighScoreRepository>();

var app = builder.Build();

var highScoreRepository = app.Services.GetRequiredService<HighScoreRepository>();
await highScoreRepository.EnsureCreatedAsync();

app.UseDefaultFiles();
app.UseStaticFiles();

app.MapGet("/api/health", () => Results.Ok(new { ok = true }));

app.MapGet("/api/highscores", async (int? limit, bool? newestFirst, HighScoreRepository repository, CancellationToken cancellationToken) =>
{
    var sanitizedLimit = Math.Clamp(limit ?? 10, 1, 50);
    var items = await repository.GetTopScoresAsync(sanitizedLimit, newestFirst ?? false, cancellationToken);
    return Results.Ok(new HighScoreListResponse
    {
        Items = items,
        GeneratedAtUtc = DateTime.UtcNow
    });
});

app.MapGet("/api/highscores/latest", async (HighScoreRepository repository, CancellationToken cancellationToken) =>
{
    var latest = await repository.GetLatestHighScoreAsync(cancellationToken);
    return Results.Ok(new LatestHighScoreResponse
    {
        Item = latest,
        GeneratedAtUtc = DateTime.UtcNow
    });
});

app.MapPost("/api/highscores", async (RecordHighScoreRequest request, HighScoreRepository repository, CancellationToken cancellationToken) =>
{
    var validationResults = new List<ValidationResult>();
    if (!Validator.TryValidateObject(request, new ValidationContext(request), validationResults, true))
    {
        return Results.ValidationProblem(validationResults
            .GroupBy(x => x.MemberNames.FirstOrDefault() ?? string.Empty)
            .ToDictionary(g => g.Key, g => g.Select(x => x.ErrorMessage ?? "Invalid value.").ToArray()));
    }

    if (!HighScoreValidationService.HasConsistentProgression(request))
    {
        return Results.BadRequest(new { message = "Submitted score looks invalid for the reported lines and level." });
    }

    var score = await repository.AddAsync(request, cancellationToken);
    return Results.Created($"/api/highscores/{score.Id}", score);
});

app.MapFallbackToFile("index.html");

app.Run();
