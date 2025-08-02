use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Returns current UTC timestamp.
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

/// Formats timestamp to ISO-8601 string.
pub fn format_timestamp(ts: OffsetDateTime) -> String {
    ts.format(&Rfc3339)
        .unwrap_or_else(|_| "invalid-timestamp".into())
}

/// Parses ISO-8601 string to OffsetDateTime.
pub fn parse_timestamp(s: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(s, &Rfc3339).ok()
}
