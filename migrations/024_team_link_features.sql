-- Feature parity: location, description, reminder for team links
ALTER TABLE team_links ADD COLUMN location_type TEXT;
ALTER TABLE team_links ADD COLUMN location_value TEXT;
ALTER TABLE team_links ADD COLUMN description TEXT;
ALTER TABLE team_links ADD COLUMN reminder_minutes INTEGER;

-- Track reminders for team link bookings
ALTER TABLE team_link_bookings ADD COLUMN reminder_sent_at TEXT;
