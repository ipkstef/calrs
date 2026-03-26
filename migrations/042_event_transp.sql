-- Add TRANSP (transparency) column to events.
-- TRANSP:TRANSPARENT means the event does NOT block time (e.g. "Available" in calendar apps).
-- TRANSP:OPAQUE (or absent) means it DOES block time. NULL treated as OPAQUE.
ALTER TABLE events ADD COLUMN transp TEXT;

-- Backfill from raw_ical for existing events
UPDATE events SET transp = 'TRANSPARENT'
WHERE raw_ical LIKE '%TRANSP:TRANSPARENT%';
