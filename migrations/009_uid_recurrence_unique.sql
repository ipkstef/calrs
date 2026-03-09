-- Change unique constraint from uid alone to (uid, recurrence_id).
-- A recurring event and its modified instances share the same UID but have
-- different RECURRENCE-ID values.  The parent has NULL recurrence_id.

-- SQLite cannot ALTER a UNIQUE constraint, so we recreate the table.
CREATE TABLE events_new (
    id          TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL REFERENCES calendars(id) ON DELETE CASCADE,
    uid         TEXT NOT NULL,
    summary     TEXT,
    start_at    TEXT NOT NULL,
    end_at      TEXT NOT NULL,
    location    TEXT,
    description TEXT,
    status      TEXT,
    rrule       TEXT,
    raw_ical    TEXT,
    recurrence_id TEXT,
    all_day     INTEGER NOT NULL DEFAULT 0,
    timezone    TEXT,
    synced_at   TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(uid, COALESCE(recurrence_id, ''))
);

INSERT INTO events_new SELECT * FROM events;
DROP TABLE events;
ALTER TABLE events_new RENAME TO events;

CREATE INDEX IF NOT EXISTS idx_events_calendar ON events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_events_start ON events(start_at);
