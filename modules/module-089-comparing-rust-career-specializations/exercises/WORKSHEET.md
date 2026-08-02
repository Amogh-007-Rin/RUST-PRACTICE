# Module 089 Worksheet

This is a written exercise, not a coding one. There are no automated tests — the deliverable is a research document and a plan. Spend roughly equal time on each prompt (15-20 minutes each). Be specific: name real companies, real libraries, and real projects. Generic answers ("I want to work in blockchain because it's the future") are not useful to you or to a hiring manager.

---

## 1. Your Top 3 Rust Specialization Interests

List your top 3 Rust specialization interests (from: backend web, embedded systems, blockchain/smart contracts, game development, WASM/frontend). For each:

1. The specialization name
2. Why it appeals to you (2-3 sentences)
3. One specific project idea you would build to demonstrate skills in this area

| # | Specialization | Why It Appeals | Project Idea |
|---|---------------|----------------|-------------|
| 1 | | | |
| 2 | | | |
| 3 | | | |

---

## 2. Companies Hiring Per Specialization

For EACH of your top 3 specializations, research 3 real companies actively using Rust. For each company, write:

1. Company name
2. What they build with Rust (specific product or service, not "they use Rust")
3. Whether they were hiring Rust roles (as of your research date)
4. What caught your attention about them

| Specialization | Company | What They Build | Hiring? | Why Interesting |
|---------------|---------|----------------|---------|-----------------|
| | | | | |
| | | | | |
| | | | | |

(Repeat for each specialization — 9 rows total)

---

## 3. Tech Stack Comparison

Compare the typical tech stacks across the five specializations. For each, list:

- Primary framework(s) or engine
- Database / storage layer
- Serialization / encoding
- Build tooling
- Testing approach
- Deployment target (server, MCU, browser, validator node, game binary)

| Layer | Backend | Embedded | Blockchain | Game Dev | WASM |
|-------|---------|----------|------------|----------|------|
| Framework | Axum/Actix | embedded-hal | Solana SDK / Substrate | Bevy | Leptos/Dioxus |
| Storage | Postgres (sqlx) | Flash / EEPROM | On-chain state | Scene graph | IndexedDB / Memory |
| Serialization | serde (JSON) | postcard / bincode | SCALE / Borsh | ron / custom | serde (JS boundary) |
| Build | cargo | cargo + probe-rs | cargo + solana CLI | cargo | wasm-pack / trunk |
| Testing | #[tokio::test] + integration | defmt + QEMU | Anchor / unit tests | Bevy integration tests | wasm-pack test --headless |
| Deployment | Docker / K8s | Flashed firmware | On-chain program deploy | Native binary / WASM | Static web server |

Fill in: based on the modules you have completed and your own research, what would you add or change in the table above? What is one stack element unique to each specialization that surprised you?

---

## 4. Your 3-Month Learning Plan

Pick ONE specialization from your top 3 and design a 12-week learning plan. Assume you have roughly 10-15 hours per week.

**Chosen specialization:** _______________

| Week | Focus Area | Rust.Stack Modules to Revisit | New Resources to Study | Deliverable |
|------|-----------|------------------------------|----------------------|-------------|
| 1 | | | | |
| 2 | | | | |
| 3 | | | | |
| 4 | | | | |
| 5 | | | | |
| 6 | | | | |
| 7 | | | | |
| 8 | | | | |
| 9 | | | | |
| 10 | | | | |
| 11 | | | | |
| 12 | | | | |

The "Deliverable" column should describe something concrete — a completed project, a merged PR, a deployed service, a published crate. A plan that ends with "I will have studied a lot of things" is not a plan.

---

## 5. Open-Source Projects to Target

Identify 2 real open-source projects in your chosen specialization that accept contributions. For each:

1. Project name and repository URL
2. What the project does (1-2 sentences)
3. Why contributing to it would advance your career goals
4. A specific "good first issue" or feature you could tackle (link to an issue or describe one)
5. What Rust skills from the curriculum you would apply

**Project 1:**
- Name & URL:
- What it does:
- Why it matters for your career:
- Potential contribution:
- Relevant Rust.Stack modules:

**Project 2:**
- Name & URL:
- What it does:
- Why it matters for your career:
- Potential contribution:
- Relevant Rust.Stack modules:

---

### Self-Assessment Checklist

Before considering this exercise complete, verify:

- [ ] I named at least 3 real companies per specialization (9+ total).
- [ ] My tech stack comparison identifies at least one surprising difference between specializations.
- [ ] My 3-month plan has concrete weekly deliverables, not just "study more."
- [ ] My open-source projects are real — not hypothetical, not "someday."
- [ ] I can explain my specialization choice to another developer in 30 seconds.
