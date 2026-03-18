-- Member weight for round-robin priority (higher = assigned first)
ALTER TABLE user_groups ADD COLUMN weight INTEGER NOT NULL DEFAULT 1;
