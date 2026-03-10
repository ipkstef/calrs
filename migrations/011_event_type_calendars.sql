CREATE TABLE IF NOT EXISTS event_type_calendars (
    event_type_id TEXT NOT NULL REFERENCES event_types(id) ON DELETE CASCADE,
    calendar_id   TEXT NOT NULL REFERENCES calendars(id) ON DELETE CASCADE,
    PRIMARY KEY (event_type_id, calendar_id)
);
