-- Make team links reusable by default (one_time_use = 0).
-- Existing links are assumed one-time-use to preserve prior behavior.
ALTER TABLE team_links ADD COLUMN one_time_use INTEGER NOT NULL DEFAULT 1;
