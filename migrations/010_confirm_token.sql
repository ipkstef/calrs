-- Add confirm_token for email-based approve/decline of pending bookings.
ALTER TABLE bookings ADD COLUMN confirm_token TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_bookings_confirm_token ON bookings(confirm_token) WHERE confirm_token IS NOT NULL;
