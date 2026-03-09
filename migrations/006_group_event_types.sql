-- Add slug to groups for URL-friendly names
ALTER TABLE groups ADD COLUMN slug TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_groups_slug ON groups(slug);

-- Add group_id to event_types (NULL = personal)
ALTER TABLE event_types ADD COLUMN group_id TEXT REFERENCES groups(id) ON DELETE CASCADE;
CREATE UNIQUE INDEX IF NOT EXISTS idx_event_types_group_slug ON event_types(group_id, slug) WHERE group_id IS NOT NULL;

-- Track who created group event types
ALTER TABLE event_types ADD COLUMN created_by_user_id TEXT REFERENCES users(id) ON DELETE SET NULL;

-- Track assigned member for group bookings
ALTER TABLE bookings ADD COLUMN assigned_user_id TEXT REFERENCES users(id) ON DELETE SET NULL;
