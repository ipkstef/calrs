-- Scheduling mode for group event types: round_robin (default) or collective
ALTER TABLE event_types ADD COLUMN scheduling_mode TEXT NOT NULL DEFAULT 'round_robin' CHECK(scheduling_mode IN ('round_robin', 'collective'));
