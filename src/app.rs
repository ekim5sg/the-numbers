// E:\rust_dev\the-numbers\src\app.rs

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;

mod data;
pub mod grade;
mod questions;
mod storage;
mod ai_day;
mod stats;

use data::{Day, DAYS};
use grade::Grade;
use questions::{
    questions_for as local_questions_for, Difficulty as LocalDifficulty, Question as LocalQuestion,
};
use storage::{load_grade, load_progress, save_grade, save_progress, Progress};
use ai_day::{DayResponse, Difficulty as AiDifficulty};
use stats::{load_stats, record_attempt, save_stats, last_n_days, sum_days};

use rand::seq::SliceRandom;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::spawn_local;

// ----------------------------
// CONFIG: set to your deployed Worker URL
// Local dev: http://127.0.0.1:8787
// Deployed:  https://the-numbers-worker.mikegyver.workers.dev
// ----------------------------
const WORKER_BASE_URL: &str = "https://the-numbers-worker.mikegyver.workers.dev";

// ----------------------------
// Routes
// ----------------------------

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/day/:id")]
    Day { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn day_tag(day_id: usize) -> (&'static str, &'static str) {
    match day_id {
        1 => ("🔍", "Detective Day"),
        2 => ("🧩", "Puzzle Day"),
        3 => ("🧱", "Builder Day"),
        4 => ("🧭", "Turn Day"),
        5 => ("📊", "Data Day"),
        6 => ("🧊", "Grid Day"),
        7 => ("🎲", "Chance Day"),
        8 => ("🕵️", "Secret Numbers"),
        9 => ("🧠", "Smart Counting"),
        10 => ("⏱️", "Change Day"),
        11 => ("✅", "Truth Day"),
        12 => ("♟️", "Strategy Day"),
        _ => ("✨", "Math Day"),
    }
}

// ----------------------------
// Subtle audio tones (robust: avoids web-sys feature gating issues)
// ----------------------------

fn is_sound_enabled() -> bool {
    let Some(win) = web_sys::window() else { return true; };
    let Ok(Some(storage)) = win.local_storage() else { return true; };
    let Ok(Some(v)) = storage.get_item("the_numbers_sound_v1") else { return true; };
    v == "1"
}

// Set AudioParam value via JS reflection so we don't depend on web-sys generated methods.
fn set_audio_param(node: &wasm_bindgen::JsValue, field: &str, value: f32) {
    use js_sys::{Function, Reflect};

    let Ok(param) = Reflect::get(node, &wasm_bindgen::JsValue::from_str(field)) else { return; };

    // Prefer setValueAtTime(value, currentTime) if present; otherwise try setting ".value".
    let maybe_fn = Reflect::get(&param, &wasm_bindgen::JsValue::from_str("setValueAtTime")).ok();
    if let Some(f) = maybe_fn {
        if f.is_function() {
            let func: Function = f.unchecked_into();
            // time is not critical for our tiny tones; pass 0
            let _ = func.call2(
                &param,
                &wasm_bindgen::JsValue::from_f64(value as f64),
                &wasm_bindgen::JsValue::from_f64(0.0),
            );
            return;
        }
    }

    let _ = Reflect::set(
        &param,
        &wasm_bindgen::JsValue::from_str("value"),
        &wasm_bindgen::JsValue::from_f64(value as f64),
    );
}

fn play_tone(freq: f32, ms: i32, gain: f32) {
    let Ok(ctx) = web_sys::AudioContext::new() else { return; };

    let Ok(osc) = ctx.create_oscillator() else { return; };
    let Ok(g) = ctx.create_gain() else { return; };

    // Set frequency + gain without relying on generated accessors
    set_audio_param(&osc.clone().into(), "frequency", freq);
    set_audio_param(&g.clone().into(), "gain", gain);

    let _ = osc.connect_with_audio_node(&g);
    let _ = g.connect_with_audio_node(&ctx.destination());

    let _ = osc.start();
    let _ = osc.stop_with_when(ctx.current_time() + (ms as f64 / 1000.0));

    // Close after tone completes (best effort)
    let cb = Closure::<dyn FnMut()>::new(move || {
        let _ = ctx.close();
    });

    if let Some(win) = web_sys::window() {
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            ms + 80,
        );
    }
    cb.forget();
}

// ----------------------------
// Celebration banner
// ----------------------------

