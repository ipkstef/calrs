CREATE TABLE IF NOT EXISTS booking_frequency_limits (
    id TEXT PRIMARY KEY,
    event_type_id TEXT NOT NULL REFERENCES event_types(id) ON DELETE CASCADE,
    max_bookings INTEGER NOT NULL,
    period TEXT NOT NULL CHECK(period IN ('day', 'week', 'month', 'year'))
);
CREATE INDEX IF NOT EXISTS idx_bfl_event_type ON booking_frequency_limits(event_type_id);
