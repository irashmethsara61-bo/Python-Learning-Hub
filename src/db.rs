use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A small, thread-safe wrapper around a single SQLite connection.
///
/// The site is a low-traffic, single-process application, so one connection
/// guarded by a mutex is simpler and faster than a connection pool. All calls
/// are executed inside `tokio::task::spawn_blocking` by the API layer.
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Serialize)]
pub struct Attempt {
    pub id: i64,
    pub name: String,
    pub score: i64,
    pub total: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub name: String,
    pub score: i64,
    pub total: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct Progress {
    pub client_id: String,
    pub completed: Vec<bool>,
    pub updated_at: Option<String>,
}

impl Database {
    /// Open (or create) the database and apply the schema.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS quiz_attempts (
                 id         INTEGER PRIMARY KEY AUTOINCREMENT,
                 name       TEXT    NOT NULL,
                 score      INTEGER NOT NULL,
                 total      INTEGER NOT NULL,
                 created_at TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
             );

             CREATE INDEX IF NOT EXISTS idx_quiz_attempts_rank
                 ON quiz_attempts (score DESC, created_at ASC);

             CREATE TABLE IF NOT EXISTS roadmap_progress (
                 client_id  TEXT PRIMARY KEY,
                 completed  TEXT NOT NULL,
                 updated_at TEXT NOT NULL
             );",
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Verify that the database is usable.
    pub fn health(&self) -> rusqlite::Result<()> {
        self.with_conn(|conn| {
            conn.query_row("SELECT 1", [], |row| row.get::<_, i64>(0))
                .map(|_| ())
        })
    }

    /// Store one quiz attempt and return the persisted row.
    pub fn insert_attempt(&self, name: &str, score: i64, total: i64) -> rusqlite::Result<Attempt> {
        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO quiz_attempts (name, score, total) VALUES (?1, ?2, ?3)",
                params![name, score, total],
            )?;

