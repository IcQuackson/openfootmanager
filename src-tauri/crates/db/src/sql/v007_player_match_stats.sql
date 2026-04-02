-- Persist per-player per-match stats for later aggregation/history views
ALTER TABLE players ADD COLUMN match_stats TEXT NOT NULL DEFAULT '[]';
