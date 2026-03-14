-- Track whether a reschedule was initiated by the host (guest picks new time → confirmed directly)
ALTER TABLE bookings ADD COLUMN reschedule_by_host INTEGER NOT NULL DEFAULT 0;