fn celebration_banner() -> Html {
    html! {
        <div style="
            margin: 14px 0 18px;
            padding: 14px 16px;
            border-radius: 16px;
            border: 1px solid #ddd;
            background: linear-gradient(90deg,
              rgba(255,215,0,.16),
              rgba(135,206,250,.16),
              rgba(255,182,193,.16),
              rgba(255,215,0,.16)
            );
            background-size: 200% 200%;
            animation: shimmer 3s ease infinite;
            box-shadow: 0 2px 10px rgba(0,0,0,.06);
            position: relative;
            overflow: hidden;
        ">
            <style>
                {r#"
                @keyframes sparkleFloat {
                    0%   { transform: translateY(6px); opacity: .25; }
                    50%  { transform: translateY(-4px); opacity: .65; }
                    100% { transform: translateY(6px); opacity: .25; }
                }
                .sparkle { display:inline-block; animation: sparkleFloat 1.8s ease-in-out infinite; }
                .sparkle:nth-child(2) { animation-delay: .2s; }
                .sparkle:nth-child(3) { animation-delay: .4s; }
                .sparkle:nth-child(4) { animation-delay: .6s; }

                @keyframes shimmer {
                    0% { background-position: 0% 50%; }
                    100% { background-position: 100% 50%; }
                }
                "#}
            </style>

            <div style="display:flex; align-items:center; justify-content:space-between; gap:12px; flex-wrap:wrap;">
                <div>
                    <div style="font-size: 20px; font-weight: 800;">
                        {"🎉 You did it! All 12 days complete!"}
                    </div>
                    <div style="margin-top:6px; opacity:.85;">
                        {"You leveled up your math brain. Take a bow… then teach someone one cool thing you learned!"}
                    </div>
                </div>

                <div style="font-size: 26px; white-space: nowrap;">
                    <span class="sparkle">{"✨"}</span>
                    <span class="sparkle">{"🎊"}</span>
                    <span class="sparkle">{"✨"}</span>
                    <span class="sparkle">{"🏆"}</span>
                </div>
            </div>
        </div>
    }
}

// ----------------------------
// Date + localStorage helpers
// ----------------------------

fn today_ymd_local() -> String {
    let d = js_sys::Date::new_0();
    let yyyy = d.get_full_year() as i32;
    let mm = (d.get_month() + 1) as i32;
    let dd = d.get_date() as i32;
    format!("{:04}-{:02}-{:02}", yyyy, mm, dd)
}

fn day_cache_key(grade: Grade, day_id: usize) -> String {
    format!("the_numbers_day_v1_{}_{}", grade.as_u8(), day_id)
}

fn load_day_from_local_storage(grade: Grade, day_id: usize) -> Option<DayResponse> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let raw = storage.get_item(&day_cache_key(grade, day_id)).ok()??;
    serde_json::from_str::<DayResponse>(&raw).ok()
}

fn save_day_to_local_storage(grade: Grade, day_id: usize, day: &DayResponse) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(raw) = serde_json::to_string(day) {
                let _ = storage.set_item(&day_cache_key(grade, day_id), &raw);
            }
        }
    }
}

// ----------------------------
// AI fetch: per-day endpoint
// ----------------------------

async fn fetch_day(
    worker_base_url: &str,
    grade: Grade,
    day_id: usize,
) -> Result<DayResponse, String> {
    let url = format!(
        "{}/api/day-questions?grade={}&day={}",
        worker_base_url.trim_end_matches('/'),
        grade.as_u8(),
        day_id
    );

    gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("fetch error: {e:?}"))?
        .json::<DayResponse>()
        .await
        .map_err(|e| format!("json error: {e:?}"))
}

// ----------------------------
// Skills classifier (Parent/Educator + Weekly Summary)
// ----------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Skill {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    WordProblem,
    Unknown,
}

fn classify_skill(prompt: &str) -> Skill {
    let p = prompt.to_lowercase();

    // word-problem signals first
    if p.contains("how many")
        || p.contains("left")
        || p.contains("total")
        || p.contains("in all")
        || p.contains("shared")
        || p.contains("altogether")
    {
        return Skill::WordProblem;
    }

    if p.contains('+') || p.contains(" plus ") {
        return Skill::Addition;
    }
    if p.contains('-') || p.contains(" minus ") {
        return Skill::Subtraction;
    }
    if p.contains('×') || p.contains(" x ") || p.contains(" times ") {
        return Skill::Multiplication;
    }
    if p.contains('÷') || p.contains(" per ") || p.contains(" each ") {
        return Skill::Division;
    }

    Skill::Unknown
}

fn skill_label(s: Skill) -> &'static str {
    match s {
        Skill::Addition => "Addition",
        Skill::Subtraction => "Subtraction",
        Skill::Multiplication => "Multiplication",
        Skill::Division => "Division",
        Skill::WordProblem => "Word Problems",
        Skill::Unknown => "Mixed Skills",
    }
}

// ----------------------------
// Clipboard helpers (NO web-sys feature gating)
// ----------------------------

fn copy_text_to_clipboard(text: &str) -> bool {
    use js_sys::{Function, Reflect};

    let Some(win) = web_sys::window() else { return false; };
    let Some(doc) = win.document() else { return false; };

    // Try: window.navigator.clipboard.writeText(text)
    let nav = Reflect::get(&win, &wasm_bindgen::JsValue::from_str("navigator")).ok();
    if let Some(nav) = nav {
        let clip = Reflect::get(&nav, &wasm_bindgen::JsValue::from_str("clipboard")).ok();
        if let Some(clip) = clip {
            let wt = Reflect::get(&clip, &wasm_bindgen::JsValue::from_str("writeText")).ok();
            if let Some(wt) = wt {
                if wt.is_function() {
                    let f: Function = wt.unchecked_into();
                    let _ = f.call1(&clip, &wasm_bindgen::JsValue::from_str(text));
                    return true; // optimistic: most browsers resolve promise
                }
            }
        }
    }

    // Fallback: textarea + document.execCommand("copy")
    let Ok(el) = doc.create_element("textarea") else { return false; };

    // Set textarea.value via reflection (avoid HtmlTextAreaElement feature gating)
    let _ = Reflect::set(
        &el,
        &wasm_bindgen::JsValue::from_str("value"),
        &wasm_bindgen::JsValue::from_str(text),
    );

    // Make it invisible / offscreen using style attribute (no .style())
    let _ = el.set_attribute(
        "style",
        "position:fixed;left:-9999px;top:0;opacity:0;pointer-events:none;",
    );
    let _ = el.set_attribute("readonly", "true");

    let Some(body) = doc.body() else { return false; };
    let _ = body.append_child(&el);

    // Focus + select via reflection
    let focus = Reflect::get(&el, &wasm_bindgen::JsValue::from_str("focus")).ok();
    if let Some(focus) = focus {
        if focus.is_function() {
            let f: Function = focus.unchecked_into();
            let _ = f.call0(&el);
        }
    }
    let select = Reflect::get(&el, &wasm_bindgen::JsValue::from_str("select")).ok();
    if let Some(select) = select {
        if select.is_function() {
            let f: Function = select.unchecked_into();
            let _ = f.call0(&el);
        }
    }

    // document.execCommand("copy") via reflection (no exec_command())
    let exec = Reflect::get(&doc, &wasm_bindgen::JsValue::from_str("execCommand")).ok();
    let ok = if let Some(exec) = exec {
        if exec.is_function() {
            let f: Function = exec.unchecked_into();
            f.call1(&doc, &wasm_bindgen::JsValue::from_str("copy"))
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        } else {
            false
        }
    } else {
        false
    };

    let _ = body.remove_child(&el);
    ok
}

