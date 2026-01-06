# THE NUMBERS 🧠✨  
**12 Days of Math — daily practice for kids, clarity for parents**

THE NUMBERS is a lightweight, web-based math practice app designed to replace mindless scrolling with **15 minutes of focused daily math**. It blends kid-friendly practice, AI-assisted question generation, and **actionable parent insights** — all running as a static Rust/WASM app.

Live build runs on static hosting (IIS / Hostek) with zero server requirements.

---

## 🎯 What This App Does

### For Students
- 📅 **12 themed math days**
- ⏱️ Structured **15-minute sessions** (Learn → Practice → Win)
- 🎯 Core + stretch questions
- 🎵 Optional subtle sound feedback
- 💾 Works offline after first load

### For Parents & Educators
- 👨‍👩‍👧 **Weekly Parent Summary (Last 7 Days)**
- 📊 Skill-level accuracy tracking
- 🔥 Most practiced skill
- 🏅 Strongest skill
- 🎯 Focus-next recommendation
- 🧭 2-minute actionable practice plan
- 📋 **Copy Weekly Report** (paste into email, notes, or LMS)

### For Developers
- 🦀 Built in **Rust + Yew**
- ⚡ Compiled to **WASM**
- 🌐 Static hosting (no backend required)
- 🤖 Optional Cloudflare Worker for AI-generated daily questions
- 🧠 LocalStorage-backed progress + stats

---

## 🧱 Tech Stack

- **Rust** (stable)
- **Yew** (frontend framework)
- **WASM** (`wasm32-unknown-unknown`)
- **Trunk** (build pipeline)
- **Cloudflare Workers** (optional AI question service)
- **Static Hosting** (IIS / Hostek compatible)

---

## 🚀 Live Architecture

```text
Browser (WASM)
   │
   ├─ Local questions (offline-safe)
   ├─ LocalStorage (progress + weekly stats)
   └─ Optional fetch →
        Cloudflare Worker (AI daily questions)

If the worker is unavailable, the app automatically falls back to built-in questions.

📁 Project Structure
the-numbers/
├─ src/
│  ├─ app.rs          # Main Yew app
│  ├─ data.rs         # Day definitions
│  ├─ grade.rs        # Grade handling
│  ├─ questions.rs   # Local question bank
│  ├─ stats.rs       # Weekly stats + parent summary logic
│  ├─ storage.rs     # LocalStorage persistence
│  └─ ai_day.rs      # Worker API types
├─ dist/              # Built static output (optional to commit)
├─ index.html
├─ Trunk.toml
├─ Cargo.toml
└─ README.md

🛠️ Local Development
Prerequisites

Rust (stable)

wasm32-unknown-unknown target

Trunk

rustup target add wasm32-unknown-unknown
cargo install trunk

Run locally
trunk serve

Production build
trunk build --release


Output goes to dist/.

🌍 Deployment (Static Hosting)

Works on:

IIS (Hostek)

Apache / Nginx

GitHub Pages

Cloudflare Pages

IIS / Hostek notes

Ensure .wasm MIME type is mapped:

<mimeMap fileExtension=".wasm" mimeType="application/wasm" />


Use HashRouter (already configured) — no rewrites required

🤖 Optional AI Question Service

Daily questions can be served by a Cloudflare Worker.

Configured in app.rs:

const WORKER_BASE_URL: &str = "https://the-numbers-worker.mikegyver.workers.dev";


If unavailable, the app continues normally using built-in questions.

🔐 Privacy & Safety

No accounts

No tracking

No ads

No external analytics

All progress stored locally in the browser

📌 Philosophy

Small daily wins beat big occasional efforts.

THE NUMBERS is designed to:

Reduce friction

Build confidence

Encourage explanation, not memorization

Give parents clarity, not dashboards

🏷️ Versioning

v1.0 — Initial release

Student practice flow

Parent Summary (weekly)

Copy Weekly Report

Static deployment ready

📄 License

MIT (or update as desired)

🙌 Credits

Built by MikeGyver Studio
Designed for real families, real kids, and real attention spans.
