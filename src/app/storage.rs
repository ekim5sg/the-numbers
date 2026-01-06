use serde::{Deserialize, Serialize};
use web_sys::window;

use super::grade::Grade;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Progress {
    pub completed: Vec<usize>,
}

const KEY_PROGRESS: &str = "the_numbers_progress_v1";
const KEY_GRADE: &str = "the_numbers_grade_v1";

pub fn load_progress() -> Progress {
    let Some(win) = window() else { return Progress::default(); };
    let Ok(Some(storage)) = win.local_storage() else { return Progress::default(); };
    let Ok(Some(raw)) = storage.get_item(KEY_PROGRESS) else { return Progress::default(); };
    serde_json::from_str(&raw).unwrap_or_default()
}

pub fn save_progress(p: &Progress) {
    let Some(win) = window() else { return; };
    let Ok(Some(storage)) = win.local_storage() else { return; };
    if let Ok(raw) = serde_json::to_string(p) {
        let _ = storage.set_item(KEY_PROGRESS, &raw);
    }
}

pub fn load_grade() -> Grade {
    let Some(win) = window() else { return Grade::G3; };
    let Ok(Some(storage)) = win.local_storage() else { return Grade::G3; };
    let Ok(Some(raw)) = storage.get_item(KEY_GRADE) else { return Grade::G3; };

    let v: u8 = raw.parse().unwrap_or(3);
    Grade::from_u8(v)
}

pub fn save_grade(g: Grade) {
    let Some(win) = window() else { return; };
    let Ok(Some(storage)) = win.local_storage() else { return; };
    let _ = storage.set_item(KEY_GRADE, &g.as_u8().to_string());
}