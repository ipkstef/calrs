# calrs

**Fast, self-hostable scheduling. Like Cal.com, but written in Rust.**

> _"Your time, your stack."_

## What is this?

`calrs` is an open-source scheduling platform built in Rust. Connect your CalDAV calendar (Nextcloud, Fastmail, iCloud, Google…), define bookable meeting types, and share a link. No Node.js, no PostgreSQL, no subscription.

## Status

Early development. CLI is functional. Web interface coming next.

## Quick start

```bash
# Build
cargo build --release

# Initialize
calrs init

# Connect your CalDAV calendar (e.g. Nextcloud)
calrs source add --url https://nextcloud.example.com/remote.php/dav \
                 --username alice --name Nextcloud

# Pull events
calrs sync

# Create a bookable meeting type
calrs event-type create --title "30min intro call" --slug intro --duration 30

# Check your availability
calrs event-type slots intro

# See your upcoming events
calrs calendar show
```

## CLI reference

```
calrs init                        First-time setup
calrs source add                  Connect a CalDAV calendar
calrs source list                 List connected sources
calrs source remove <id>          Remove a source
calrs source test <id>            Test a connection
calrs sync [--full]               Pull latest events from CalDAV
calrs event-type create           Define a new bookable meeting
calrs event-type list             List your event types
calrs event-type slots <slug>     Show available slots
calrs calendar show [--from] [--to]  View your calendar
calrs booking list [--upcoming]   View bookings
calrs booking cancel <id>         Cancel a booking
```

## Architecture

```
calrs/
├── Cargo.toml
├── CLAUDE.md
├── README.md
├── migrations/
│   └── 001_initial.sql        SQLite schema
└── src/
    ├── main.rs                CLI entry point (clap)
    ├── db.rs                  SQLite connection + migrations
    ├── models.rs              Domain types
    ├── caldav/
    │   └── mod.rs             CalDAV client (RFC 4791)
    └── commands/
        ├── mod.rs             Re-exports
        ├── init.rs            calrs init
        ├── source.rs          calrs source add/list/remove/test
        ├── sync.rs            calrs sync
        ├── calendar.rs        calrs calendar show
        ├── event_type.rs      calrs event-type create/list/slots
        └── booking.rs         calrs booking list/cancel
```

**Storage:** SQLite (WAL mode). Single file, zero ops.

**CalDAV:** Pull-based sync. Reads your existing calendars for free/busy.
Does not write to your CalDAV server (bookings are stored locally, with optional push).

## Roadmap

- [x] CalDAV sync (pull)
- [x] SQLite storage
- [x] CLI availability viewer
- [ ] Web booking page (Axum + HTMX, no JS framework)
- [ ] Email notifications (SMTP)
- [ ] iCal `.ics` generation for booking confirmations
- [ ] CalDAV write (push confirmed bookings back to your calendar)
- [ ] Recurrence rule expansion
- [ ] Multi-timezone support
- [ ] Docker image

## License

AGPL-3.0 — free to use, modify, and self-host. Contributions welcome.
