-- Fix: unique constraint on events must be per-calendar, not global.
-- When multiple users sync the same event (shared calendars, attendees),
-- the global (uid, recurrence_id) constraint caused ON CONFLICT to
-- overwrite calendar_id, making events invisible to some users.

-- Drop the old global unique index
DROP INDEX IF EXISTS idx_events_uid_recurrence;

-- Create the new per-calendar unique index
CREATE UNIQUE INDEX idx_events_cal_uid_recurrence ON events(calendar_id, uid, COALESCE(recurrence_id, ''));
