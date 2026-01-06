// src/app/stats.rs

use serde::{Deserialize, Serialize};

use crate::app::Skill;

const KEY: &str = "the_numbers_stats_week_v1";
const MAX_DAYS_TO_KEEP: usize = 45; // plenty for "weekly" without growing forever

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SkillCount {
    pub attempts: u32,
    pub correct: u32,
}

impl SkillCount {
    pub fn record(&mut self, ok: bool) {
        self.attempts += 1;
        if ok {
            self.correct += 1;
        }
    }

    pub fn accuracy(&self) -> Option<f32> {
        if self.attempts == 0 {
            None
        } else {
            Some(self.correct as f32 / self.attempts as f32)
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DayCounts {
    pub addition: SkillCount,
    pub subtraction: SkillCount,
    pub multiplication: SkillCount,
    pub division: SkillCount,
    pub word: SkillCount,
    pub mixed: SkillCount,
}

impl DayCounts {
    pub fn skill_mut(&mut self, s: Skill) -> &mut SkillCount {
        match s {
            Skill::Addition => &mut self.addition,
            Skill::Subtraction => &mut self.subtraction,
            Skill::Multiplication => &mut self.multiplication,
            Skill::Division => &mut self.division,
            Skill::WordProblem => &mut self.word,
            Skill::Unknown => &mut self.mixed,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WeeklyStats {
    /// Map-like storage: Vec of (YYYY-MM-DD, counts). We keep it Vec for stable serde + easy prune.
    pub days: Vec<(String, DayCounts)>,
}

fn today_ymd_local() -> String {
    let d = js_sys::Date::new_0();
    let yyyy = d.get_full_year() as i32;
    let mm = (d.get_month() + 1) as i32;
    let dd = d.get_date() as i32;
    format!("{:04}-{:02}-{:02}", yyyy, mm, dd)
}

fn load_raw() -> Option<String> {
    let win = web_sys::window()?;
    let storage = win.local_storage().ok()??;
    storage.get_item(KEY).ok()?
}

fn save_raw(s: &str) {
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item(KEY, s);
        }
    }
}

pub fn load_stats() -> WeeklyStats {
    let Some(raw) = load_raw() else {
        return WeeklyStats::default();
    };
    serde_json::from_str::<WeeklyStats>(&raw).unwrap_or_default()
}

pub fn save_stats(st: &WeeklyStats) {
    if let Ok(raw) = serde_json::to_string(st) {
        save_raw(&raw);
    }
}

fn prune_old_days(mut st: WeeklyStats) -> WeeklyStats {
    // Keep newest MAX_DAYS_TO_KEEP by sorting by date string (YYYY-MM-DD sorts lexicographically).
    st.days.sort_by(|a, b| a.0.cmp(&b.0));
    if st.days.len() > MAX_DAYS_TO_KEEP {
        let start = st.days.len() - MAX_DAYS_TO_KEEP;
        st.days = st.days[start..].to_vec();
    }
    st
}

// ✅ FIXED: avoids holding an iter_mut borrow across push/len.
fn get_or_create_day<'a>(st: &'a mut WeeklyStats, ymd: &str) -> &'a mut DayCounts {
    if let Some(idx) = st.days.iter().position(|(d, _)| d == ymd) {
        return &mut st.days[idx].1;
    }

    st.days.push((ymd.to_string(), DayCounts::default()));
    let last = st.days.len() - 1;
    &mut st.days[last].1
}

pub fn record_attempt(st: &mut WeeklyStats, skill: Skill, ok: bool) {
    let ymd = today_ymd_local();
    let day = get_or_create_day(st, &ymd);
    day.skill_mut(skill).record(ok);
    *st = prune_old_days(std::mem::take(st));
}

/// Return the last N days (including today if present), sorted newest->oldest.
pub fn last_n_days(st: &WeeklyStats, n: usize) -> Vec<(String, DayCounts)> {
    let mut v = st.days.clone();
    v.sort_by(|a, b| b.0.cmp(&a.0)); // newest first
    v.into_iter().take(n).collect()
}

/// Sum a slice of DayCounts into a single DayCounts (acts like a “week totals” object).
pub fn sum_days(days: &[(String, DayCounts)]) -> DayCounts {
    let mut out = DayCounts::default();

    for (_, d) in days.iter() {
        out.addition.attempts += d.addition.attempts;
        out.addition.correct += d.addition.correct;

        out.subtraction.attempts += d.subtraction.attempts;
        out.subtraction.correct += d.subtraction.correct;

        out.multiplication.attempts += d.multiplication.attempts;
        out.multiplication.correct += d.multiplication.correct;

        out.division.attempts += d.division.attempts;
        out.division.correct += d.division.correct;

        out.word.attempts += d.word.attempts;
        out.word.correct += d.word.correct;

        out.mixed.attempts += d.mixed.attempts;
        out.mixed.correct += d.mixed.correct;
    }

    out
}