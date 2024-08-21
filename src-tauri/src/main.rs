// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod model;
mod heatmap;

use db::{PrayerRecord, create_table, add_or_update_prayer_record, get_prayer_records_by_date, get_monthly_prayer_data, get_prayer_data_in_range};
use model::{Location, TodayVerse, QuranData, Surah, Ayah, AyahTranslation};
use heatmap::generate_prayer_heatmap_svg;
use rusqlite::Connection;
use reqwest;
use serde_json::{Value, json};
use tauri::{command, State, Window};
use chrono::{DateTime, FixedOffset, Local, NaiveTime, NaiveDate, Duration};
use geolocation;
use rand::Rng;
use std::{io::Cursor, sync::{Arc, Mutex}, time::{Duration as StdDuration, Instant}, thread};
use rodio::{Decoder, OutputStream, Sink, Source};
use futures::future::join_all;

// App state structure
struct AppState {
  current_ayah: Mutex<Option<Arc<Sink>>>,
  is_playing: Mutex<bool>,
}

// get local time
#[command]
fn get_local_time() -> String {
  let now = Local::now().to_rfc3339();
  now
}

// get local date
#[command]
fn local_date() -> Result<String, String> {
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).map_err(|e| format!("Unexpected error at parsing date: {}", e.to_string()))?;

  let formatted_date = datetime.format("%d-%m-%Y").to_string();
  Ok(formatted_date)
}

// get formatted local date
#[command]
fn formatted_date() -> Result<String, String> {
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).map_err(|e| format!("Unexpected error at formatting date: {}", e.to_string()))?;

  let formatted_date = datetime.format("%-d %B %Y").to_string();
  Ok(formatted_date)
}

// get local clock
#[command]
fn local_clock() -> Result<String, String> {
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).map_err(|e| format!("Unexpected error at parsing clock: {}", e.to_string()))?;

  let formatted_date = datetime.format("%H:%M:%S").to_string();
  Ok(formatted_date)
}

// get user ip
async fn get_ip() -> Result<String, String> {
  let url = "https://api.ipify.org?format=json";

  let req = reqwest::get(url).await.map_err(|e| format!("Unexpected error at parsing ip: {}", e.to_string()))?;
  let res: Value = req.json().await.map_err(|e| format!("Unexpected error at ip json parsing: {}", e.to_string()))?;

  let ip = res["ip"].as_str().ok_or("IP not found in response".to_string())?.to_string();

  Ok(ip)
}

// get user location
#[command]
async fn get_location() -> Result<Location, String> {
  let ip = get_ip().await.map_err(|e| format!("Unexpected error at parsing ip: {}", e))?;

  let locator = geolocation::find(&ip).map_err(|e| format!("Unexpected error at parsing location: {}", e))?;

  let location = Location {
    ip: locator.ip,
    latitude: locator.latitude,
    longitude: locator.longitude,
    city: locator.city,
    region: locator.region,
    country: locator.country,
    timezone: locator.timezone,
    location: locator.location,
  };

  Ok(location)
}

// generate random number
fn generate_random_number() -> u32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..=6236)
}

// add user prayer record to database
#[command]
async fn add_prayer(user_id: i32, date: String, fajr: bool, dhuhr: bool, asr: bool, maghrib: bool, isha: bool) -> Result<(), String> {
  let conn = Connection::open("prayer_tracker.db").map_err(|e| format!("Unexpected error at opening database: {}", e.to_string()))?;
  create_table(&conn).map_err(|e| format!("Unexpected error at creating table: {}", e.to_string()))?;
  let record = PrayerRecord {
    user_id,
    date,
    fajr,
    dhuhr,
    asr,
    maghrib,
    isha,
  };

  add_or_update_prayer_record(&conn, &record).map_err(|e| format!("Unexpected error at inserting prayer record: {}", e.to_string()))?;
  Ok(())
}

// get user prayer record from database
#[command]
async fn get_prayer_data_by_date(date: String) -> Result<Vec<PrayerRecord>, String> {
  let conn = Connection::open("prayer_tracker.db").map_err(|e| format!("Unexpected error at opening database: {}", e.to_string()))?;
  let records = get_prayer_records_by_date(&conn, &date).map_err(|e| format!("Unexpected error at fetching prayer record: {}", e.to_string()))?;
  Ok(records)
}

