-- Unified Teams: replace groups-as-scheduling-units and team links
-- with a single Teams concept.

-- Step 1: Create teams tables
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT UNIQUE,
    description TEXT,
    avatar_path TEXT,
    visibility TEXT NOT NULL DEFAULT 'public' CHECK(visibility IN ('public', 'private')),
    invite_token TEXT UNIQUE,
    created_by TEXT REFERENCES users(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS team_members (
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'member' CHECK(role IN ('admin', 'member')),
    source TEXT NOT NULL DEFAULT 'direct' CHECK(source IN ('direct', 'group')),
    PRIMARY KEY (team_id, user_id)
);

CREATE TABLE IF NOT EXISTS team_groups (
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    group_id TEXT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, group_id)
);

-- Step 2: Add team_id to event_types
ALTER TABLE event_types ADD COLUMN team_id TEXT REFERENCES teams(id) ON DELETE CASCADE;

-- Step 3: Migrate groups with event types → public teams
INSERT INTO teams (id, name, slug, description, avatar_path, visibility, created_by, created_at)
SELECT g.id, g.name, g.slug, g.description, g.avatar_path, 'public', NULL, g.created_at
FROM groups g
WHERE EXISTS (SELECT 1 FROM event_types et WHERE et.group_id = g.id);

-- Link OIDC groups to their teams
INSERT INTO team_groups (team_id, group_id)
SELECT g.id, g.id
FROM groups g
WHERE EXISTS (SELECT 1 FROM event_types et WHERE et.group_id = g.id);

-- Copy group members to team members
INSERT INTO team_members (team_id, user_id, role, source)
SELECT ug.group_id, ug.user_id, 'member', 'group'
FROM user_groups ug
WHERE EXISTS (SELECT 1 FROM teams t WHERE t.id = ug.group_id);

-- Set team_id on group event types
UPDATE event_types SET team_id = group_id WHERE group_id IS NOT NULL;

-- Step 4: Migrate team links → private teams
-- Each team link becomes a private team with an invite token
INSERT INTO teams (id, name, slug, description, avatar_path, visibility, invite_token, created_by, created_at)
SELECT tl.id, tl.title,
       LOWER(REPLACE(REPLACE(REPLACE(REPLACE(TRIM(tl.title), ' ', '-'), '''', ''), '"', ''), '.', '')),
       tl.description, NULL, 'private', tl.token, tl.created_by_user_id, tl.created_at
FROM team_links tl;

-- Copy team link members to team members (creator = admin)
INSERT INTO team_members (team_id, user_id, role, source)
SELECT tlm.team_link_id, tlm.user_id,
       CASE WHEN tlm.user_id = (SELECT created_by_user_id FROM team_links WHERE id = tlm.team_link_id) THEN 'admin' ELSE 'member' END,
       'direct'
FROM team_link_members tlm;

-- Ensure creator is always a team member (may not be in team_link_members)
INSERT OR IGNORE INTO team_members (team_id, user_id, role, source)
SELECT tl.id, tl.created_by_user_id, 'admin', 'direct'
FROM team_links tl;
