use serde::{Serialize, Deserialize};

// Location structure
#[derive(Serialize, Deserialize)]
pub struct Location {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
    pub location: String,
}

// Today Verse structure
#[derive(Serialize, Deserialize)]
pub struct TodayVerse {
    pub surah_name: String,
    pub surah_name_translation: String,
    pub surah_number: String,
    pub verse_number: String,
    pub verse_text: String,
}

// Ayah structure
#[derive(Serialize, Deserialize)]
pub struct Ayah {
    pub text: String,
    pub audio: String,
}

// Ayah translation structure
#[derive(Serialize, Deserialize)]
pub struct AyahTranslation {
    pub text: String,
}

// Surah structure
#[derive(Serialize, Deserialize)]
pub struct Surah {
    pub name: String,
    pub english_name: String,
    pub english_name_translation: String,
    pub ayahs: Vec<Ayah>,
}

// Quran data structure
#[derive(Serialize, Deserialize)]
pub struct QuranData {
    pub surahs: Vec<Surah>,
}