// ----------------------------
// App Root
// Use HashRouter for IIS/static hosting (no server rewrites needed).
// ----------------------------

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <div style="
                font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
                max-width: 980px; margin: 0 auto; padding: 20px;
            ">
                <Switch<Route> render={switch} />
            </div>
        </HashRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Day { id } => html! { <DayView id={id} /> },
        Route::NotFound => html! { <h2>{"Not found"}</h2> },
    }
}

// ----------------------------
// Sound flag writer (Home toggle -> PracticeWidget reads via localStorage)
// ----------------------------

#[derive(Properties, PartialEq)]
struct SoundFlagProps {
    enabled: bool,
}

#[function_component(SoundFlag)]
fn sound_flag(props: &SoundFlagProps) -> Html {
    use_effect_with(props.enabled, move |enabled| {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.set_item("the_numbers_sound_v1", if *enabled { "1" } else { "0" });
            }
        }
        || ()
    });

    html! {}
}

// ----------------------------
// Parent Summary (weekly breakdown) ✅ ACTIONABLE + COPY REPORT
// (MUST be a component to use hooks)
// ----------------------------

fn pct_label_from(sc: &stats::SkillCount) -> String {
    match sc.accuracy() {
        Some(a) => format!("{}%", (a * 100.0).round() as i32),
        None => "—".to_string(),
    }
}

fn total_attempts_for_week(t: &stats::DayCounts) -> u32 {
    t.addition.attempts
        + t.subtraction.attempts
        + t.multiplication.attempts
        + t.division.attempts
        + t.word.attempts
        + t.mixed.attempts
}