// get user monthly prayer data
#[command]
async fn get_prayer_data_by_month(user_id: i32, year: i32, month: u32) -> Result<Value, String> {
  let conn = Connection::open("prayer_tracker.db")
    .map_err(|e| format!("Unexpected error at opening database: {}", e.to_string()))?;
  
  get_monthly_prayer_data(&conn, user_id, year, month)
    .map_err(|e| format!("Unexpected error at getting monthly prayer data: {}", e.to_string()))
}

// get user prayer data in range
#[command]
async fn get_prayer_data_by_range(user_id: i32, start_date: &str, end_date: &str) -> Result<Value, String> {
  let conn = Connection::open("prayer_tracker.db")
    .map_err(|e| format!("Unexpected error at opening database: {}", e.to_string()))?;
  
  get_prayer_data_in_range(&conn, user_id, start_date, end_date)
    .map_err(|e| format!("Unexpected error at getting prayer data in range: {}", e.to_string()))
}

// generate heatmap by month
#[command]
fn get_prayer_heatmap_by_month(user_id: i32, year: i32, month: u32) -> Result<String, String> {
  let month_name = NaiveDate::from_ymd_opt(year, month, 1)
    .map(|date| date.format("%B").to_string())
    .unwrap_or_else(|| "Invalid month".to_string());

  let description = format!("Prayer Record for {} {}", month_name, year);

  let conn = Connection::open("prayer_tracker.db")
    .map_err(|e| format!("Unexpected error at opening database: {}", e.to_string()))?;

  match get_monthly_prayer_data(&conn, user_id, year, month) {
    Ok(prayer_data) => {
      let svg = generate_prayer_heatmap_svg(&prayer_data, description);
      Ok(svg)
    },
    Err(err) => Err(format!("Error generating heatmap: {}", err)),
  }
}

// generate heatmap by range
#[command]
fn get_prayer_heatmap_by_range(user_id: i32, start_date: &str, end_date: &str) -> Result<String, String> {
  let start_date_parsed = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
    .map_err(|e| format!("Invalid start date format: {}", e.to_string()))?;

  let end_date_parsed = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
    .map_err(|e| format!("Invalid end date format: {}", e.to_string()))?;

  let start_date_formatted = start_date_parsed.format("%d %b %Y").to_string();
  let end_date_formatted = end_date_parsed.format("%d %b %Y").to_string();

  let description = format!("Prayer Record for {} - {}", start_date_formatted, end_date_formatted);

  let conn = Connection::open("prayer_tracker.db")
    .map_err(|e| format!("Unexpected error at opening database: {}", e.to_string()))?;

  match get_prayer_data_in_range(&conn, user_id, start_date, end_date) {
    Ok(prayer_data) => {
      let svg = generate_prayer_heatmap_svg(&prayer_data, description);
      Ok(svg)
    },
    Err(err) => Err(format!("Error generating heatmap: {}", err)),
  }
}

// get this month data
#[command]
async fn get_this_month_data() -> Result<Value, String> {
  // Get the location (latitude and longitude)
  let location = get_location().await.map_err(|e| format!("Unexpected error at parsing location for this month data: {}", e.to_string()))?;
  let lat = location.latitude;
  let lon = location.longitude;

  // Get the current year and month
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).map_err(|e| format!("Unexpected error at parsing date for this month data: {}", e.to_string()))?;
  let year = datetime.format("%Y").to_string();
  let month = datetime.format("%m").to_string();

  // API URL to fetch the prayer times for the whole month
  let url = format!("https://api.aladhan.com/v1/calendar/{}/{}?latitude={}&longitude={}", year, month, lat, lon);
  
  // Fetch the response from the API
  let response = reqwest::get(&url).await.map_err(|e| format!("Unexpected error at fetching this month data: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing this month data: {}", e.to_string()))?;

  Ok(req)
}

