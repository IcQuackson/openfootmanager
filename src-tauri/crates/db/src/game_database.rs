use log::{debug, error, info};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::migrations::{MIGRATION_COUNT, all_migrations};

/// Represents an open per-save game database with migrations applied.
pub struct GameDatabase {
    conn: Connection,
    path: Option<PathBuf>,
}

fn read_user_version(conn: &Connection) -> Result<i64, String> {
    conn.pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|e| format!("Failed to read schema version: {}", e))
}

fn has_column(conn: &Connection, table: &str, column: &str) -> Result<bool, String> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_info({})", table))
        .map_err(|e| format!("Failed to inspect {} schema: {}", table, e))?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| format!("Failed to inspect {} schema: {}", table, e))?;

    for row in rows {
        if row.map_err(|e| format!("Failed to read {} schema row: {}", table, e))? == column {
            return Ok(true);
        }
    }

    Ok(false)
}

fn ensure_column(
    conn: &Connection,
    table: &str,
    column: &str,
    migration_sql: &str,
) -> Result<(), String> {
    if has_column(conn, table, column)? {
        return Ok(());
    }

    info!(
        "[game_db] adding missing compatibility column {}.{}",
        table, column
    );
    conn.execute_batch(migration_sql).map_err(|e| {
        format!(
            "Failed to add compatibility column {}.{}: {}",
            table, column, e
        )
    })
}

fn reconcile_post_v6_schema(conn: &Connection) -> Result<(), String> {
    ensure_column(
        conn,
        "players",
        "match_stats",
        include_str!("sql/v007_player_match_stats.sql"),
    )?;
    ensure_column(
        conn,
        "teams",
        "financial_ledger",
        include_str!("sql/v007_team_financial_ledger.sql"),
    )?;
    ensure_column(
        conn,
        "teams",
        "sponsorship",
        include_str!("sql/v008_team_sponsorship.sql"),
    )?;
    ensure_column(
        conn,
        "teams",
        "facilities",
        include_str!("sql/v009_team_facilities.sql"),
    )?;
    ensure_column(
        conn,
        "players",
        "morale_core",
        include_str!("sql/v010_player_morale_core.sql"),
    )?;
    ensure_column(
        conn,
        "players",
        "footedness",
        include_str!("sql/v011_player_footedness.sql"),
    )?;
    ensure_column(
        conn,
        "fixtures",
        "competition",
        include_str!("sql/v012_fixture_competition.sql"),
    )?;
    ensure_column(
        conn,
        "players",
        "fitness",
        include_str!("sql/v013_player_fitness.sql"),
    )?;

    conn.pragma_update(None, "user_version", MIGRATION_COUNT as i64)
        .map_err(|e| format!("Failed to update schema version: {}", e))?;
    Ok(())
}

impl GameDatabase {
    /// Open (or create) a game database at the given path and apply all migrations.
    pub fn open(path: &Path) -> Result<Self, String> {
        debug!("[game_db] opening database at {:?}", path);
        let mut conn = Connection::open(path).map_err(|e| {
            error!("[game_db] failed to open database at {:?}: {}", path, e);
            format!("Failed to open database: {}", e)
        })?;

        let initial_version = read_user_version(&conn)?;
        if initial_version > 6 {
            info!(
                "[game_db] reconciling divergent post-v6 schema at {:?} from version {}",
                path, initial_version
            );
            reconcile_post_v6_schema(&conn)?;
        } else {
            let migrations = all_migrations();
            migrations.to_latest(&mut conn).map_err(|e| {
                error!("[game_db] migration failed for {:?}: {}", path, e);
                format!("Database migration failed: {}", e)
            })?;
        }

        info!("[game_db] database ready at {:?}", path);
        Ok(Self {
            conn,
            path: Some(path.to_path_buf()),
        })
    }

    /// Create an in-memory game database (useful for tests and pre-save state).
    pub fn open_in_memory() -> Result<Self, String> {
        debug!("[game_db] opening in-memory database");
        let mut conn = Connection::open_in_memory().map_err(|e| {
            error!("[game_db] failed to open in-memory database: {}", e);
            format!("Failed to open in-memory database: {}", e)
        })?;

        let migrations = all_migrations();
        migrations.to_latest(&mut conn).map_err(|e| {
            error!("[game_db] migration failed for in-memory db: {}", e);
            format!("Database migration failed: {}", e)
        })?;

        Ok(Self { conn, path: None })
    }

    /// Get a reference to the underlying connection (for repositories).
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Get the file path, if this is a file-backed database.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Get the current schema version (number of applied migrations).
    pub fn schema_version(&self) -> Result<i64, String> {
        read_user_version(&self.conn)
    }