#[function_component(ParentSummaryPanel)]
fn parent_summary_panel() -> Html {
    let weekly = load_stats();
    let last7 = last_n_days(&weekly, 7);
    let totals = sum_days(&last7);

    let week_attempts = total_attempts_for_week(&totals);

    let copied_toast = use_state(|| false);

    // auto-hide copied toast
    {
        let copied_toast = copied_toast.clone();
        use_effect_with(*copied_toast, move |on| {
            if *on {
                let copied_toast2 = copied_toast.clone();
                let cb = Closure::<dyn FnMut()>::new(move || {
                    copied_toast2.set(false);
                });
                if let Some(win) = web_sys::window() {
                    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        1800,
                    );
                }
                cb.forget();
            }
            || ()
        });
    }

    if week_attempts == 0 {
        return html! {
            <div style="margin: 0 0 16px; padding: 12px 14px; border:1px solid #ddd; border-radius: 14px;">
                <div style="font-weight: 800;">{"👨‍👩‍👧 Parent Summary (Last 7 days)"}</div>
                <div style="margin-top: 8px; opacity:.85;">
                    {"No practice attempts recorded yet this week. Once the student checks answers, this fills in automatically."}
                </div>
            </div>
        };
    }

    let pct_label = |attempts: u32, correct: u32| -> String {
        if attempts == 0 {
            "—".to_string()
        } else {
            let pct = ((correct as f32) / (attempts as f32) * 100.0).round() as i32;
            format!("{}%", pct)
        }
    };

    let total_correct = |t: &stats::DayCounts| -> u32 {
        t.addition.correct
            + t.subtraction.correct
            + t.multiplication.correct
            + t.division.correct
            + t.word.correct
            + t.mixed.correct
    };

    let week_correct = total_correct(&totals);
    let week_pct = pct_label(week_attempts, week_correct);

    // Pull skill stats
    let skills: Vec<(&'static str, u32, u32, String)> = vec![
        ("Addition", totals.addition.attempts, totals.addition.correct, pct_label_from(&totals.addition)),
        ("Subtraction", totals.subtraction.attempts, totals.subtraction.correct, pct_label_from(&totals.subtraction)),
        ("Multiplication", totals.multiplication.attempts, totals.multiplication.correct, pct_label_from(&totals.multiplication)),
        ("Division", totals.division.attempts, totals.division.correct, pct_label_from(&totals.division)),
        ("Word Problems", totals.word.attempts, totals.word.correct, pct_label_from(&totals.word)),
        ("Mixed Skills", totals.mixed.attempts, totals.mixed.correct, pct_label_from(&totals.mixed)),
    ];

    // Most practiced
    let most_practiced = {
        let mut v = skills.clone();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.into_iter().find(|(_, att, _, _)| *att > 0)
    };

    // Strongest (10+ attempts): highest accuracy
    let strongest = {
        let mut v: Vec<(&'static str, u32, u32, i32)> = skills
            .iter()
            .filter_map(|(name, att, cor, _)| {
                if *att >= 10 {
                    let pct = ((*cor as f32) / (*att as f32) * 100.0).round() as i32;
                    Some((*name, *att, *cor, pct))
                } else {
                    None
                }
            })
            .collect();
        v.sort_by(|a, b| b.3.cmp(&a.3));
        v.into_iter().next()
    };

    // Focus next (10+ attempts): lowest accuracy under 75% if possible, otherwise lowest overall
    let focus = {
        let mut v: Vec<(&'static str, u32, u32, i32)> = skills
            .iter()
            .filter_map(|(name, att, cor, _)| {
                if *att >= 10 {
                    let pct = ((*cor as f32) / (*att as f32) * 100.0).round() as i32;
                    Some((*name, *att, *cor, pct))
                } else {
                    None
                }
            })
            .collect();

        v.sort_by(|a, b| a.3.cmp(&b.3));
        let below_75 = v.iter().cloned().find(|(_, _, _, pct)| *pct < 75);
        below_75.or_else(|| v.into_iter().next())
    };

    let plan_text = |skill: &str| -> &'static str {
        match skill {
            "Addition" => "Do 10 quick addition facts (0–20). Say answers out loud. Repeat misses.",
            "Subtraction" => "Do 10 subtraction facts (0–20). For tough ones, count back slowly.",
            "Multiplication" => "Pick one table (2s/5s/10s). Do 10 facts, then 3 mixed.",
            "Division" => "Use fact families: 12÷3, 3×4, 12÷4… keep numbers small.",
            "Word Problems" => "Read 2 word problems. Ask: “What are we solving for?” then “Which operation?”",
            _ => "Do 8 mixed questions. Identify the operation first, then solve.",
        }
    };

    // Daily breakdown rows
    let day_rows = last7.iter().map(|(ymd, d)| {
        let att = total_attempts_for_week(d);
        let cor = total_correct(d);
        let day_pct = pct_label(att, cor);

        // Best/worst skill for the DAY (3+ attempts so it’s meaningful)
        let mut day_skill_pcts: Vec<(&'static str, u32, u32, i32)> = vec![
            ("Addition", d.addition.attempts, d.addition.correct, 0),
            ("Subtraction", d.subtraction.attempts, d.subtraction.correct, 0),
            ("Multiplication", d.multiplication.attempts, d.multiplication.correct, 0),
            ("Division", d.division.attempts, d.division.correct, 0),
            ("Word Problems", d.word.attempts, d.word.correct, 0),
            ("Mixed Skills", d.mixed.attempts, d.mixed.correct, 0),
        ]
        .into_iter()
        .filter_map(|(name, a, c, _)| {
            if a >= 3 && a > 0 {
                let p = ((c as f32) / (a as f32) * 100.0).round() as i32;
                Some((name, a, c, p))
            } else {
                None
            }
        })
        .collect();

        day_skill_pcts.sort_by(|a, b| b.3.cmp(&a.3));
        let best = day_skill_pcts.first().map(|(n, _, _, p)| format!("🏅 {} ({}%)", n, p)).unwrap_or_else(|| "—".to_string());

        day_skill_pcts.sort_by(|a, b| a.3.cmp(&b.3));
        let worst = day_skill_pcts.first().map(|(n, _, _, p)| format!("🎯 {} ({}%)", n, p)).unwrap_or_else(|| "—".to_string());

        html! {
            <tr style="border-top: 1px solid rgba(0,0,0,.06);">
                <td style="padding:8px 6px; font-weight:700;">{ymd.clone()}</td>
                <td style="padding:8px 6px; text-align:right;">{att}</td>
                <td style="padding:8px 6px; text-align:right;">{day_pct}</td>
                <td style="padding:8px 6px;">{best}</td>
                <td style="padding:8px 6px;">{worst}</td>
            </tr>
        }
    });

    // Build plain-text report
    let report_text = {
        let mut lines: Vec<String> = vec![];
        lines.push("THE NUMBERS — Weekly Report (Last 7 days)".to_string());
        lines.push(format!("Total: {} attempts • {} accuracy", week_attempts, week_pct));
        lines.push("".to_string());

        if let Some((name, att, cor, _)) = most_practiced {
            lines.push(format!("Most practiced: {} ({} tries • {})", name, att, pct_label(att, cor)));
        }
        if let Some((name, att, _cor, pct)) = strongest {
            lines.push(format!("Strongest (10+ tries): {} ({}% • {} tries)", name, pct, att));
        }
        if let Some((name, att, _cor, pct)) = focus {
            lines.push(format!("Focus next (10+ tries): {} ({}% • {} tries)", name, pct, att));
            lines.push(format!("2-minute plan: {}", plan_text(name)));
        } else {
            lines.push("Focus next: Balanced week (no clear weak spot with 10+ tries)".to_string());
        }

        lines.push("".to_string());
        lines.push("By skill:".to_string());
        for (name, att, cor, pct) in skills.iter() {
            lines.push(format!("• {}: {} tries, {} correct ({})", name, att, cor, pct));
        }

        lines.push("".to_string());
        lines.push("Daily breakdown:".to_string());
        for (ymd, d) in last7.iter() {
            let a = total_attempts_for_week(d);
            let c = total_correct(d);
            let p = pct_label(a, c);
            lines.push(format!("• {}: {} tries • {}", ymd, a, p));
        }

        lines.join("\n")
    };

    let on_copy_weekly = {
        let copied_toast = copied_toast.clone();
        Callback::from(move |_| {
            let ok = copy_text_to_clipboard(&report_text);
            if ok {
                copied_toast.set(true);
            }
        })
    };

    html! {
        <div style="margin: 0 0 16px; padding: 12px 14px; border:1px solid #ddd; border-radius: 14px;">
            <div style="display:flex; justify-content:space-between; align-items:flex-start; gap: 10px; flex-wrap:wrap;">
                <div>
                    <div style="font-weight: 900;">{"👨‍👩‍👧 Parent Summary (Last 7 days)"}</div>
                    <div style="margin-top: 6px; opacity:.85;">
                        {format!("{} attempts • {} accuracy", week_attempts, week_pct)}
                    </div>
                </div>

                <div style="display:flex; gap:10px; flex-wrap:wrap; align-items:center;">
                    <button
                        onclick={on_copy_weekly}
                        style="padding:10px 12px; border-radius:10px; border:1px solid #222; background:#fff; cursor:pointer;"
                        title="Copy a plain-text weekly report to paste into email/text"
                    >
                        {"📋 Copy weekly report"}
                    </button>

                    {
                        if *copied_toast {
                            html!{
                                <div style="padding:8px 10px; border-radius: 999px; border:1px solid #e6e6e6; background: rgba(0,128,0,.08); font-weight:700;">
                                    {"Copied ✅"}
                                </div>
                            }
                        } else {
                            html!{}
                        }
                    }
                </div>
            </div>

            <div style="margin-top: 12px; padding: 10px 12px; border-radius: 12px; border: 1px solid #eee; background: rgba(0,0,0,.02);">
                <div style="font-weight:900;">{"🧭 Do this next (2 minutes)"}</div>
                <div style="margin-top:6px; opacity:.9;">
                    {
                        if let Some((name, _, _, _)) = focus {
                            plan_text(name).to_string()
                        } else {
                            "Do 8 mixed questions and explain ONE answer out loud.".to_string()
                        }
                    }
                </div>
            </div>

            <div style="margin-top: 12px; font-weight:900;">{"📅 Daily breakdown"}</div>

            <div style="margin-top: 8px; overflow-x:auto;">
                <table style="width:100%; border-collapse: collapse;">
                    <thead>
                        <tr style="opacity:.7; font-size: 13px; text-align:left;">
                            <th style="padding:6px;">{"Date"}</th>
                            <th style="padding:6px; text-align:right;">{"Attempts"}</th>
                            <th style="padding:6px; text-align:right;">{"Accuracy"}</th>
                            <th style="padding:6px;">{"Best skill"}</th>
                            <th style="padding:6px;">{"Needs work"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for day_rows }
                    </tbody>
                </table>
            </div>

            <div style="margin-top: 12px; overflow-x:auto;">
                <table style="width:100%; border-collapse: collapse;">
                    <thead>
                        <tr style="opacity:.7; font-size: 13px; text-align:left;">
                            <th style="padding:6px;">{"Skill"}</th>
                            <th style="padding:6px; text-align:right;">{"Attempts"}</th>
                            <th style="padding:6px; text-align:right;">{"Correct"}</th>
                            <th style="padding:6px; text-align:right;">{"Accuracy"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td style="padding:8px 6px; font-weight:700;">{"Addition"}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.addition.attempts}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.addition.correct}</td>
                            <td style="padding:8px 6px; text-align:right;">{pct_label_from(&totals.addition)}</td>
                        </tr>
                        <tr>
                            <td style="padding:8px 6px; font-weight:700;">{"Subtraction"}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.subtraction.attempts}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.subtraction.correct}</td>
                            <td style="padding:8px 6px; text-align:right;">{pct_label_from(&totals.subtraction)}</td>
                        </tr>
                        <tr>
                            <td style="padding:8px 6px; font-weight:700;">{"Multiplication"}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.multiplication.attempts}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.multiplication.correct}</td>
                            <td style="padding:8px 6px; text-align:right;">{pct_label_from(&totals.multiplication)}</td>
                        </tr>
                        <tr>
                            <td style="padding:8px 6px; font-weight:700;">{"Division"}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.division.attempts}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.division.correct}</td>
                            <td style="padding:8px 6px; text-align:right;">{pct_label_from(&totals.division)}</td>
                        </tr>
                        <tr>
                            <td style="padding:8px 6px; font-weight:700;">{"Word Problems"}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.word.attempts}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.word.correct}</td>
                            <td style="padding:8px 6px; text-align:right;">{pct_label_from(&totals.word)}</td>
                        </tr>
                        <tr>
                            <td style="padding:8px 6px; font-weight:700;">{"Mixed Skills"}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.mixed.attempts}</td>
                            <td style="padding:8px 6px; text-align:right;">{totals.mixed.correct}</td>
                            <td style="padding:8px 6px; text-align:right;">{pct_label_from(&totals.mixed)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div style="margin-top: 10px; font-size: 13px; opacity:.75;">
                {"Parent tip: ask your student to explain ONE answer out loud. That’s where understanding locks in."}
            </div>
        </div>
    }
}