            let id = conn.last_insert_rowid();
            conn.query_row(
                "SELECT id, name, score, total, created_at FROM quiz_attempts WHERE id = ?1",
                [id],
                |row| {
                    Ok(Attempt {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        score: row.get(2)?,
                        total: row.get(3)?,
                        created_at: row.get(4)?,
                    })
                },
            )
        })
    }

    /// Rank and best score for one player (ties share a rank).
    pub fn player_standing(&self, name: &str, total: i64) -> rusqlite::Result<(i64, i64)> {
        self.with_conn(|conn| {
            let best: i64 = conn.query_row(
                "SELECT MAX(score) FROM quiz_attempts WHERE name = ?1 AND total = ?2",
                params![name, total],
                |row| row.get(0),
            )?;

            let rank: i64 = conn.query_row(
                "SELECT COUNT(*) + 1
                 FROM (
                     SELECT name, MAX(score) AS best
                     FROM quiz_attempts
                     WHERE total = ?1
                     GROUP BY name
                 )
                 WHERE best > ?2",
                params![total, best],
                |row| row.get(0),
            )?;

            Ok((rank, best))
        })
    }

    /// Top scores, one row per player, ranked by their best attempt.
    pub fn leaderboard(&self, limit: usize) -> rusqlite::Result<Vec<LeaderboardEntry>> {
        self.with_conn(|conn| {
            let mut statement = conn.prepare(
                "SELECT name, MAX(score) AS best, total, MIN(created_at) AS first_at
                 FROM quiz_attempts
                 GROUP BY name
                 ORDER BY best DESC, first_at ASC
                 LIMIT ?1",
            )?;

            let rows = statement.query_map([limit as i64], |row| {
                Ok(LeaderboardEntry {
                    rank: 0,
                    name: row.get(0)?,
                    score: row.get(1)?,
                    total: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })?;

            let mut entries = Vec::new();
            let mut previous_score = None;
            let mut current_rank = 0;
            for (index, row) in rows.enumerate() {
                let mut entry = row?;
                if previous_score != Some(entry.score) {
                    current_rank = (index + 1) as i64;
                    previous_score = Some(entry.score);
                }
                entry.rank = current_rank;
                entries.push(entry);
            }
            Ok(entries)
        })
    }

    /// Read saved roadmap progress. Missing records return an empty list.
    pub fn get_progress(&self, client_id: &str) -> rusqlite::Result<Progress> {
        self.with_conn(|conn| {
            let stored = conn
                .query_row(
                    "SELECT completed, updated_at FROM roadmap_progress WHERE client_id = ?1",
                    [client_id],
                    |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
                )
                .optional()?;

            Ok(match stored {
                Some((completed, updated_at)) => Progress {
                    client_id: client_id.to_owned(),
                    completed: serde_json::from_str(&completed).unwrap_or_default(),
                    updated_at: Some(updated_at),
                },
                None => Progress {
                    client_id: client_id.to_owned(),
                    completed: Vec::new(),
                    updated_at: None,
                },
            })
        })
    }

    /// Insert or replace roadmap progress for one browser.
    pub fn put_progress(&self, client_id: &str, completed: &[bool]) -> rusqlite::Result<Progress> {
        let completed_json = serde_json::to_string(completed).unwrap_or_else(|_| "[]".to_owned());

        self.with_conn(|conn| {
            conn.execute(
                "INSERT INTO roadmap_progress (client_id, completed, updated_at)
                 VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
                 ON CONFLICT(client_id) DO UPDATE SET
                     completed  = excluded.completed,
                     updated_at = excluded.updated_at",
                params![client_id, completed_json],
            )?;

            let updated_at = conn.query_row(
                "SELECT updated_at FROM roadmap_progress WHERE client_id = ?1",
                [client_id],
                |row| row.get(0),
            )?;

            Ok(Progress {
                client_id: client_id.to_owned(),
                completed: completed.to_vec(),
                updated_at: Some(updated_at),
            })
        })
    }

    /// Delete saved roadmap progress.
    pub fn delete_progress(&self, client_id: &str) -> rusqlite::Result<()> {
        self.with_conn(|conn| {
            conn.execute(
                "DELETE FROM roadmap_progress WHERE client_id = ?1",
                [client_id],
            )
            .map(|_| ())
        })
    }

    fn with_conn<T>(
        &self,
        operation: impl FnOnce(&Connection) -> rusqlite::Result<T>,
    ) -> rusqlite::Result<T> {
        // Recover the guard if a previous caller panicked while holding it.
        let conn = self
            .conn
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        operation(&conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Database {
        Database::open(Path::new(":memory:")).expect("in-memory database")
    }

    #[test]
    fn leaderboard_keeps_best_attempt_and_shares_ranks() {
        let db = test_db();
        db.insert_attempt("Alice", 6, 8).unwrap();
        db.insert_attempt("Bob", 8, 8).unwrap();
        db.insert_attempt("Alice", 7, 8).unwrap();
        db.insert_attempt("Cara", 7, 8).unwrap();

        let entries = db.leaderboard(10).unwrap();
        let mut summary: Vec<(&str, i64, i64)> = entries
            .iter()
            .map(|entry| (entry.name.as_str(), entry.score, entry.rank))
            .collect();
        summary.sort_unstable();
        assert_eq!(
            summary,
            vec![("Alice", 7, 2), ("Bob", 8, 1), ("Cara", 7, 2)]
        );

        assert_eq!(db.player_standing("Alice", 8).unwrap(), (2, 7));
        assert_eq!(db.player_standing("Bob", 8).unwrap(), (1, 8));

        let limited = db.leaderboard(1).unwrap();
        assert_eq!(limited.len(), 1);
        assert_eq!(limited[0].name, "Bob");
    }

    #[test]
    fn progress_roundtrip_and_delete() {
        let db = test_db();
        let client_id = "integration-client-0001";

        let missing = db.get_progress(client_id).unwrap();
        assert!(missing.completed.is_empty());
        assert!(missing.updated_at.is_none());

        let values = [true, false, true, false, false];
        let saved = db.put_progress(client_id, &values).unwrap();
        assert_eq!(saved.completed, values);
        assert!(saved.updated_at.is_some());

        let loaded = db.get_progress(client_id).unwrap();
        assert_eq!(loaded.completed, values);
        assert!(loaded.updated_at.is_some());

        db.delete_progress(client_id).unwrap();
        let gone = db.get_progress(client_id).unwrap();
        assert!(gone.completed.is_empty());
        assert!(gone.updated_at.is_none());
    }

    #[test]
    fn health_check_succeeds() {
        test_db().health().unwrap();
    }
}
