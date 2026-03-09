# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-03-09

Initial development release. CLI-only, no web interface yet.

### Added

- **Account setup** (`calrs init`) — interactive first-time configuration with name, email, and timezone
- **CalDAV source management** (`calrs source add/list/remove/test`) — connect CalDAV servers (Nextcloud, Fastmail, iCloud, etc.), test connections, hex-encoded credential storage
- **Calendar sync** (`calrs sync`) — pull events from all CalDAV sources via PROPFIND/REPORT, upsert into local SQLite
- **Calendar viewer** (`calrs calendar show`) — display synced events in a table with date range filtering
- **Event types** (`calrs event-type create/list/slots`) — define bookable meeting templates with duration, buffers, and minimum notice. Default Mon–Fri 09:00–17:00 availability rules
- **Availability engine** — compute free slots by intersecting availability rules with synced busy events
- **Booking management** (`calrs booking list/cancel`) — view and cancel bookings
- **SQLite storage** — WAL mode, foreign keys with CASCADE, indexed queries
- **CalDAV client** — minimal RFC 4791 implementation: OPTIONS check, principal discovery, calendar listing, VEVENT fetch
