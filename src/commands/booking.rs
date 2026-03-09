use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
use sqlx::SqlitePool;
use tabled::{Table, Tabled};

#[derive(Debug, Subcommand)]
pub enum BookingCommands {
    /// List bookings
    List {
        /// Show only upcoming bookings
        #[arg(long)]
        upcoming: bool,
    },
    /// Cancel a booking
    Cancel {
        /// Booking ID (prefix match)
        id: String,
    },
}

#[derive(Tabled)]
struct BookingRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Guest")]
    guest: String,
    #[tabled(rename = "Event Type")]
    event_type: String,
    #[tabled(rename = "When")]
    when: String,
    #[tabled(rename = "Status")]
    status: String,
}

pub async fn run(pool: &SqlitePool, cmd: BookingCommands) -> Result<()> {
    match cmd {
        BookingCommands::List { upcoming } => {
            let query = if upcoming {
                "SELECT b.id, b.guest_name, b.guest_email, et.title, b.start_at, b.end_at, b.status
                 FROM bookings b
                 JOIN event_types et ON b.event_type_id = et.id
                 WHERE b.start_at >= datetime('now')
                 ORDER BY b.start_at"
            } else {
                "SELECT b.id, b.guest_name, b.guest_email, et.title, b.start_at, b.end_at, b.status
                 FROM bookings b
                 JOIN event_types et ON b.event_type_id = et.id
                 ORDER BY b.start_at DESC"
            };

            let bookings: Vec<(String, String, String, String, String, String, String)> =
                sqlx::query_as(query).fetch_all(pool).await?;

            if bookings.is_empty() {
                println!("No bookings found.");
                return Ok(());
            }

            let rows: Vec<BookingRow> = bookings
                .into_iter()
                .map(
                    |(id, guest_name, guest_email, title, start, end, status)| {
                        let time = if start.contains('T') {
                            let date = &start[..10];
                            let start_time = &start[11..16];
                            let end_time = if end.len() > 16 { &end[11..16] } else { &end };
                            format!("{} {} – {}", date, start_time, end_time)
                        } else {
                            start
                        };

                        BookingRow {
                            id: id[..8].to_string(),
                            guest: format!("{} <{}>", guest_name, guest_email),
                            event_type: title,
                            when: time,
                            status,
                        }
                    },
                )
                .collect();

            println!("{}", Table::new(rows));
        }
        BookingCommands::Cancel { id } => {
            let full_id: Option<(String,)> = sqlx::query_as(
                "SELECT id FROM bookings WHERE id LIKE ? || '%' AND status = 'confirmed'",
            )
            .bind(&id)
            .fetch_optional(pool)
            .await?;

            match full_id {
                Some((full_id,)) => {
                    sqlx::query(
                        "UPDATE bookings SET status = 'cancelled' WHERE id = ?",
                    )
                    .bind(&full_id)
                    .execute(pool)
                    .await?;
                    println!("{} Booking {} cancelled.", "✓".green(), &full_id[..8]);
                }
                None => {
                    println!(
                        "{} No confirmed booking found matching '{}'",
                        "✗".red(),
                        id
                    );
                }
            }
        }
    }

    Ok(())
}
