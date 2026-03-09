use anyhow::Result;
use colored::Colorize;
use sqlx::SqlitePool;
use uuid::Uuid;

use std::io::{self, Write};

fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_with_default(label: &str, default: &str) -> String {
    print!("{} [{}]: ", label, default);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

pub async fn run(pool: &SqlitePool) -> Result<()> {
    // Check if account already exists
    let existing: Option<(String,)> =
        sqlx::query_as("SELECT id FROM accounts LIMIT 1")
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        println!("{} Account already initialized.", "✓".green());
        return Ok(());
    }

    println!("{}", "Welcome to calrs! Let's set up your account.".bold());
    println!();

    let name = prompt("Your name");
    let email = prompt("Your email");
    let timezone = prompt_with_default("Timezone (IANA)", "UTC");

    let id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO accounts (id, name, email, timezone) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&name)
        .bind(&email)
        .bind(&timezone)
        .execute(pool)
        .await?;

    println!();
    println!("{} Account created for {} <{}>", "✓".green(), name, email);
    println!(
        "{}",
        "Next: add a CalDAV source with `calrs source add`".dimmed()
    );

    Ok(())
}