// ----------------------------
// Home
// ----------------------------

#[function_component(Home)]
fn home() -> Html {
    let progress = use_state(load_progress);
    let grade = use_state(load_grade);

    let toast = use_state(|| None::<String>);
    let sound_on = use_state(|| is_sound_enabled());

    // Prefetch all 12 days in background whenever grade changes (best effort)
    {
        let toast = toast.clone();
        let g = *grade;

        use_effect_with(g, move |_| {
            let today = today_ymd_local();

            spawn_local(async move {
                let mut any_ai_success = false;

                for day_id in 1..=12 {
                    let cached = load_day_from_local_storage(g, day_id);
                    let needs = match &cached {
                        Some(d) => d.date_ymd != today,
                        None => true,
                    };
                    if !needs {
                        continue;
                    }

                    match fetch_day(WORKER_BASE_URL, g, day_id).await {
                        Ok(day) => {
                            save_day_to_local_storage(g, day_id, &day);
                            if day.source == "ai" {
                                any_ai_success = true;
                            }
                        }
                        Err(_) => {
                            // Worker down/offline; local fallback still works.
                        }
                    }
                }

                if any_ai_success {
                    toast.set(Some("Fresh questions loaded ✅".to_string()));
                }
            });

            || ()
        });
    }

    // auto-hide toast
    {
        let toast = toast.clone();
        use_effect_with((*toast).clone(), move |t| {
            if t.is_some() {
                let toast2 = toast.clone();
                let cb = Closure::<dyn FnMut()>::new(move || {
                    toast2.set(None);
                });
                if let Some(win) = web_sys::window() {
                    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        2500,
                    );
                }
                cb.forget();
            }
            || ()
        });
    }

    let completed_count = progress.completed.len();
    let all_done = completed_count >= DAYS.len();
    let pct = ((completed_count as f32) / (DAYS.len() as f32) * 100.0).round() as i32;

    let on_grade_change = {
        let grade = grade.clone();
        Callback::from(move |e: Event| {
            let v = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();

            let g = match v.as_str() {
                "1" => Grade::G1,
                "2" => Grade::G2,
                "3" => Grade::G3,
                "4" => Grade::G4,
                _ => Grade::G5,
            };

            save_grade(g);
            grade.set(g);
        })
    };

    html! {
        <>
            <h1 style="letter-spacing: .5px;">{"THE NUMBERS."}</h1>
            <p>{"Swap social media for 15 minutes of daily math mastery."}</p>

            {
                if let Some(msg) = &*toast {
                    html! {
                        <div style="
                            position: fixed;
                            right: 16px;
                            bottom: 16px;
                            padding: 10px 12px;
                            border-radius: 12px;
                            border: 1px solid #ddd;
                            background: #fff;
                            box-shadow: 0 6px 20px rgba(0,0,0,.12);
                            z-index: 9999;
                            font-weight: 600;
                        ">
                            {msg.clone()}
                        </div>
                    }
                } else {
                    html!{}
                }
            }

            <div style="display:flex; gap:12px; align-items:center; flex-wrap: wrap; margin: 10px 0 12px;">
                <span style="padding:6px 10px; border:1px solid #ddd; border-radius:999px;">
                    {format!("✅ Completed: {}/{}", completed_count, DAYS.len())}
                </span>

                <label style="display:flex; align-items:center; gap:8px; padding:6px 10px; border:1px solid #ddd; border-radius:999px;">
                    <span>{"🎓 Grade:"}</span>
                    <select onchange={on_grade_change} style="padding:6px 8px; border-radius:10px; border:1px solid #ddd;">
                        <option value="1" selected={*grade == Grade::G1}>{"1"}</option>
                        <option value="2" selected={*grade == Grade::G2}>{"2"}</option>
                        <option value="3" selected={*grade == Grade::G3}>{"3"}</option>
                        <option value="4" selected={*grade == Grade::G4}>{"4"}</option>
                        <option value="5" selected={*grade == Grade::G5}>{"5"}</option>
                    </select>
                </label>

                <button
                    style="padding:10px 14px; border-radius:10px; border:1px solid #222; background:#222; color:#fff; cursor:pointer;"
                    onclick={{
                        let progress = progress.clone();
                        Callback::from(move |_| {
                            let mut p = (*progress).clone();
                            p.completed.clear();
                            save_progress(&p);
                            progress.set(p);
                        })
                    }}
                >
                    {"Reset Progress"}
                </button>

                <button
                    style="padding:10px 14px; border-radius:10px; border:1px solid #ddd; background:#fff; cursor:pointer;"
                    onclick={{
                        let sound_on = sound_on.clone();
                        Callback::from(move |_| sound_on.set(!*sound_on))
                    }}
                    title="Toggle tiny sound effects"
                >
                    { if *sound_on { "🔊 Sound: On" } else { "🔇 Sound: Off" } }
                </button>
            </div>

            <div style="margin: 0 0 16px;">
                <div style="height: 10px; border-radius: 999px; background: rgba(0,0,0,.08); overflow:hidden;">
                    <div style={format!(
                        "height:100%; width:{}%; border-radius:999px; background: rgba(0,0,0,.75); transition: width 400ms ease;",
                        pct
                    )} />
                </div>
                <div style="margin-top:6px; font-size: 13px; opacity:.75;">
                    {format!("Progress: {}%", pct)}
                </div>
            </div>

            <ParentSummaryPanel />

            { if all_done { celebration_banner() } else { html!{} } }

            <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 14px;">
                { for DAYS.iter().map(|d| day_card(d, &progress)) }
            </div>

            <div style="margin-top: 14px; opacity:.7; font-size: 13px;">
                {"Daily refresh + caching enabled. If the AI service is flaky, the app still runs on built-in questions."}
            </div>

            <SoundFlag enabled={*sound_on} />
        </>
    }
}

