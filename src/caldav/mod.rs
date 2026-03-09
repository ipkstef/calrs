use anyhow::{bail, Result};
use reqwest::Client;

pub struct CaldavClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
}

impl CaldavClient {
    pub fn new(base_url: &str, username: &str, password: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Check if the server supports CalDAV (OPTIONS request)
    pub async fn check_connection(&self) -> Result<bool> {
        let resp = self
            .client
            .request(reqwest::Method::OPTIONS, &self.base_url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        if !resp.status().is_success() {
            bail!(
                "Server returned {} {}",
                resp.status().as_u16(),
                resp.status().canonical_reason().unwrap_or("")
            );
        }

        let dav_header = resp
            .headers()
            .get("DAV")
            .map(|v| v.to_str().unwrap_or(""))
            .unwrap_or("");

        Ok(dav_header.contains("calendar-access"))
    }

    /// Discover the principal URL via PROPFIND
    pub async fn discover_principal(&self) -> Result<String> {
        let body = PROPFIND_PRINCIPAL;
        let resp = self
            .client
            .request(reqwest::Method::from_bytes(b"PROPFIND")?, &self.base_url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Content-Type", "application/xml; charset=utf-8")
            .header("Depth", "0")
            .body(body)
            .send()
            .await?;

        let text = resp.text().await?;
        // Simple extraction of principal href
        if let Some(start) = text.find("<d:href>") {
            if let Some(end) = text[start..].find("</d:href>") {
                let href = &text[start + 8..start + end];
                return Ok(href.to_string());
            }
        }

        bail!("Could not discover principal URL from response")
    }

    /// List calendars under a calendar-home-set URL
    pub async fn list_calendars(&self, home_url: &str) -> Result<Vec<CalendarInfo>> {
        let url = if home_url.starts_with("http") {
            home_url.to_string()
        } else {
            format!("{}{}", self.base_url, home_url)
        };

        let resp = self
            .client
            .request(reqwest::Method::from_bytes(b"PROPFIND")?, &url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Content-Type", "application/xml; charset=utf-8")
            .header("Depth", "1")
            .body(PROPFIND_CALENDARS)
            .send()
            .await?;

        let text = resp.text().await?;
        let calendars = parse_calendar_list(&text);
        Ok(calendars)
    }

    /// Fetch events from a calendar using REPORT
    pub async fn fetch_events(&self, calendar_href: &str) -> Result<Vec<RawEvent>> {
        let url = if calendar_href.starts_with("http") {
            calendar_href.to_string()
        } else {
            format!("{}{}", self.base_url, calendar_href)
        };

        let resp = self
            .client
            .request(reqwest::Method::from_bytes(b"REPORT")?, &url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Content-Type", "application/xml; charset=utf-8")
            .header("Depth", "1")
            .body(REPORT_CALENDAR_DATA)
            .send()
            .await?;

        let text = resp.text().await?;
        let events = parse_event_responses(&text);
        Ok(events)
    }
}

#[derive(Debug, Clone)]
pub struct CalendarInfo {
    pub href: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub ctag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RawEvent {
    pub href: String,
    pub ical_data: String,
}

fn parse_calendar_list(xml: &str) -> Vec<CalendarInfo> {
    // Simplified parser — extracts calendar hrefs and display names
    let mut calendars = Vec::new();
    for response_block in xml.split("<d:response>").skip(1) {
        let href = extract_tag(response_block, "d:href").unwrap_or_default();
        if href.is_empty() {
            continue;
        }
        let display_name = extract_tag(response_block, "d:displayname");
        let color = extract_tag(response_block, "x1:calendar-color");
        let ctag = extract_tag(response_block, "cs:getctag");
        calendars.push(CalendarInfo {
            href,
            display_name,
            color,
            ctag,
        });
    }
    calendars
}

fn parse_event_responses(xml: &str) -> Vec<RawEvent> {
    let mut events = Vec::new();
    for response_block in xml.split("<d:response>").skip(1) {
        let href = extract_tag(response_block, "d:href").unwrap_or_default();
        let ical_data = extract_tag(response_block, "cal:calendar-data").unwrap_or_default();
        if !ical_data.is_empty() {
            events.push(RawEvent { href, ical_data });
        }
    }
    events
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    if let Some(start) = xml.find(&open) {
        let content_start = start + open.len();
        if let Some(end) = xml[content_start..].find(&close) {
            let value = xml[content_start..content_start + end].trim().to_string();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

// --- XML Templates ---

const PROPFIND_PRINCIPAL: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:">
  <d:prop>
    <d:current-user-principal />
  </d:prop>
</d:propfind>"#;

const PROPFIND_CALENDARS: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:" xmlns:cs="http://calendarserver.org/ns/" xmlns:c="urn:ietf:params:xml:ns:caldav" xmlns:x1="http://apple.com/ns/ical/">
  <d:prop>
    <d:resourcetype />
    <d:displayname />
    <x1:calendar-color />
    <cs:getctag />
  </d:prop>
</d:propfind>"#;

const REPORT_CALENDAR_DATA: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<c:calendar-query xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
  <d:prop>
    <d:getetag />
    <c:calendar-data />
  </d:prop>
  <c:filter>
    <c:comp-filter name="VCALENDAR">
      <c:comp-filter name="VEVENT" />
    </c:comp-filter>
  </c:filter>
</c:calendar-query>"#;
