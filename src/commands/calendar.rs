use anyhow::Result;
use chrono::{Local, NaiveDate};
use colored::Colorize;
use sqlx::SqlitePool;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct EventRow {
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Time")]
    time: String,
    #[tabled(rename = "Summary")]
    summary: String,
    #[tabled(rename = "Calendar")]
    calendar: String,
}

pub async fn run(pool: &SqlitePool, from: Option<String>, to: Option<String>) -> Result<()> {
    let from_date = from
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()?
        .unwrap_or_else(|| Local::now().date_naive());

    let to_date = to
        .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d"))
        .transpose()?
        .unwrap_or_else(|| from_date + chrono::Duration::days(14));

    let from_str = from_date.format("%Y-%m-%d").to_string();
    let to_str = to_date.format("%Y-%m-%d").to_string();

    let events: Vec<(String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT e.start_at, e.end_at, e.summary, c.display_name
         FROM events e
         JOIN calendars c ON e.calendar_id = c.id
         WHERE e.start_at >= ? AND e.start_at <= ?
         ORDER BY e.start_at",
    )
    .bind(&from_str)
    .bind(format!("{}T23:59:59", to_str))
    .fetch_all(pool)
    .await?;

    if events.is_empty() {
        println!("No events from {} to {}.", from_str, to_str);
        return Ok(());
    }

    let rows: Vec<EventRow> = events
        .into_iter()
        .map(|(start_at, end_at, summary, cal_name)| {
            let (date, time) = if start_at.contains('T') {
                let parts: Vec<&str> = start_at.splitn(2, 'T').collect();
                (
                    parts[0].to_string(),
                    format!("{} – {}", parts.get(1).unwrap_or(&""), extract_time(&end_at)),
                )
            } else {
                (start_at, "all-day".to_string())
            };

            EventRow {
                date,
                time,
                summary: summary.unwrap_or_else(|| "(no title)".to_string()),
                calendar: cal_name.unwrap_or_else(|| "—".to_string()),
            }
        })
        .collect();

    println!(
        "{} events from {} to {}:\n",
        rows.len().to_string().bold(),
        from_str,
        to_str
    );
    println!("{}", Table::new(rows));

    Ok(())
}

fn extract_time(dt: &str) -> String {
    if let Some(pos) = dt.find('T') {
        dt[pos + 1..].to_string()
    } else {
        dt.to_string()
    }
}
