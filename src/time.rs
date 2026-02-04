use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, TimeZone, Weekday};

pub fn parse_time(input: &str) -> Option<DateTime<Local>> {
    let input = input.trim().to_lowercase();

    // Handle relative times: "in 2 hours", "in 3 days"
    if input.starts_with("in ") {
        return parse_relative(&input[3..]);
    }

    // Handle "today"
    if input == "today" {
        return Some(Local::now().date_naive().and_hms_opt(23, 59, 0)?.and_local_timezone(Local).single()?);
    }

    if input.starts_with("today ") {
        let time_part = &input[6..];
        let time = parse_time_only(time_part)?;
        let date = Local::now().date_naive();
        return date.and_time(time).and_local_timezone(Local).single();
    }

    // Handle "tomorrow"
    if input == "tomorrow" {
        return Some(Local::now().date_naive().succ_opt()?.and_hms_opt(9, 0, 0)?.and_local_timezone(Local).single()?);
    }

    if input.starts_with("tomorrow ") {
        let time_part = &input[9..];
        let time = parse_time_only(time_part)?;
        let date = Local::now().date_naive().succ_opt()?;
        return date.and_time(time).and_local_timezone(Local).single();
    }

    // Handle weekday names: "monday", "fri", "fri 3pm"
    if let Some(result) = parse_weekday_input(&input) {
        return Some(result);
    }

    // Handle time only: "11am", "3:30pm", "14:00"
    if let Some(time) = parse_time_only(&input) {
        let today = Local::now().date_naive();
        return today.and_time(time).and_local_timezone(Local).single();
    }

    // Handle date formats: "12/25", "2024-12-25"
    if let Some(result) = parse_date_input(&input) {
        return Some(result);
    }

    None
}

fn parse_relative(input: &str) -> Option<DateTime<Local>> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let amount: i64 = parts[0].parse().ok()?;
    let unit = parts[1];

    let duration = match unit {
        "hour" | "hours" | "h" => Duration::hours(amount),
        "day" | "days" | "d" => Duration::days(amount),
        "week" | "weeks" | "w" => Duration::weeks(amount),
        "minute" | "minutes" | "min" | "m" => Duration::minutes(amount),
        _ => return None,
    };

    Some(Local::now() + duration)
}

fn parse_time_only(input: &str) -> Option<NaiveTime> {
    let input = input.trim().to_lowercase();

    // Handle "3pm", "11am"
    if input.ends_with("am") || input.ends_with("pm") {
        let is_pm = input.ends_with("pm");
        let time_str = &input[..input.len() - 2];

        if let Some((hour, minute)) = parse_hour_minute(time_str) {
            let hour = if is_pm && hour != 12 {
                hour + 12
            } else if !is_pm && hour == 12 {
                0
            } else {
                hour
            };
            return NaiveTime::from_hms_opt(hour, minute, 0);
        }
    }

    // Handle "14:00", "9:30"
    if input.contains(':') {
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() == 2 {
            let hour: u32 = parts[0].parse().ok()?;
            let minute: u32 = parts[1].parse().ok()?;
            return NaiveTime::from_hms_opt(hour, minute, 0);
        }
    }

    None
}

fn parse_hour_minute(input: &str) -> Option<(u32, u32)> {
    if input.contains(':') {
        let parts: Vec<&str> = input.split(':').collect();
        if parts.len() == 2 {
            let hour: u32 = parts[0].parse().ok()?;
            let minute: u32 = parts[1].parse().ok()?;
            return Some((hour, minute));
        }
    } else {
        let hour: u32 = input.parse().ok()?;
        return Some((hour, 0));
    }
    None
}

fn parse_weekday_input(input: &str) -> Option<DateTime<Local>> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let weekday_str = parts[0];

    let weekday = match weekday_str {
        "monday" | "mon" => Weekday::Mon,
        "tuesday" | "tue" => Weekday::Tue,
        "wednesday" | "wed" => Weekday::Wed,
        "thursday" | "thu" => Weekday::Thu,
        "friday" | "fri" => Weekday::Fri,
        "saturday" | "sat" => Weekday::Sat,
        "sunday" | "sun" => Weekday::Sun,
        _ => return None,
    };

    let time = if parts.len() > 1 {
        parse_time_only(parts[1])?
    } else {
        NaiveTime::from_hms_opt(9, 0, 0)?
    };

    let today = Local::now().date_naive();
    let today_weekday = today.weekday();

    let days_until = (weekday.num_days_from_monday() as i64 - today_weekday.num_days_from_monday() as i64 + 7) % 7;
    let days_until = if days_until == 0 { 7 } else { days_until };

    let target_date = today + Duration::days(days_until);
    target_date.and_time(time).and_local_timezone(Local).single()
}

fn parse_date_input(input: &str) -> Option<DateTime<Local>> {
    let now = Local::now();

    // Handle "12/25" or "12-25"
    if input.len() <= 5 && (input.contains('/') || input.contains('-')) {
        let sep = if input.contains('/') { '/' } else { '-' };
        let parts: Vec<&str> = input.split(sep).collect();
        if parts.len() == 2 {
            let month: u32 = parts[0].parse().ok()?;
            let day: u32 = parts[1].parse().ok()?;
            let date = Local.with_ymd_and_hms(now.year(), month, day, 9, 0, 0).single()?;
            return Some(date);
        }
    }

    // Handle "2024-12-25"
    if input.len() == 10 && input.chars().filter(|c| *c == '-').count() == 2 {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() == 3 {
            let year: i32 = parts[0].parse().ok()?;
            let month: u32 = parts[1].parse().ok()?;
            let day: u32 = parts[2].parse().ok()?;
            return Local.with_ymd_and_hms(year, month, day, 9, 0, 0).single();
        }
    }

    None
}

pub fn format_deadline(deadline: DateTime<Local>, is_overdue: bool) -> String {
    let now = Local::now();
    let today = now.date_naive();
    let deadline_date = deadline.date_naive();
    let time_str = deadline.format("%-I:%M%P").to_string();

    if is_overdue {
        let diff = now - deadline;
        if diff.num_hours() < 1 {
            format!("⚠ {}m ago", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("⚠ {}h ago", diff.num_hours())
        } else if diff.num_days() == 1 {
            "⚠ yesterday".to_string()
        } else {
            format!("⚠ {}d ago", diff.num_days())
        }
    } else if deadline_date == today {
        format!("today {}", time_str)
    } else if deadline_date == today.succ_opt().unwrap_or(today) {
        format!("tomorrow {}", time_str)
    } else if (deadline_date - today).num_days() < 7 {
        deadline.format("%a %-I:%M%P").to_string().to_lowercase()
    } else if deadline.year() == now.year() {
        deadline.format("%b %-d").to_string().to_lowercase()
    } else {
        deadline.format("%Y-%m-%d").to_string()
    }
}

pub fn format_completed_time(completed: DateTime<Local>) -> String {
    let now = Local::now();
    let diff = now - completed;

    if diff.num_minutes() < 1 {
        "done just now".to_string()
    } else if diff.num_minutes() < 60 {
        format!("done {}m ago", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("done {}h ago", diff.num_hours())
    } else {
        format!("done {}d ago", diff.num_days())
    }
}

pub fn is_due_today(deadline: DateTime<Local>) -> bool {
    deadline.date_naive() == Local::now().date_naive()
}

pub fn is_due_this_week(deadline: DateTime<Local>) -> bool {
    let now = Local::now();
    let diff = deadline.date_naive() - now.date_naive();
    diff.num_days() >= 0 && diff.num_days() <= 7
}