// get hijri calendar
#[command]
async fn get_hijri_calendar() -> Result<Value, String> {
  // Get the current year and month
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).map_err(|e| format!("Unexpected error at parsing date for hijri calendar: {}", e.to_string()))?;
  let year = datetime.format("%Y").to_string();
  let month = datetime.format("%m").to_string();

  // API URL for fetching hijri calendar
  let url = format!("https://api.aladhan.com/v1/gToHCalendar/{}/{}", month, year);
  
  // Fetch the response from the API
  let response = reqwest::get(url).await.map_err(|e| format!("Unexpected error at fetching hijri calendar: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing hijri calendar: {}", e.to_string()))?;

  // Check if the response contains data
  let data = req["data"].as_array().ok_or("No data found in API response for hijri calendar.")?;

  // Process the data
  let mut processed_data = Vec::new();

  // Iterate over the data and process each entry
  for entry in data {
    if let Some(hijri) = entry["hijri"].as_object() {
      let mut processed_entry = json!({
        "day": hijri.get("day").cloned().unwrap_or(Value::Null),
        "month": hijri.get("month").and_then(|m| m.get("en")).cloned().unwrap_or(Value::Null),
        "year": hijri.get("year").cloned().unwrap_or(Value::Null),
      });

      // Check if the entry contains any holidays
      if let Some(holidays) = hijri.get("holidays").and_then(|h| h.as_array()) {
        if !holidays.is_empty() {
          processed_entry["holiday"] = json!(holidays[0]);
        }
      }

      processed_data.push(processed_entry);
    }
  }

  // Return the processed data
  Ok(Value::Array(processed_data))
}

// get hijri calendar
#[command]
async fn get_hijri_calendar_by_month(month: String, year: String) -> Result<Value, String> {
  // API URL for fetching hijri calendar
  let url = format!("https://api.aladhan.com/v1/gToHCalendar/{}/{}", month, year);
  
  // Fetch the response from the API
  let response = reqwest::get(&url).await.map_err(|e| format!("Unexpected error at fetching hijri calendar by month: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing hijri calendar by month: {}", e.to_string()))?;

  // Check if the response contains data
  let data = req["data"].as_array().ok_or("No data found in API response for hijri calendar by month.")?;

  // Process the data
  let mut processed_data = Vec::new();
  let mut futures = Vec::new();

  // Iterate over the data and process each entry
  for entry in data {
    if let Some(hijri) = entry["hijri"].as_object() {
      let mut processed_entry = json!({
        "date": hijri.get("date").cloned().unwrap_or(Value::Null),
        "day": hijri.get("day").cloned().unwrap_or(Value::Null),
        "month": hijri.get("month").and_then(|m| m.get("en")).cloned().unwrap_or(Value::Null),
        "year": hijri.get("year").cloned().unwrap_or(Value::Null),
      });

      // Check if the entry contains any holidays
      if let Some(holidays) = hijri.get("holidays").and_then(|h| h.as_array()) {
        if !holidays.is_empty() {
          processed_entry["holidays"] = json!(holidays);
        }
      }

      // Convert Hijri date to Gregorian date
      if let Some(hijri_date) = hijri.get("date").and_then(|d| d.as_str()) {
        let url = format!("https://api.aladhan.com/v1/hToG/{}", hijri_date);
        futures.push(async move {
          let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
          let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing Hijri date to Gregorian date: {}", e.to_string()))?;
          if let Some(gregorian_date) = req["data"]["gregorian"]["date"].as_str() {
            processed_entry["gregorian_date"] = json!(gregorian_date);
          }
          Ok(processed_entry)
        });
      } else {
        processed_data.push(processed_entry);
      }
    }
  }

  // Await all futures and collect results
  let results: Vec<Result<Value, String>> = join_all(futures).await;
  for result in results {
    if let Ok(entry) = result {
      processed_data.push(entry);
    }
  }

  // Return the processed data
  Ok(Value::Array(processed_data))
}

