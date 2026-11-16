//! Read-only system presentation preferences. No calendar event access.
use objc2_foundation::{NSCalendar, NSLocale};

#[derive(serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CalendarPreferences {
    locale: String,
    first_weekday: u8,
}

pub fn read() -> CalendarPreferences {
    let locale = NSLocale::autoupdatingCurrentLocale();
    let mut tags = vec![locale.languageCode().to_string()];
    if let Some(script) = locale.scriptCode() {
        tags.push(script.to_string());
    }
    if let Some(region) = locale.regionCode() {
        tags.push(region.to_string());
    }
    CalendarPreferences {
        locale: tags.join("-"),
        // Foundation: Sunday=1; JS Date: Sunday=0. Includes user override.
        first_weekday: (NSCalendar::autoupdatingCurrentCalendar().firstWeekday() - 1) as u8,
    }
}
