# Module 097 Worksheet

These prompts guide you through auditing, improving, and framing your public developer profile for Rust roles. Write thoughtful, specific answers — this is the material you'll pull from when writing your real portfolio and resume.

---

## Prompt 1: Audit Your Current GitHub Profile

Open your GitHub profile page (`github.com/<your-username>`) as a hiring manager would. Write a candid audit covering:

- **Profile README:** Do you have a `githubusername/githubusername` profile README? If so, what does it communicate in the first 10 seconds? If not, what would you put there?
- **Pinned repositories:** Which repos are pinned? Do they show depth (one complex project) or breadth (many shallow ones)? Are the pinned repos relevant to the type of Rust role you want?
- **Repository READMEs:** Pick your top 3 repos. Do their READMEs have: a clear one-sentence pitch, installation instructions, a screenshot or demo, and a "why this exists" section? Rate each on a scale of 1–5.
- **Activity graph:** Does your contribution history show consistent work, or is it sparse? If sparse, how would you address that (e.g. regular open source contributions, a project you commit to daily)?
- **Languages / topics:** Do your repo topics and languages accurately reflect Rust? If GitHub shows your top language as JavaScript because of a config repo, how would you fix that?

Action item: List 3 concrete, high-impact changes you can make to your GitHub profile this week.

---

## Prompt 2: Write a README Case Study for a Project

Pick a project you've built — either from this repository (e.g. a capstone) or an existing personal project. Write a case-study-style README that would make a hiring manager stop scrolling. Include:

- **Title and one-sentence pitch** (what it is, who it's for)
- **The problem it solves** (real or hypothetical — frame it as a user story)
- **Architecture overview:** A 2–3 paragraph description of the codebase structure, the tech stack (Rust + which crates?), and the most interesting design decision you made.
- **A code snippet** showcasing the best-written part of the project (keep it under 20 lines)
- **What you learned** — be specific: "I learned that `Arc<RwLock<T>>` in a multi-threaded processor creates contention. Switching to sharded locks with `dashmap` dropped p99 latency from 12ms to 4ms." Avoid generic "I learned Rust."
- **If you were to rebuild it:** What would you do differently? One concrete change.

**Tip:** Look at `github.com/tonsky/FiraCode` or `github.com/BurntSushi/ripgrep` for READMEs that communicate credibility and competence without being salesy.

---

## Prompt 3: Draft a Resume Bullet Point for a Rust Project

Write one bullet point for your resume describing a Rust project you've built. Follow the "Action → Context → Result (numbers)" formula. Examples of strong Rust resume bullets:

- **Bad:** "Built a CLI tool in Rust." (No context, no impact, no specificity.)
- **Good:** "Built a multi-threaded log processor in Rust that parses 2M lines/sec across 8 cores, using `rayon` for parallelism and `serde` for structured output — replaced a Python script that took 45 seconds per run."
- **Good:** "Implemented a token-bucket rate limiter in Rust with `std`-only dependencies, handling 10K requests/sec in benchmarks with p99 latency under 2ms."

Your turn. Pick one of your Rust projects and write one bullet. If the project doesn't have obvious metrics, estimate reasonable numbers (e.g. "reduced startup time from X to Y," "processes Z items/sec") and note that they're estimates.

---

## Prompt 4: Design a Portfolio Project Idea

If you had 2–4 weeks to build one Rust project from scratch that would demonstrate depth in a specific specialization, what would you build? Write a 300–500 word project brief covering:

- **Specialization:** Which Rust career track does this target? (backend, systems/infra, embedded, game dev, blockchain, WASM, networking)
- **What it does:** A one-paragraph description of the finished project from the user's perspective.
- **Why it's impressive:** What specifically would make a hiring manager say "I want to talk to this person"?
- **Technical challenges:** What are the 2–3 hardest parts of building this? Which Rust features would you lean on heavily?
- **Dependency choices:** Which crates would you use and why? What would you deliberately implement yourself to show depth?
- **Deliverables:** Code, tests, benchmarks, documentation, a blog post? Which of these matter most for the role you're targeting?

**Examples of high-signal portfolio projects by specialization:**
- Backend: A REST API with auth, database migrations, integration tests, Docker, and a load-test report.
- Systems: A custom memory allocator, a toy database storage engine, or a file format parser with fuzz testing.
- Embedded: A `no_std` driver for a real sensor, with a blog post showing it running on hardware.
- WASM: An interactive data visualization compiled to WASM, with a before/after benchmark vs. JS.

---

## Prompt 5: List 5 Interview Talking Points From Your Portfolio

Imagine you're in a Rust job interview and the interviewer says: "Tell me about a Rust project you're proud of." List 5 specific things you could talk about — these should be derived from your portfolio projects, not generic Rust facts.

For each talking point, write:
- **What you'd say** (2–3 sentences — practice saying this out loud)
- **What it demonstrates** (ownership fluency? async expertise? performance awareness? design taste?)
- **A potential follow-up question** the interviewer might ask — and how you'd answer it.

**Example:**

> **Talking point:** "I built a rate limiter that uses a token bucket algorithm with injected timestamps for testing. The key insight was that advancing the refill clock by `added * interval` rather than `now_ms` preserves partial intervals — otherwise you lose up to one interval of refill time on every check."
>
> **Demonstrates:** Performance awareness, testable design, algorithmic thinking.
>
> **Follow-up:** "Why not use an off-the-shelf rate limiter crate?" → "The crate I found (`governor`) was great, but implementing it myself forced me to understand the algorithm deeply. In production I'd use the crate, but the exercise taught me exactly what to look for in a rate limiter implementation — particularly around clock injection for testing."