// check holidays
#[command]
async fn check_holidays(month: String, year: String) -> Result<Value, String> {
  // Call the existing function to get the Hijri calendar data
  let calendar_data = get_hijri_calendar_by_month(month, year).await?;

  // Check if the calendar data is an array
  let data_array = calendar_data.as_array().ok_or("Unexpected data format at checking holidays.")?;

  // Initialize an array to store days with holidays
  let mut holidays = Vec::new();

  // Iterate over the data and check for holidays
  for entry in data_array {
    if let Some(day) = entry.get("day") {
      if let Some(holidays_list) = entry.get("holidays").and_then(|h| h.as_array()) {
        if !holidays_list.is_empty() {
          // If there are holidays, add the day and holidays to the array
          holidays.push(json!({
            "date": entry.get("date").cloned().unwrap_or(Value::Null),
            "gregorian_date": entry.get("gregorian_date").cloned().unwrap_or(Value::Null),
            "day": day,
            "month": entry.get("month").and_then(|m| m.as_str()),
            "year": entry.get("year").and_then(|y| y.as_str()),
            "holidays": holidays_list
          }));
        }
      }
    }
  }

  // Return the array of days with holidays
  Ok(Value::Array(holidays))
}

// get holiday days array
#[command]
async fn get_holiday_days(month: String, year: String) -> Result<Vec<u32>, String> {
  // Call the existing function to get the holidays
  let holidays_data = check_holidays(month, year).await?;

  // Check if the holidays data is an array
  let data_array = holidays_data.as_array().ok_or("Unexpected data format at getting holiday days.")?;

  // Initialize an array to store days with holidays
  let mut holiday_days = Vec::new();

  // Iterate over the data and extract the days
  for entry in data_array {
    // Construct the Hijri date string
    if let Some(date) = entry.get("date").and_then(|d| d.as_str()) {
      // Call the API to convert Hijri date to Gregorian date
      let url = format!("https://api.aladhan.com/v1/hToG/{}", date);
      let response = reqwest::get(&url).await.map_err(|e| format!("Unexpected error at converting Hijri date to Gregorian date for holidays: {}", e.to_string()))?;
      let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing Hijri date to Gregorian date for holidays: {}", e.to_string()))?;

      // Extract the Gregorian date from the response
      if let Some(gregorian_date) = req["data"]["gregorian"]["day"].as_str() {
        // Parse the Gregorian day as a u32 and add it to the array
        if let Ok(day_num) = gregorian_date.parse::<u32>() {
          holiday_days.push(day_num);
        }
      }
    }
  }

  // Return the array of Gregorian dates with holidays
  Ok(holiday_days)
}

// get hijri date
#[command]
async fn get_today_hijri_date() -> Result<Value, String> {
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).unwrap();
  let date = datetime.format("%d-%m-%Y").to_string();
  
  let url = format!("http://api.aladhan.com/v1/gToH/{}", date);
  let response = reqwest::get(&url).await.unwrap();
  let req: Value = response.json().await.unwrap();
  
  if let Some(hijri) = req["data"]["hijri"].as_object() {
    let mut processed_entry = json!({
      "day": hijri.get("day").cloned().unwrap_or(Value::Null),
      "month": hijri.get("month").and_then(|m| m.get("en")).cloned().unwrap_or(Value::Null),
      "year": hijri.get("year").cloned().unwrap_or(Value::Null),
    });

    // Check if the entry contains any holidays
    if let Some(holidays) = hijri.get("holidays").and_then(|h| h.as_array()) {
      if !holidays.is_empty() {
        processed_entry["holiday"] = json!(holidays[0]);
      }
    }

    return Ok(processed_entry);
  }

  Err("Unexpected data format at getting today's Hijri date.".to_string())
}

// get this day data
#[command]
async fn get_this_day_data() -> Result<Value, String> {
  // Get the location (latitude and longitude)
  let location = get_location().await.map_err(|e| format!("Unexpected error at parsing location for this day data: {}", e.to_string()))?;
  let lat = location.latitude;
  let lon = location.longitude;

  // Get the current year and month
  let timestamp = get_local_time();
  let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(timestamp.as_str()).map_err(|e| format!("Unexpected error at parsing date for this day data: {}", e.to_string()))?;
  let date = datetime.format("%d-%m-%Y").to_string();

  // Construct the URL for the API request
  let url = format!("https://api.aladhan.com/v1/timings/{}?latitude={}&longitude={}", date, lat, lon);
  
  // Fetch the response from the API
  let response = reqwest::get(&url).await.map_err(|e| format!("Unexpected error at fetching this day data: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing this day data: {}", e.to_string()))?;

  Ok(req)
}

