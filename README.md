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