fn day_card(day: &Day, progress: &UseStateHandle<Progress>) -> Html {
    let done = progress.completed.contains(&day.id);
    let (emoji, label) = day_tag(day.id);

    html! {
        <Link<Route> to={Route::Day { id: day.id }}>
            <div style="
                border:1px solid #ddd; border-radius:14px; padding:14px; text-decoration:none; color:inherit;
                box-shadow: 0 1px 0 rgba(0,0,0,.03);
            ">
                <div style="display:flex; justify-content:space-between; align-items:flex-start; gap: 10px;">
                    <div>
                        <div style="font-weight:700;">{format!("Day {}:", day.id)}</div>
                        <div style="font-size: 18px; margin-top: 4px;">{day.title}</div>
                        <div style="opacity:.75; margin-top: 6px;">{day.subtitle}</div>

                        <div style="
                            display:inline-flex; align-items:center; gap:8px;
                            margin-top:10px; padding:5px 10px;
                            border:1px solid #eee; border-radius:999px;
                            font-size: 13px; opacity:.9;
                        ">
                            <span>{emoji}</span>
                            <span>{label}</span>
                        </div>
                    </div>

                    <div style="font-size: 22px;">{ if done { "✅" } else { "⬜" } }</div>
                </div>
            </div>
        </Link<Route>>
    }
}