// get this month prayer times
#[command]
async fn get_prayer_times_this_month() -> Result<Value, String> {
  // Call the async function and await its result
  let req = get_this_month_data().await?;

  // Check if the response contains data
  let data = req["data"].as_array().ok_or("No data found in API response for this month at prayer times this month.")?;

  // Initialize an array to hold all prayer times for the month
  let mut prayer_times_array = Vec::new();

  // Iterate over each day in the response and extract the relevant prayer times
  for day in data {
    if let Some(timings) = day["timings"].as_object() {
      let selected_timings = json!({
        "Fajr": timings.get("Fajr").cloned().unwrap_or(Value::Null),
        "Dhuhr": timings.get("Dhuhr").cloned().unwrap_or(Value::Null),
        "Asr": timings.get("Asr").cloned().unwrap_or(Value::Null),
        "Maghrib": timings.get("Maghrib").cloned().unwrap_or(Value::Null),
        "Isha": timings.get("Isha").cloned().unwrap_or(Value::Null),
      });
      prayer_times_array.push(selected_timings);
    }
  }

  // Return the array of prayer times as a JSON value
  Ok(Value::Array(prayer_times_array))
}

// get this day prayer times
#[command]
async fn get_prayer_times_this_day() -> Result<Value, String> {
  let req = get_this_day_data().await?;

  // Extract the prayer times for the day
  if let Some(timings) = req["data"]["timings"].as_object() {
    let selected_timings = json!({
      "Fajr": timings.get("Fajr"),
      "Dhuhr": timings.get("Dhuhr"),
      "Asr": timings.get("Asr"),
      "Maghrib": timings.get("Maghrib"),
      "Isha": timings.get("Isha"),
    });

    // Return the prayer times as a JSON object
    return Ok(selected_timings);
  }

  Err("No data found in API response for this day at prayer times this day.".to_string())
}

// get nearest prayer
#[command]
async fn get_nearest_prayer() -> Result<String, String> {
  // Get the prayer times for the day
  let prayer_times = get_prayer_times_this_day().await?;

  // Get the current time
  let current_time = local_clock()?;
  let current_time = NaiveTime::parse_from_str(&current_time, "%H:%M:%S")
    .map_err(|e| format!("Unexpected error at parsing current time for nearest prayer: {}", e.to_string()))?;

  // List of prayer names in correct order
  let prayer_names = ["Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"];

  // Initialize nearest prayer and time difference
  let mut nearest_prayer = "";
  let mut time_diff = Duration::hours(24);
  let tolerance = Duration::minutes(35);

  // Iterate over each prayer and calculate time difference
  for prayer in prayer_names.iter() {
    if let Some(time_str) = prayer_times[prayer].as_str() {
      // Parse time string
      let prayer_time = NaiveTime::parse_from_str(time_str, "%H:%M")
        .map_err(|e| format!("Unexpected error at parsing time for nearest prayer: {}", e.to_string()))?;

      // Calculate time difference
      let diff = if prayer_time > current_time {
        prayer_time - current_time
      } else {
        let passed_time = current_time - prayer_time;
        if passed_time <= tolerance {
          // If prayer time has just passed within tolerance, consider it as nearest
          passed_time
        } else {
          // Add 24 hours if the prayer time is before current time and outside tolerance
          Duration::hours(24) - passed_time
        }
      };

      // Update nearest prayer and time difference
      if diff < time_diff {
        time_diff = diff;
        nearest_prayer = prayer;
      }
    }
  }

  Ok(nearest_prayer.to_string())
}