    /// Validate that the database has the expected schema version.
    /// Returns Ok(true) if valid, Ok(false) if version mismatch.
    pub fn validate_schema(&self) -> Result<bool, String> {
        let migrations = all_migrations();
        let current: usize = migrations
            .current_version(&self.conn)
            .map_err(|e| format!("Failed to get current version: {}", e))?
            .into();
        // We expect the version to equal the number of migrations (1 for V1)
        let expected = MIGRATION_COUNT;
        Ok(current == expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn apply_sql(conn: &Connection, sql: &str, version: i64) {
        conn.execute_batch(sql).unwrap();
        conn.pragma_update(None, "user_version", version).unwrap();
    }

    fn test_has_column(conn: &Connection, table: &str, column: &str) -> bool {
        has_column(conn, table, column).unwrap()
    }

    #[test]
    fn test_open_in_memory() {
        let db = GameDatabase::open_in_memory().unwrap();
        assert!(db.path().is_none());
        assert_eq!(
            db.schema_version().unwrap(),
            crate::migrations::MIGRATION_COUNT as i64
        );
    }

    #[test]
    fn test_open_file_database() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test_game.db");

        let db = GameDatabase::open(&db_path).unwrap();
        assert_eq!(db.path().unwrap(), db_path);
        assert_eq!(
            db.schema_version().unwrap(),
            crate::migrations::MIGRATION_COUNT as i64
        );
        assert!(db.validate_schema().unwrap());
    }

    #[test]
    fn test_reopen_existing_database() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test_reopen.db");

        // Create and close
        {
            let db = GameDatabase::open(&db_path).unwrap();
            assert_eq!(
                db.schema_version().unwrap(),
                crate::migrations::MIGRATION_COUNT as i64
            );
        }

        // Reopen — migrations should be idempotent
        let db = GameDatabase::open(&db_path).unwrap();
        assert_eq!(
            db.schema_version().unwrap(),
            crate::migrations::MIGRATION_COUNT as i64
        );
        assert!(db.validate_schema().unwrap());
    }

    #[test]
    fn test_validate_schema_on_empty_db() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("empty.db");

        // Create a raw DB without migrations
        {
            let _conn = Connection::open(&db_path).unwrap();
        }

        // Opening via GameDatabase applies migrations, so it should be valid
        let db = GameDatabase::open(&db_path).unwrap();
        assert!(db.validate_schema().unwrap());
    }

    #[test]
    fn test_conn_is_usable() {
        let db = GameDatabase::open_in_memory().unwrap();
        // Verify we can query a table created by the migration
        let count: i64 = db
            .conn()
            .query_row("SELECT COUNT(*) FROM teams", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_open_repairs_release_lineage_schema() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("release_lineage.db");
        let conn = Connection::open(&db_path).unwrap();

        apply_sql(&conn, include_str!("sql/v001_initial_schema.sql"), 1);
        apply_sql(&conn, include_str!("sql/v002_training_groups.sql"), 2);
        apply_sql(&conn, include_str!("sql/v003_alternate_positions.sql"), 3);
        apply_sql(&conn, include_str!("sql/v004_natural_position.sql"), 4);
        apply_sql(&conn, include_str!("sql/v005_player_training_focus.sql"), 5);
        apply_sql(&conn, include_str!("sql/v006_team_match_roles.sql"), 6);
        apply_sql(&conn, include_str!("sql/v007_team_financial_ledger.sql"), 7);
        apply_sql(&conn, include_str!("sql/v008_team_sponsorship.sql"), 8);
        apply_sql(&conn, include_str!("sql/v009_team_facilities.sql"), 9);
        apply_sql(&conn, include_str!("sql/v010_player_morale_core.sql"), 10);
        apply_sql(&conn, include_str!("sql/v011_player_footedness.sql"), 11);
        apply_sql(&conn, include_str!("sql/v012_fixture_competition.sql"), 12);
        apply_sql(&conn, include_str!("sql/v013_player_fitness.sql"), 13);
        drop(conn);

        let db = GameDatabase::open(&db_path).unwrap();
        assert_eq!(db.schema_version().unwrap(), MIGRATION_COUNT as i64);
        assert!(test_has_column(db.conn(), "players", "match_stats"));
        assert!(test_has_column(db.conn(), "teams", "financial_ledger"));
    }

    #[test]
    fn test_open_repairs_custom_lineage_schema() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("custom_lineage.db");
        let conn = Connection::open(&db_path).unwrap();

        apply_sql(&conn, include_str!("sql/v001_initial_schema.sql"), 1);
        apply_sql(&conn, include_str!("sql/v002_training_groups.sql"), 2);
        apply_sql(&conn, include_str!("sql/v003_alternate_positions.sql"), 3);
        apply_sql(&conn, include_str!("sql/v004_natural_position.sql"), 4);
        apply_sql(&conn, include_str!("sql/v005_player_training_focus.sql"), 5);
        apply_sql(&conn, include_str!("sql/v006_team_match_roles.sql"), 6);
        apply_sql(&conn, include_str!("sql/v007_player_match_stats.sql"), 7);
        drop(conn);

        let db = GameDatabase::open(&db_path).unwrap();
        assert_eq!(db.schema_version().unwrap(), MIGRATION_COUNT as i64);
        assert!(test_has_column(db.conn(), "players", "match_stats"));
        assert!(test_has_column(db.conn(), "teams", "financial_ledger"));
        assert!(test_has_column(db.conn(), "players", "fitness"));
    }
}
