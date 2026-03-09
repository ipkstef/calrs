-- calrs initial schema

CREATE TABLE IF NOT EXISTS accounts (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    email       TEXT NOT NULL UNIQUE,
    timezone    TEXT NOT NULL DEFAULT 'UTC',
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS caldav_sources (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    url         TEXT NOT NULL,
    username    TEXT NOT NULL,
    password_enc TEXT,
    last_synced TEXT,
    sync_token  TEXT,
    enabled     INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS calendars (
    id          TEXT PRIMARY KEY,
    source_id   TEXT NOT NULL REFERENCES caldav_sources(id) ON DELETE CASCADE,
    href        TEXT NOT NULL,
    display_name TEXT,
    color       TEXT,
    ctag        TEXT,
    is_busy     INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS events (
    id          TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL REFERENCES calendars(id) ON DELETE CASCADE,
    uid         TEXT NOT NULL UNIQUE,
    etag        TEXT,
    summary     TEXT,
    description TEXT,
    location    TEXT,
    start_at    TEXT NOT NULL,
    end_at      TEXT NOT NULL,
    all_day     INTEGER NOT NULL DEFAULT 0,
    timezone    TEXT,
    rrule       TEXT,
    status      TEXT DEFAULT 'confirmed',
    raw_ical    TEXT,
    synced_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS event_types (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    slug            TEXT NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT,
    duration_min    INTEGER NOT NULL,
    location_type   TEXT NOT NULL DEFAULT 'link',
    location_value  TEXT,
    buffer_before   INTEGER DEFAULT 0,
    buffer_after    INTEGER DEFAULT 0,
    min_notice_min  INTEGER DEFAULT 60,
    enabled         INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(account_id, slug)
);

CREATE TABLE IF NOT EXISTS availability_rules (
    id              TEXT PRIMARY KEY,
    event_type_id   TEXT NOT NULL REFERENCES event_types(id) ON DELETE CASCADE,
    day_of_week     INTEGER NOT NULL CHECK(day_of_week BETWEEN 0 AND 6),
    start_time      TEXT NOT NULL,
    end_time        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS availability_overrides (
    id              TEXT PRIMARY KEY,
    event_type_id   TEXT NOT NULL REFERENCES event_types(id) ON DELETE CASCADE,
    date            TEXT NOT NULL,
    start_time      TEXT,
    end_time        TEXT,
    is_blocked      INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS bookings (
    id              TEXT PRIMARY KEY,
    event_type_id   TEXT NOT NULL REFERENCES event_types(id),
    uid             TEXT NOT NULL UNIQUE,
    guest_name      TEXT NOT NULL,
    guest_email     TEXT NOT NULL,
    guest_timezone  TEXT NOT NULL,
    notes           TEXT,
    start_at        TEXT NOT NULL,
    end_at          TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'confirmed',
    cancel_token    TEXT NOT NULL UNIQUE,
    reschedule_token TEXT NOT NULL UNIQUE,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS smtp_config (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    host            TEXT NOT NULL,
    port            INTEGER NOT NULL DEFAULT 587,
    username        TEXT NOT NULL,
    password_enc    TEXT,
    from_email      TEXT NOT NULL,
    from_name       TEXT,
    enabled         INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(account_id)
);

CREATE INDEX IF NOT EXISTS idx_events_calendar ON events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_events_start ON events(start_at);
CREATE INDEX IF NOT EXISTS idx_bookings_start ON bookings(start_at);
CREATE INDEX IF NOT EXISTS idx_bookings_event_type ON bookings(event_type_id);
