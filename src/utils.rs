use std::io::{self, Write};

/// Split an iCal blob into individual VEVENT blocks.
/// A single CalDAV resource can contain multiple VEVENTs when a recurring
/// event has modified instances (RECURRENCE-ID).
pub fn split_vevents(ical: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut search_from = 0;
    while let Some(start) = ical[search_from..].find("BEGIN:VEVENT") {
        let abs_start = search_from + start;
        if let Some(end) = ical[abs_start..].find("END:VEVENT") {
            let abs_end = abs_start + end + "END:VEVENT".len();
            blocks.push(ical[abs_start..abs_end].to_string());
            search_from = abs_end;
        } else {
            break;
        }
    }
    if blocks.is_empty() {
        blocks.push(ical.to_string());
    }
    blocks
}

/// Extract a field value from a single VEVENT block.
pub fn extract_vevent_field(vevent: &str, field: &str) -> Option<String> {
    for line in vevent.lines() {
        if line.starts_with(field) {
            if let Some(colon_pos) = line.find(':') {
                let value = line[colon_pos + 1..].trim().to_string();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }
    }
    None
}

pub fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn prompt_password(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    // TODO: use rpassword for hidden input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- split_vevents ---

    #[test]
    fn split_single_vevent() {
        let ical = "BEGIN:VCALENDAR\nBEGIN:VEVENT\nUID:abc\nEND:VEVENT\nEND:VCALENDAR";
        let blocks = split_vevents(ical);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].starts_with("BEGIN:VEVENT"));
        assert!(blocks[0].ends_with("END:VEVENT"));
    }

    #[test]
    fn split_multiple_vevents() {
        let ical = "\
BEGIN:VCALENDAR\n\
BEGIN:VEVENT\n\
UID:abc\n\
RRULE:FREQ=WEEKLY\n\
END:VEVENT\n\
BEGIN:VEVENT\n\
UID:abc\n\
RECURRENCE-ID:20260309T100000\n\
END:VEVENT\n\
END:VCALENDAR";
        let blocks = split_vevents(ical);
        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].contains("RRULE"));
        assert!(blocks[1].contains("RECURRENCE-ID"));
    }

    #[test]
    fn split_no_vevent_returns_whole() {
        let ical = "BEGIN:VCALENDAR\nEND:VCALENDAR";
        let blocks = split_vevents(ical);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0], ical);
    }

    #[test]
    fn split_missing_end_vevent() {
        let ical = "BEGIN:VCALENDAR\nBEGIN:VEVENT\nUID:abc\n";
        let blocks = split_vevents(ical);
        // No END:VEVENT → falls back to returning whole string
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0], ical);
    }

    // --- extract_vevent_field ---

    #[test]
    fn extract_existing_field() {
        let vevent = "BEGIN:VEVENT\nUID:test-uid-123\nSUMMARY:Team meeting\nEND:VEVENT";
        assert_eq!(extract_vevent_field(vevent, "UID"), Some("test-uid-123".to_string()));
        assert_eq!(extract_vevent_field(vevent, "SUMMARY"), Some("Team meeting".to_string()));
    }

    #[test]
    fn extract_field_with_params() {
        // DTSTART has timezone parameters before the colon
        let vevent = "BEGIN:VEVENT\nDTSTART;TZID=Europe/Paris:20260310T100000\nEND:VEVENT";
        assert_eq!(extract_vevent_field(vevent, "DTSTART"), Some("20260310T100000".to_string()));
    }

    #[test]
    fn extract_nonexistent_field() {
        let vevent = "BEGIN:VEVENT\nUID:abc\nEND:VEVENT";
        assert_eq!(extract_vevent_field(vevent, "SUMMARY"), None);
    }

    #[test]
    fn extract_empty_value() {
        let vevent = "BEGIN:VEVENT\nSUMMARY:\nEND:VEVENT";
        assert_eq!(extract_vevent_field(vevent, "SUMMARY"), None);
    }

    #[test]
    fn extract_does_not_match_substring() {
        // DTSTART should not match DTSTART-EXTRA or other prefixed fields
        let vevent = "BEGIN:VEVENT\nDTSTART:20260310T100000\nDTSTART-EXTRA:ignored\nEND:VEVENT";
        assert_eq!(extract_vevent_field(vevent, "DTSTART"), Some("20260310T100000".to_string()));
    }
}