// ----------------------------
// Day View
// ----------------------------

#[derive(Properties, PartialEq)]
struct DayViewProps {
    id: usize,
}

#[function_component(DayView)]
fn day_view(props: &DayViewProps) -> Html {
    let progress = use_state(load_progress);
    let grade = load_grade();

    let day = DAYS.iter().find(|d| d.id == props.id).cloned();
    if day.is_none() {
        return html! { <h2>{"Day not found"}</h2> };
    }
    let day = day.unwrap();

    let (tag_emoji, tag_label) = day_tag(day.id);
    let done = progress.completed.contains(&day.id);

    let show_edu = use_state(|| false);

    let mark_done = {
        let progress = progress.clone();
        let id = day.id;
        Callback::from(move |_| {
            let mut p = (*progress).clone();
            if !p.completed.contains(&id) {
                p.completed.push(id);
            }
            save_progress(&p);
            progress.set(p);
        })
    };

    // Compute skills for today from cached AI (if fresh)
    let today = today_ymd_local();
    let cached = load_day_from_local_storage(grade, day.id);

    let skills_list: Vec<&'static str> = if let Some(d) = cached {
        if d.date_ymd == today {
            let mut skills: std::collections::BTreeSet<&'static str> =
                std::collections::BTreeSet::new();
            for q in d.items.iter() {
                skills.insert(skill_label(classify_skill(&q.prompt)));
            }
            skills.into_iter().collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    html! {
        <>
            <div style="display:flex; justify-content:space-between; align-items:flex-start; gap: 12px; flex-wrap: wrap;">
                <div>
                    <Link<Route> to={Route::Home}>{"← Back"}</Link<Route>>
                    <h2 style="margin: 8px 0 0;">{format!("Day {} — {}", day.id, day.title)}</h2>
                    <div style="opacity:.75; margin-top: 6px;">{day.subtitle}</div>

                    <div style="margin-top: 10px; display:flex; gap:10px; flex-wrap:wrap;">
                        <div style="padding:6px 10px; border:1px solid #ddd; border-radius:999px; display:inline-flex; gap:8px; align-items:center;">
                            <span>{tag_emoji}</span>
                            <span>{tag_label}</span>
                        </div>
                        <div style="padding:6px 10px; border:1px solid #ddd; border-radius:999px; display:inline-block;">
                            {format!("🎓 {}", grade.label())}
                        </div>
                    </div>
                </div>

                <div style="display:flex; gap:10px; flex-wrap:wrap;">
                    <button
                        style="padding:10px 14px; border-radius:10px; border:1px solid #ddd; background:#fff; cursor:pointer;"
                        onclick={{
                            let show_edu = show_edu.clone();
                            Callback::from(move |_| show_edu.set(!*show_edu))
                        }}
                    >
                        { if *show_edu { "Hide Parent/Educator" } else { "Parent/Educator" } }
                    </button>

                    <button
                        style="padding:10px 14px; border-radius:10px; border:1px solid #222; background:#222; color:#fff; cursor:pointer;"
                        onclick={mark_done}
                        disabled={done}
                    >
                        { if done { "Completed ✅" } else { "Mark Done" } }
                    </button>
                </div>
            </div>

            {
                if *show_edu {
                    html!{
                        <div style="margin-top: 14px; padding: 12px 14px; border:1px solid #ddd; border-radius: 14px;">
                            <div style="font-weight: 800;">{"🧑‍🏫 Parent / Educator Notes"}</div>
                            <div style="margin-top: 8px; opacity:.85;">
                                {"Skills likely covered today: "}
                                {
                                    if skills_list.is_empty() {
                                        html!{ <span>{"(not loaded yet — still fine; built-in practice works)"}</span> }
                                    } else {
                                        html!{ <span>{skills_list.join(", ")}</span> }
                                    }
                                }
                            </div>
                            <div style="margin-top: 10px; font-size: 13px; opacity:.75;">
                                {"Ask the student to explain ONE answer out loud. That builds confidence + understanding."}
                            </div>
                        </div>
                    }
                } else {
                    html!{}
                }
            }

            <hr style="margin: 18px 0;" />

            <h3>{"15-minute session"}</h3>

            <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 14px;">
                <section style="border:1px solid #ddd; border-radius:14px; padding:14px;">
                    <div style="font-weight:700;">{"1) Learn (3 min)"}</div>
                    <p style="opacity:.85;">{day.learn}</p>
                </section>

                <section style="border:1px solid #ddd; border-radius:14px; padding:14px;">
                    <div style="font-weight:700;">{"2) Practice (10 min)"}</div>
                    <p style="opacity:.85;">{day.practice}</p>

                    <PracticeWidget
                        key={format!("practice-{}-{}", day.id, grade.as_u8())}
                        day_id={day.id}
                        grade={grade}
                    />
                </section>

                <section style="border:1px solid #ddd; border-radius:14px; padding:14px;">
                    <div style="font-weight:700;">{"3) Win (2 min)"}</div>
                    <p style="opacity:.85;">{day.win}</p>
                </section>
            </div>
        </>
    }
}

// ----------------------------
// Practice Widget (AI day cache first; fallback local)
// ----------------------------

#[derive(Properties, PartialEq)]
struct PracticeWidgetProps {
    day_id: usize,
    grade: Grade,
}

fn normalize_answer(s: &str) -> String {
    s.trim().to_lowercase().replace(' ', "")
}

fn is_correct(guess: &str, correct: &str) -> bool {
    let g = normalize_answer(guess);
    correct
        .split('|')
        .map(|x| normalize_answer(x))
        .any(|c| !c.is_empty() && g == c)
}

// Choose a question from AI cached day if fresh; else local questions.rs.
fn pick_question(day_id: usize, grade: Grade) -> (String, String, bool) {
    let today = today_ymd_local();

    if let Some(day) = load_day_from_local_storage(grade, day_id) {
        if day.date_ymd == today && !day.items.is_empty() {
            let core: Vec<_> = day
                .items
                .iter()
                .filter(|q| q.difficulty == AiDifficulty::Core)
                .collect();
            let stretch: Vec<_> = day
                .items
                .iter()
                .filter(|q| q.difficulty == AiDifficulty::Stretch)
                .collect();

            let use_stretch = !stretch.is_empty() && (rand::random::<u8>() % 10) < 3;
            let pick_from = if use_stretch { &stretch } else { &core };

            let mut rng = rand::thread_rng();
            if let Some(q) = pick_from.choose(&mut rng) {
                return (
                    q.prompt.clone(),
                    q.answer.clone(),
                    q.difficulty == AiDifficulty::Stretch,
                );
            }
        }
    }

    let bank: Vec<LocalQuestion> = local_questions_for(day_id, grade);
    let core: Vec<_> = bank
        .iter()
        .filter(|q| q.difficulty == LocalDifficulty::Core)
        .collect();
    let stretch: Vec<_> = bank
        .iter()
        .filter(|q| q.difficulty == LocalDifficulty::Stretch)
        .collect();

    let use_stretch = !stretch.is_empty() && (rand::random::<u8>() % 10) < 3;
    let pick_from = if use_stretch { &stretch } else { &core };

    let mut rng = rand::thread_rng();
    if let Some(q) = pick_from.choose(&mut rng) {
        return (
            q.prompt.to_string(),
            q.answer.to_string(),
            q.difficulty == LocalDifficulty::Stretch,
        );
    }

    let q = bank.choose(&mut rng).unwrap();
    (
        q.prompt.to_string(),
        q.answer.to_string(),
        q.difficulty == LocalDifficulty::Stretch,
    )
}

#[derive(Clone, PartialEq)]
struct QA {
    prompt: String,
    answer: String,
    is_stretch: bool,
}

#[function_component(PracticeWidget)]
fn practice_widget(props: &PracticeWidgetProps) -> Html {
    let input = use_state(|| "".to_string());
    let feedback = use_state(|| "".to_string());

    // Pick ONCE per mount so the question does not change while typing.
    let qa = use_state(|| {
        let (p, a, s) = pick_question(props.day_id, props.grade);
        QA {
            prompt: p,
            answer: a,
            is_stretch: s,
        }
    });

    let on_new_question = {
        let input = input.clone();
        let feedback = feedback.clone();
        let qa = qa.clone();

        let day_id = props.day_id;
        let grade = props.grade;

        Callback::from(move |_| {
            let (p2, a2, s2) = pick_question(day_id, grade);
            qa.set(QA {
                prompt: p2,
                answer: a2,
                is_stretch: s2,
            });

            input.set("".to_string());
            feedback.set("".to_string());
        })
    };

    let on_check = {
        let input = input.clone();
        let feedback = feedback.clone();
        let qa_now = (*qa).clone();

        Callback::from(move |_| {
            let ok = is_correct(&input, &qa_now.answer);

            // record attempts into weekly/day buckets
            let mut st = load_stats();
            let skill = classify_skill(&qa_now.prompt);
            record_attempt(&mut st, skill, ok);
            save_stats(&st);

            if ok {
                if is_sound_enabled() {
                    if qa_now.is_stretch {
                        play_tone(880.0, 80, 0.05);
                        play_tone(1175.0, 90, 0.04);
                    } else {
                        play_tone(880.0, 80, 0.05);
                    }
                }

                feedback.set(if qa_now.is_stretch {
                    "🌟 Stretch win! Awesome job.".to_string()
                } else {
                    "✅ Nice! You got it.".to_string()
                });
            } else {
                feedback.set("❌ Not yet — try again.".to_string());
            }
        })
    };

    html! {
        <div style="margin-top: 10px;">
            <div style="display:flex; align-items:center; justify-content:space-between; gap:10px; flex-wrap:wrap;">
                <div style="display:flex; align-items:center; gap:8px;">
                    <div style="font-weight:600;">{(*qa).prompt.clone()}</div>
                    { if (*qa).is_stretch { html!{ <span title="Stretch question">{"🌟"}</span> } } else { html!{} } }
                </div>

                <button
                    onclick={on_new_question}
                    style="padding:8px 10px; border-radius:10px; border:1px solid #ddd; background:#fff; cursor:pointer;"
                    title="Get a new question"
                >
                    {"New question"}
                </button>
            </div>

            <div style="display:flex; gap:10px; align-items:center; flex-wrap: wrap; margin-top: 10px;">
                <input
                    value={(*input).clone()}
                    oninput={{
                        let input = input.clone();
                        let feedback = feedback.clone();
                        Callback::from(move |e: InputEvent| {
                            let v = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            input.set(v);
                            feedback.set("".to_string());
                        })
                    }}
                    placeholder="Your answer"
                    style="padding:10px 12px; border:1px solid #ddd; border-radius:10px; min-width: 200px;"
                />
                <button
                    onclick={on_check}
                    style="padding:10px 12px; border-radius:10px; border:1px solid #222; background:#fff; cursor:pointer;"
                >
                    {"Check"}
                </button>
            </div>

            {
                if !(*feedback).is_empty() {
                    html! { <div style="margin-top: 8px;">{(*feedback).clone()}</div> }
                } else {
                    html!{}
                }
            }
        </div>
    }
}