// get time until next prayer
#[command]
async fn get_time_until_next_prayer() -> Result<String, String> {
  // Get the prayer times for the day
  let prayer_times = get_prayer_times_this_day().await?;

  // Get the current time
  let current_time = local_clock()?;
  let current_time = NaiveTime::parse_from_str(&current_time, "%H:%M:%S")
    .map_err(|e| format!("Unexpected error at parsing current time for time until next prayer: {}", e.to_string()))?;

  // List of prayer names in correct order
  let prayer_names = ["Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"];

  // Initialize nearest prayer and time difference
  let mut time_diff = Duration::hours(24);
  let tolerance = Duration::minutes(35);

  // Iterate over each prayer and calculate time difference
  for prayer in prayer_names.iter() {
    if let Some(time_str) = prayer_times[prayer].as_str() {
      // Parse time string
      let prayer_time = NaiveTime::parse_from_str(time_str, "%H:%M")
        .map_err(|e| format!("Unexpected error at parsing time for time until next prayer: {}", e.to_string()))?;

      // Calculate time difference
      let diff = if prayer_time > current_time {
        prayer_time - current_time
      } else {
        let passed_time = current_time - prayer_time;
        if passed_time <= tolerance {
          // If prayer time has just passed within tolerance, consider it as current
          Duration::zero()
        } else {
          // Add 24 hours if the prayer time is before current time and outside tolerance
          Duration::hours(24) - passed_time
        }
      };

      // Update nearest prayer and time difference
      if diff < time_diff {
        time_diff = diff;
      }
    }
  }

  // Check if we're within 20 minutes after a prayer time
  if time_diff == Duration::zero() {
    Ok("Now".to_string())
  } else {
    // Convert the duration to hours, minutes, and seconds
    let hours = time_diff.num_hours();
    let minutes = (time_diff - Duration::hours(hours)).num_minutes();
    let seconds = (time_diff - Duration::hours(hours) - Duration::minutes(minutes)).num_seconds();
    
    // Format the result
    let result = format!("-{}:{:02}:{:02}", hours, minutes, seconds);
    Ok(result)
  }
}

// get random verse
#[command]
async fn get_random_verse() -> Result<TodayVerse, String> {
  let random_number = generate_random_number().to_string();
  let url = format!("https://api.alquran.cloud/v1/ayah/{}/en.asad", random_number);
  let response = reqwest::get(&url).await.map_err(|e| format!("Unexpected error at fetching random verse: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing random verse: {}", e.to_string()))?;

  let data = &req["data"];
  let verse = TodayVerse {
    surah_name: data["surah"]["englishName"].as_str().unwrap_or("").to_string(),
    surah_name_translation: data["surah"]["englishNameTranslation"].as_str().unwrap_or("").to_string(),
    surah_number: data["surah"]["number"].to_string(),
    verse_number: data["numberInSurah"].to_string(),
    verse_text: data["text"].as_str().unwrap_or("").to_string(),
  };

  Ok(verse)
}

// get Quran data
#[command]
async fn get_quran_data() -> Result<QuranData, String> {
  let url = "https://api.alquran.cloud/v1/quran/ar.alafasy";
  let response = reqwest::get(url).await.map_err(|e| format!("Unexpected error at fetching Quran data: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing Quran data: {}", e.to_string()))?;

  // Check if the response contains data
  let data = req["data"]["surahs"].as_array().ok_or("No data found in API response for Quran data.")?;

  let mut surah_array = Vec::new();

  for surah in data {
    let surah_info = Surah {
      name: surah["name"].as_str().unwrap_or("").to_string(),
      english_name: surah["englishName"].as_str().unwrap_or("").to_string(),
      english_name_translation: surah["englishNameTranslation"].as_str().unwrap_or("").to_string(),
      ayahs: surah["ayahs"].as_array().unwrap_or(&Vec::new()).iter().map(|a| Ayah { text: a["text"].as_str().unwrap_or("").to_string(), audio: a["audioSecondary"][0].as_str().unwrap_or("").to_string() }).collect(),
    };
    surah_array.push(surah_info);
  }

  Ok(QuranData { surahs: surah_array })
}

// get surah translation
#[command]
async fn get_surah_translation(id: String) -> Result<Vec<AyahTranslation>, String> {
  let url = format!("https://api.alquran.cloud/v1/surah/{}/en.asad", id);
  let response = reqwest::get(&url).await.map_err(|e| format!("Unexpected error at fetching surah translation: {}", e.to_string()))?;
  let req: Value = response.json().await.map_err(|e| format!("Unexpected error at parsing surah translation: {}", e.to_string()))?;

  // Check if the response contains the array of ayahs
  let data = req["data"]["ayahs"].as_array().ok_or("No ayahs found in API response for surah translation.")?;

  let mut ayah_array = Vec::new();

  for ayah in data {
    let ayah_info = AyahTranslation { text: ayah["text"].as_str().unwrap_or("").to_string() };
    ayah_array.push(ayah_info);
  }

  Ok(ayah_array)
}

