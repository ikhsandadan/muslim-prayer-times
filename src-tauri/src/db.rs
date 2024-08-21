use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use chrono::{NaiveDate, Duration, Datelike};

// PrayerRecord structure
#[derive(Serialize, Deserialize)]
pub struct PrayerRecord {
    pub user_id: i32,
    pub date: String,
    pub fajr: bool,
    pub dhuhr: bool,
    pub asr: bool,
    pub maghrib: bool,
    pub isha: bool,
}

// create table if not exists
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS prayer_records (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            fajr BOOLEAN NOT NULL,
            dhuhr BOOLEAN NOT NULL,
            asr BOOLEAN NOT NULL,
            maghrib BOOLEAN NOT NULL,
            isha BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(())
}

// add or update prayer record
pub fn add_or_update_prayer_record(conn: &Connection, record: &PrayerRecord) -> Result<()> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM prayer_records WHERE user_id = ?1 AND date = ?2")?;
    let count: i32 = stmt.query_row(params![record.user_id, record.date], |row| row.get(0))?;

    if count > 0 {
        conn.execute(
            "UPDATE prayer_records SET fajr = ?1, dhuhr = ?2, asr = ?3, maghrib = ?4, isha = ?5 WHERE user_id = ?6 AND date = ?7",
            params![
                record.fajr,
                record.dhuhr,
                record.asr,
                record.maghrib,
                record.isha,
                record.user_id,
                record.date,
            ],
        )?;
    } else {
        conn.execute(
            "INSERT INTO prayer_records (user_id, date, fajr, dhuhr, asr, maghrib, isha) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                record.user_id,
                record.date,
                record.fajr,
                record.dhuhr,
                record.asr,
                record.maghrib,
                record.isha,
            ],
        )?;
    }
    Ok(())
}

// get prayer record by date
pub fn get_prayer_records_by_date(conn: &Connection, date: &str) -> Result<Vec<PrayerRecord>> {
    let mut stmt = conn.prepare("SELECT user_id, date, fajr, dhuhr, asr, maghrib, isha FROM prayer_records WHERE date = ?1")?;
    let prayer_iter = stmt.query_map(params![date], |row| {
        Ok(PrayerRecord {
            user_id: row.get(0)?,
            date: row.get(1)?,
            fajr: row.get(2)?,
            dhuhr: row.get(3)?,
            asr: row.get(4)?,
            maghrib: row.get(5)?,
            isha: row.get(6)?,
        })
    })?;

    let mut prayers = Vec::new();
    for prayer in prayer_iter {
        prayers.push(prayer?);
    }
    Ok(prayers)
}

// get monthly prayer data
pub fn get_monthly_prayer_data(conn: &Connection, user_id: i32, year: i32, month: u32) -> Result<Value> {
    // Get the number of days in the specified month
    let days_in_month = NaiveDate::from_ymd_opt(year, month, 1)
        .map(|d| d.with_month(month + 1).unwrap_or(d.with_year(year + 1).unwrap().with_month(1).unwrap()))
        .map(|next_month| next_month - chrono::Duration::days(1))
        .map(|last_day| last_day.day())
        .unwrap_or(31);

    let mut prayer_data = Vec::new();

    for day in 1..=days_in_month {
        let date = format!("{:04}-{:02}-{:02}", year, month, day);
        let query = "SELECT fajr, dhuhr, asr, maghrib, isha FROM prayer_records WHERE user_id = ?1 AND date = ?2";
        let mut stmt = conn.prepare(query)?;

        let result: Result<(bool, bool, bool, bool, bool)> = stmt.query_row(params![user_id, date], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        });

        let (fajr, dhuhr, asr, maghrib, isha) = match result {
            Ok(data) => data,
            Err(_) => (false, false, false, false, false), // No record for this day
        };

        let day_data = json!({
            "date": date,
            "fajr": fajr,
            "dhuhr": dhuhr,
            "asr": asr,
            "maghrib": maghrib,
            "isha": isha,
        });

        prayer_data.push(day_data);
    }

    Ok(json!({
        "year": year,
        "month": month,
        "data": prayer_data,
    }))
}

// get prayer data in range
pub fn get_prayer_data_in_range(conn: &Connection, user_id: i32, start_date: &str, end_date: &str) -> Result<Value> {
    let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").map_err(|_e| rusqlite::Error::InvalidQuery)?;
    let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").map_err(|_e| rusqlite::Error::InvalidQuery)?;
    let mut prayer_data = Vec::new();

    let mut current_date = start_date;
    while current_date <= end_date {
        let date = current_date.format("%Y-%m-%d").to_string();
        let query = "SELECT fajr, dhuhr, asr, maghrib, isha FROM prayer_records WHERE user_id = ?1 AND date = ?2";
        let mut stmt = conn.prepare(query)?;

        let result: Result<(bool, bool, bool, bool, bool)> = stmt.query_row(params![user_id, date], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        });

        let (fajr, dhuhr, asr, maghrib, isha) = match result {
            Ok(data) => data,
            Err(_) => (false, false, false, false, false), // No record for this day
        };

        let day_data = json!({
            "date": date,
            "fajr": fajr,
            "dhuhr": dhuhr,
            "asr": asr,
            "maghrib": maghrib,
            "isha": isha,
        });

        prayer_data.push(day_data);
        current_date += Duration::days(1);
    }

    Ok(json!({
        "start_date": start_date.format("%Y-%m-%d").to_string(),
        "end_date": end_date.format("%Y-%m-%d").to_string(),
        "data": prayer_data,
    }))
}