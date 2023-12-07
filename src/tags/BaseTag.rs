
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
struct BaseTag {
    id: u32,
    created_date: DateTime<Utc>,
    name: String
}

impl BaseTag {

    pub fn new() -> Self{
        Self{
            id: 0,
            created_date: chrono::offset::Utc::now(),
            name: "".to_string(),
        }
    }
}