// Play audio
#[command]
fn play_audio(url: String, vol: f32, state: State<'_, Arc<AppState>>, window: Window) {
  let state = state.inner().clone();

  thread::spawn(move || {
    // Fetch the audio data from the URL
    let audio_data = match reqwest::blocking::get(&url) {
      Ok(response) => match response.bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
          eprintln!("Error reading audio data: {}", e);
          return;
        }
      },
      Err(e) => {
        eprintln!("Error fetching audio from URL: {}", e);
        return;
      }
    };

    let cursor = Cursor::new(audio_data);

    let (_stream, stream_handle) = match OutputStream::try_default() {
      Ok(output) => output,
      Err(e) => {
        eprintln!("Error initializing audio output: {}", e);
        return;
      }
    };

    let sink = match Sink::try_new(&stream_handle) {
      Ok(sink) => Arc::new(sink),
      Err(e) => {
        eprintln!("Error creating audio sink: {}", e);
        return;
      }
    };

    let source = match Decoder::new(cursor) {
      Ok(source) => source,
      Err(e) => {
        eprintln!("Error decoding audio data: {}", e);
        return;
      }
    };

    let source_duration = source.total_duration().unwrap_or(StdDuration::from_secs(0));

    sink.append(source);

    {
      let mut current_ayah = state.current_ayah.lock().unwrap();
      if let Some(ref current) = *current_ayah {
        current.pause();
      }

      *current_ayah = Some(sink.clone());
    }

    {
      let mut is_playing = state.is_playing.lock().unwrap();
      *is_playing = true;
    }

    sink.set_volume(vol);

    // Start a timer to track elapsed time
    let start_time = Instant::now();

    // Thread to send progress updates
    let sink_clone = sink.clone();
    let window_clone = window.clone();
    let state_clone = state.clone();
    thread::spawn(move || {
      while !sink_clone.empty() {
        let is_playing = *state_clone.is_playing.lock().unwrap();

        if !is_playing {
          break;
        }

        let elapsed = start_time.elapsed();
        let progress = elapsed.as_secs_f64() / source_duration.as_secs_f64();
        window_clone.emit("audio-progress", progress).unwrap();
        thread::sleep(StdDuration::from_millis(10));
      }
    });

    // Notify the frontend when the audio finishes
    sink.sleep_until_end();
    window.emit("audio-finished", true).unwrap();

    {
      let mut is_playing = state.is_playing.lock().unwrap();
      *is_playing = false;
    }
  });
}

// Pause audio
#[command]
fn pause_audio(state: State<'_, Arc<AppState>>) {
    let current_ayah = state.current_ayah.lock().unwrap();
    if let Some(ref sink) = *current_ayah {
      sink.pause();
    }
    let mut is_playing = state.is_playing.lock().unwrap();
    *is_playing = false;
}

// Change volume
#[command]
fn set_volume(vol: f32, state: State<'_, Arc<AppState>>) {
  let current_ayah = state.current_ayah.lock().unwrap();
  if let Some(ref sink) = *current_ayah {
    sink.set_volume(vol);
  }
}

fn main() {
  tauri::Builder::default()
  .manage(
    Arc::new(AppState {
      current_ayah: Mutex::new(None),
      is_playing: Mutex::new(false),
    })
  )
  .invoke_handler(tauri::generate_handler![
    get_local_time,
    get_hijri_calendar,
    get_today_hijri_date,
    get_hijri_calendar_by_month,
    check_holidays,
    get_holiday_days,
    get_prayer_times_this_month,
    get_prayer_times_this_day,
    get_nearest_prayer,
    get_time_until_next_prayer,
    local_date,
    formatted_date,
    local_clock, 
    get_location,
    get_random_verse,
    add_prayer,
    get_prayer_data_by_date,
    get_prayer_data_by_month,
    get_prayer_data_by_range,
    get_prayer_heatmap_by_month,
    get_prayer_heatmap_by_range,
    get_quran_data,
    get_surah_translation,
    play_audio,
    pause_audio,
    set_volume
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}