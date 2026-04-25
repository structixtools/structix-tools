using Microsoft.Data.Sqlite;
using TetrisDemo.Api.Models;
using TetrisDemo.Api.Models.Requests;

namespace TetrisDemo.Api.Data;

public sealed class HighScoreRepository
{
    private readonly string _connectionString;
    private readonly ILogger<HighScoreRepository> _logger;

    public HighScoreRepository(IConfiguration configuration, ILogger<HighScoreRepository> logger)
    {
        _connectionString = configuration.GetConnectionString("HighScores")
            ?? throw new InvalidOperationException("Missing connection string 'HighScores'.");
        _logger = logger;
    }

    public async Task EnsureCreatedAsync(CancellationToken cancellationToken = default)
    {
        var builder = new SqliteConnectionStringBuilder(_connectionString);
        var dataSource = builder.DataSource;

        if (!string.IsNullOrWhiteSpace(dataSource))
        {
            var fullPath = Path.IsPathRooted(dataSource)
                ? dataSource
                : Path.Combine(AppContext.BaseDirectory, dataSource);
            var directory = Path.GetDirectoryName(fullPath);

            if (!string.IsNullOrWhiteSpace(directory))
            {
                Directory.CreateDirectory(directory);
            }
        }

        await using var connection = new SqliteConnection(_connectionString);
        await connection.OpenAsync(cancellationToken);

        var command = connection.CreateCommand();
        command.CommandText = """
            CREATE TABLE IF NOT EXISTS HighScores (
                Id INTEGER PRIMARY KEY AUTOINCREMENT,
                PlayerName TEXT NOT NULL,
                Score INTEGER NOT NULL,
                Lines INTEGER NOT NULL,
                Level INTEGER NOT NULL,
                CreatedAtUtc TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS IX_HighScores_Score ON HighScores (Score DESC, CreatedAtUtc ASC);
            """;

        await command.ExecuteNonQueryAsync(cancellationToken);
        _logger.LogInformation("SQLite high score repository initialized.");
    }

    public async Task<IReadOnlyList<HighScore>> GetTopScoresAsync(
        int limit,
        bool newestFirst = false,
        CancellationToken cancellationToken = default)
    {
        var results = new List<HighScore>();

        await using var connection = new SqliteConnection(_connectionString);
        await connection.OpenAsync(cancellationToken);

        var orderBy = newestFirst
            ? "Score DESC, CreatedAtUtc DESC"
            : "Score DESC, CreatedAtUtc ASC";

        var command = connection.CreateCommand();
        command.CommandText = $"""
            SELECT Id, PlayerName, Score, Lines, Level, CreatedAtUtc
            FROM HighScores
            ORDER BY {orderBy}
            LIMIT $limit;
            """;
        command.Parameters.AddWithValue("$limit", limit);

        await using var reader = await command.ExecuteReaderAsync(cancellationToken);
        while (await reader.ReadAsync(cancellationToken))
        {
            results.Add(new HighScore
            {
                Id = reader.GetInt64(0),
                PlayerName = reader.GetString(1),
                Score = reader.GetInt32(2),
                Lines = reader.GetInt32(3),
                Level = reader.GetInt32(4),
                CreatedAtUtc = DateTime.Parse(reader.GetString(5)).ToUniversalTime()
            });
        }

        return results;
    }

    public async Task<HighScore?> GetLatestHighScoreAsync(CancellationToken cancellationToken = default)
    {
        await using var connection = new SqliteConnection(_connectionString);
        await connection.OpenAsync(cancellationToken);

        var command = connection.CreateCommand();
        command.CommandText = """
            SELECT Id, PlayerName, Score, Lines, Level, CreatedAtUtc
            FROM HighScores
            ORDER BY CreatedAtUtc DESC
            LIMIT 1;
            """;

        await using var reader = await command.ExecuteReaderAsync(cancellationToken);
        if (!await reader.ReadAsync(cancellationToken))
        {
            return null;
        }

        return new HighScore
        {
            Id = reader.GetInt64(0),
            PlayerName = reader.GetString(1),
            Score = reader.GetInt32(2),
            Lines = reader.GetInt32(3),
            Level = reader.GetInt32(4),
            CreatedAtUtc = DateTime.Parse(reader.GetString(5)).ToUniversalTime()
        };
    }

    public async Task<HighScore> AddAsync(RecordHighScoreRequest request, CancellationToken cancellationToken = default)
    {
        var createdAtUtc = DateTime.UtcNow;
        var normalizedName = request.PlayerName.Trim();

        await using var connection = new SqliteConnection(_connectionString);
        await connection.OpenAsync(cancellationToken);

        var command = connection.CreateCommand();
        command.CommandText = """
            INSERT INTO HighScores (PlayerName, Score, Lines, Level, CreatedAtUtc)
            VALUES ($playerName, $score, $lines, $level, $createdAtUtc);
            SELECT last_insert_rowid();
            """;
        command.Parameters.AddWithValue("$playerName", normalizedName);
        command.Parameters.AddWithValue("$score", request.Score);
        command.Parameters.AddWithValue("$lines", request.Lines);
        command.Parameters.AddWithValue("$level", request.Level);
        command.Parameters.AddWithValue("$createdAtUtc", createdAtUtc.ToString("O"));

        var id = (long)(await command.ExecuteScalarAsync(cancellationToken)
            ?? throw new InvalidOperationException("Failed to insert high score."));

        return new HighScore
        {
            Id = id,
            PlayerName = normalizedName,
            Score = request.Score,
            Lines = request.Lines,
            Level = request.Level,
            CreatedAtUtc = createdAtUtc
        };
    }
}
