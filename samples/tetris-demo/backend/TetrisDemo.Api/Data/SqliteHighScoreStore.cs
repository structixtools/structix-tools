using Microsoft.Data.Sqlite;
using TetrisDemo.Api.Models;

namespace TetrisDemo.Api.Data;

public sealed class SqliteHighScoreStore
{
    private readonly string _connectionString;
    private readonly ILogger<SqliteHighScoreStore> _logger;

    public SqliteHighScoreStore(IConfiguration configuration, ILogger<SqliteHighScoreStore> logger)
    {
        _connectionString = configuration.GetConnectionString("HighScores")
            ?? throw new InvalidOperationException("Missing connection string 'HighScores'.");
        _logger = logger;
    }

    public async Task InitializeAsync(CancellationToken cancellationToken = default)
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
        _logger.LogInformation("SQLite high score store initialized.");
    }

    public async Task<IReadOnlyList<HighScore>> GetTopScoresAsync(int limit, CancellationToken cancellationToken = default)
    {
        var results = new List<HighScore>();

        await using var connection = new SqliteConnection(_connectionString);
        await connection.OpenAsync(cancellationToken);

        var command = connection.CreateCommand();
        command.CommandText = """
            SELECT Id, PlayerName, Score, Lines, Level, CreatedAtUtc
            FROM HighScores
            ORDER BY Score DESC, CreatedAtUtc ASC
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

    public async Task<HighScore> InsertAsync(SubmitHighScoreRequest request, CancellationToken cancellationToken = default)
    {
        var createdAtUtc = DateTime.UtcNow;

        await using var connection = new SqliteConnection(_connectionString);
        await connection.OpenAsync(cancellationToken);

        var command = connection.CreateCommand();
        command.CommandText = """
            INSERT INTO HighScores (PlayerName, Score, Lines, Level, CreatedAtUtc)
            VALUES ($playerName, $score, $lines, $level, $createdAtUtc);
            SELECT last_insert_rowid();
            """;
        command.Parameters.AddWithValue("$playerName", request.PlayerName.Trim());
        command.Parameters.AddWithValue("$score", request.Score);
        command.Parameters.AddWithValue("$lines", request.Lines);
        command.Parameters.AddWithValue("$level", request.Level);
        command.Parameters.AddWithValue("$createdAtUtc", createdAtUtc.ToString("O"));

        var id = (long)(await command.ExecuteScalarAsync(cancellationToken)
            ?? throw new InvalidOperationException("Failed to insert high score."));

        return new HighScore
        {
            Id = id,
            PlayerName = request.PlayerName.Trim(),
            Score = request.Score,
            Lines = request.Lines,
            Level = request.Level,
            CreatedAtUtc = createdAtUtc
        };
    }
}
