using System.ComponentModel.DataAnnotations;
using TetrisDemo.Api.Data;
using TetrisDemo.Api.Models;
using TetrisDemo.Api.Services;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSingleton<SqliteHighScoreStore>();

var app = builder.Build();

var scoreStore = app.Services.GetRequiredService<SqliteHighScoreStore>();
await scoreStore.InitializeAsync();

app.UseDefaultFiles();
app.UseStaticFiles();

app.MapGet("/api/health", () => Results.Ok(new { ok = true }));

app.MapGet("/api/highscores", async (int? limit, SqliteHighScoreStore store, CancellationToken cancellationToken) =>
{
    var sanitizedLimit = Math.Clamp(limit ?? 10, 1, 50);
    var items = await store.GetTopScoresAsync(sanitizedLimit, cancellationToken);
    return Results.Ok(new HighScoreResponse { Items = items });
});

app.MapPost("/api/highscores", async (SubmitHighScoreRequest request, SqliteHighScoreStore store, CancellationToken cancellationToken) =>
{
    var validationResults = new List<ValidationResult>();
    if (!Validator.TryValidateObject(request, new ValidationContext(request), validationResults, true))
    {
        return Results.ValidationProblem(validationResults
            .GroupBy(x => x.MemberNames.FirstOrDefault() ?? string.Empty)
            .ToDictionary(g => g.Key, g => g.Select(x => x.ErrorMessage ?? "Invalid value.").ToArray()));
    }

    if (!ScoreValidator.IsPlausible(request))
    {
        return Results.BadRequest(new { message = "Submitted score looks invalid for the reported lines and level." });
    }

    var score = await store.InsertAsync(request, cancellationToken);
    return Results.Created($"/api/highscores/{score.Id}", score);
});

app.MapFallbackToFile("index.html");

app.Run();
