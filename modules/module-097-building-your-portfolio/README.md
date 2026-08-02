# Module 097: Building Your Portfolio

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 60–90 min
**Prerequisites:** Module 096 (Open Source Contribution)

## Learning Objectives
- You will be able to audit your GitHub profile from a hiring manager's perspective and identify 3 high-impact improvements.
- You will be able to write a case-study-style README for a Rust project that communicates architecture, design decisions, and lessons learned.
- You will be able to draft a resume bullet point that follows the "Action → Context → Result" formula with concrete numbers.
- You will be able to design a portfolio project idea that demonstrates depth in one Rust specialization (backend, systems, embedded, WASM, game dev, blockchain).
- You will be able to derive 5 interview talking points from your portfolio projects and anticipate follow-up questions.

## Why This Matters

Rust developers are in demand, but "I know Rust" is not a differentiator — most candidates for Rust roles know Rust. What differentiates you is the narrative your portfolio tells. A well-structured GitHub profile with 2–3 polished project READMEs, a resume bullet that quantifies impact, and 5 interview talking points you can deliver without hesitation will get you past the resume screen and through the behavioral interview. This module is about packaging what you've built across Rust.Stack into a professional story that hiring managers recognize as job-readiness.

## Concept

### The Hiring Manager's 30-Second Scan

When a hiring manager opens your application, they spend about 30 seconds deciding whether to read further. In those 30 seconds, they look at, in order:

1. **Your resume bullet points** — specifically, the most recent 3. They're scanning for Rust keywords (`async`, `axum`, `tokio`, `no_std`, `embedded`), measurable impact ("reduced latency by 40%"), and role alignment (if the job is backend, they want to see backend projects).
2. **Your GitHub profile** — they click the link on your resume. They see: your pinned repos, your contribution graph, and your profile README (if you have one). They open 1–2 repos and skim the README.
3. **Your portfolio depth** — a single complex project with a case-study README, benchmarks, and a demo beats 10 half-finished `cargo init` repos. Depth over breadth.

If all three check out — resume is specific, GitHub is active and Rust-focused, one project shows real depth — you get a phone screen. If any fail — resume is vague ("strong problem-solver"), GitHub shows JavaScript as top language, pinned repos are stale — you get a polite rejection email.

This is not about gaming the system. It's about presenting your genuine work in the format that hiring managers are trained to evaluate. The Rust you've learned across 95+ modules of this curriculum is real. The capstones you've built are real. This module shows you how to frame them so they're seen.

### Structuring Your GitHub Profile

Your GitHub profile is the first external link on most resumes. It should communicate three things in under 10 seconds:

1. **What you do.** A profile README (in the `githubusername/githubusername` repository) that says "Rust developer | backend systems and CLI tools" in the first sentence. Not a life story — a professional headline.
2. **What you've built.** Three to four pinned repositories, all Rust, each with a README that has a clear pitch, a screenshot or demo link, and an architecture section.
3. **That you're active.** A contribution graph with at least 2–3 contributions per week for the past 3 months. Consistent activity signals discipline; a year-long gap signals you've moved on.

**Profile README essentials:**
```markdown
# Hi, I'm Alex — Rust Developer

I build backend services and developer tools in Rust.
Currently working through [Rust.Stack](https://github.com/...), a 100-module Rust curriculum.

**What I'm working on:**
- [Task Management API](link) — Axum + sqlx + Postgres, with JWT auth and Docker.
- [Log Processor](link) — Multi-threaded log aggregation at 2M lines/sec.
- [Contact Book CLI](link) — First Rust project; terminal-first with JSON persistence.

**Reach me:** [LinkedIn] | [Email]
```

Under 100 words. No emoji walls, no "passionate about," no generic adjectives. A hiring manager reads this in 5 seconds and knows exactly what you build and where to look next.

**Pinned repo strategy:** Pin your strongest 3–4 projects. If you have fewer than 3 strong projects, pin what you have and build more. The order matters — put your most impressive project first (leftmost). If you have one capstone from Rust.Stack that's fully polished, pin it. If you have two, pin both. The other pins can be small but complete utilities.

**Contribution graph realism:** You don't need a solid green graph. 2–3 contributions per week over 3 months is 24–36 squares — enough to show you're actively working in Rust, not enough to look like you're padding. Real development has quiet weeks. What hiring managers actually notice: is your graph mostly green in the recent past, or is it littered with months of gray?

### Writing Project READMEs That Get Attention

Most project READMEs on GitHub follow a "features + installation" format. This is functional but forgettable. A case-study README tells a story about the project: why it exists, what you learned, and what you'd do differently. This format is memorable because it shows *reflection* — a trait that distinguishes senior engineers from junior ones.

The case-study README structure:

```markdown
# Project Name

**One-sentence pitch:** A [what it is] for [who it's for] that [what it does better than alternatives].

## The Problem
One paragraph. Frame it as a user story. "As a developer maintaining a fleet of Raspberry Pis, I needed..."

## Architecture
2–3 paragraphs. Describe the crate/module structure, the key design decisions,
and the tech stack. Include a code snippet (under 20 lines) that showcases the
best-written part of the project.

## What I Learned
2–3 specific lessons. Use concrete before/after numbers if possible.
"I learned that dashmap sharded locks dropped p99 latency from 12ms to 4ms."

## If I Were to Rebuild It
One concrete change. Not "I'd write more tests" — something architectural.
"I'd use SQLite from day one instead of JSON files for concurrent access safety."
```

This structure works because it answers the three questions a hiring manager has about any project: "Did they build something useful?" (Problem), "Do they understand what they built?" (Architecture), and "Can they learn from mistakes?" (What I Learned + Rebuild It).

**Screenshots and demos:** If your project has a visual component (CLI output, a web UI, a terminal dashboard), include a screenshot or an animated GIF. Tools: `asciinema` for terminal recording, `peek` or `ScreenToGif` for desktop recording. A 10-second demo in the README is worth more than 1,000 words of description.

### Resume Framing for Rust Roles

Rust resume bullet points follow the "Action → Context → Result" formula, same as any other engineering resume. The difference: Rust roles care about memory safety, performance, and systems thinking more than most, so your bullets should emphasize those when applicable.

**Formula:**
```
[Verb] [what] using [Rust tech] that [measurable result]
```

**Weak vs. strong examples:**

| Weak | Strong |
|------|--------|
| "Built a CLI tool in Rust" | "Built a JSON-backed inventory CLI in Rust serving 10K+ items with sub-30ms startup, using `clap` for argument parsing and `memmap2`-backed I/O that reduced cold-start latency by 10x" |
| "Used async Rust" | "Designed an async web crawler with configurable concurrency limits using Tokio semaphores and `select!`-based cancellation, handling 500 concurrent fetches at 50 req/s without panics" |
| "Implemented data structures" | "Implemented a lock-free MPSC queue in Rust using `AtomicUsize` and `UnsafeCell`, achieving 20ns push and 30ns pop in criterion benchmarks on x86-64" |

Key principles:
- **Lead with the tech.** "Built X using Y" not "Using Y, built X." The technology is a signal, not the subject.
- **Quantify everything you can.** If you don't have exact numbers, estimate conservatively and note it. "Processed ~2M lines/sec on an 8-core machine (estimated from single-thread baseline)" is honest and specific.
- **Avoid buzzwords.** "Leveraged cutting-edge async paradigms" means nothing. "Used Tokio Semaphore for concurrency control" means something.
- **One bullet per project on the resume.** You can have multiple bullets for a professional job but for portfolio projects, one strong bullet is better than three weak ones.

### Portfolio Projects That Get Attention

Not all portfolio projects are equal. The ones that generate recruiter outreach have specific qualities:

**The "Gold Standard" portfolio project:**
1. **Is deployed.** A link to a running instance (even on a \$5 VPS) is infinitely more convincing than a screenshot. It proves the project survived deployment — config, environment variables, error handling, logging.
2. **Has tests.** A test suite that passes on CI shows you understand code quality as a practice, not an afterthought.
3. **Has documentation.** A README case study, API docs (`cargo doc` output hosted on GitHub Pages or `docs.rs`), and well-commented code.
4. **Has performance numbers.** A benchmark section in the README shows you measure and optimize — a systems-programming mindset.
5. **Solves a real problem.** Even a small problem ("I needed to convert CSV logs to JSON") beats a contrived one ("I built a to-do app because every tutorial does").

The projects that fall short:
- **To-do apps, weather CLI, and URL shorteners** (without novel architecture). These are tutorial projects. If your URL shortener uses a custom storage engine, a pluggable encoding scheme, and has a benchmark comparison against 3 alternatives, it's portfolio-worthy. If it's 50 lines wrapping `HashMap<String, String>`, it's a tutorial.
- **"Hello World" repos.** Delete them or make them private.
- **Forked-but-untouched.** Forking a repo without contributing to it signals nothing.
- **Projects with no README.** A repo without a README is invisible — hiring managers will not open the code to figure out what it does.

**The specialization strategy:** Pick one Rust domain and build depth in it. If you want a backend role, your portfolio should have 2–3 backend projects with different aspects (one REST API, one message queue worker, one database tool). If you want embedded, your portfolio should have `no_std` projects on real or simulated hardware. The "I do everything" portfolio (one CLI, one WASM demo, one game prototype, one smart contract) signals that you haven't committed to a domain — which is fine if you're exploring, but less compelling to a hiring manager hiring for a specific role.

### From Portfolio to Interview Talking Points

The final step: convert your portfolio projects into interview answers. The most common behavioral interview question for technical roles is "Tell me about a project you're proud of." You should be able to answer this without hesitation, with specific technical details, for each of your top 3 projects.

A strong answer has this shape:
1. **What it is** (one sentence)
2. **One specific technical challenge** you solved (the "hook")
3. **How you solved it** (the technical content — this is where you show Rust fluency)
4. **What the result was** (quantified if possible)
5. **What you learned** (reflection)

Practice saying these out loud. The difference between a written answer and a spoken answer is substantial — you'll find yourself stumbling over technical terms you can write fluently. Record yourself or practice with a friend. Aim for 45–90 seconds per project story.

Anticipate follow-up questions. If you mention using `Arc<Mutex<T>>`, expect: "Why not `tokio::sync::Mutex`?" If you mention `unsafe`, expect: "What invariant does the unsafe block uphold?" These follow-ups test whether you understand your own code or just copied patterns. They're the moments that separate strong candidates from average ones.

### The "Job Readiness" Self-Assessment

Before you start applying, run this checklist:

- [ ] My GitHub profile has a profile README with a professional headline.
- [ ] I have at least 3 pinned repositories, all Rust, with case-study READMEs.
- [ ] My contribution graph shows activity in the past 3 months.
- [ ] My resume has at least 2 Rust-specific bullet points with measurable results.
- [ ] I can describe each bullet point in 90 seconds out loud, with technical depth.
- [ ] I have at least one project that is deployed or demo-able (link in README).
- [ ] I have at least one open source contribution (merged PR or open PR in review).
- [ ] I know which Rust specialization I'm targeting and my portfolio reflects it.

If you check all 8, you're ready to apply. If you're missing any, the exercises in this module will help you fill the gaps.

## Common Pitfalls
- **Quantity over quality.** 10 repos with boilerplate READMEs harms your profile more than 2 repos with case studies. Archive or delete unfinished projects.
- **Writing READMEs in passive voice.** "A decision was made to use async Rust" is weak. "I chose async Rust because the 500-concurrent-fetch requirement would have required 500 OS threads" is strong. Own your decisions.
- **Making up numbers.** If you don't have benchmarks, don't invent them. Estimate conservatively and mark it: "~2M lines/sec (estimated from single-thread baseline)." Fake numbers are discoverable and disqualifying.
- **Ignoring the specialization strategy.** A portfolio split across backend, embedded, WASM, and blockchain signals "exploring" rather than "ready." Pick one and go deep.
- **Relying on the portfolio alone.** A strong portfolio gets you the interview. The interview gets you the job. Practice talking about your projects as much as you practice writing the code.

## Key Terms
- **Profile README:** a Markdown file in the `githubusername/githubusername` repository that appears at the top of your GitHub profile page.
- **Case-study README:** a project README structured as a narrative (Problem → Architecture → Lessons → Rebuild) rather than a feature list.
- **Action → Context → Result:** a formula for resume bullet points: what you did, in what context, with what measurable outcome.
- **Specialization strategy:** focusing a portfolio on one Rust domain (backend, systems, embedded, etc.) to signal depth rather than breadth.
- **Behavioral interview:** an interview format that asks about past experiences ("Tell me about a time when...") to assess soft skills, communication, and professionalism.
- **Pinned repositories:** the 6 repositories displayed prominently on a GitHub profile, manually selectable in the "Customize your pins" section.

## Exercise

This module uses a worksheet format. Open `exercises/WORKSHEET.md` and work through the five prompts:

1. **Audit Your Current GitHub Profile** — Evaluate your profile through a hiring manager's eyes: profile README, pinned repos, repo READMEs, activity graph, language stats. List 3 concrete improvements you can make this week.
2. **Write a README Case Study** — Pick an existing project (from this repo or your own) and write a case-study README with a one-sentence pitch, architecture overview, code snippet, lessons learned, and a "rebuild it" reflection.
3. **Draft a Resume Bullet Point** — Write one bullet for a Rust project following the Action → Context → Result formula. Quantify impact even if you have to estimate conservatively.
4. **Design a Portfolio Project Idea** — Write a 300–500 word project brief for a Rust project you'd build in 2–4 weeks, targeting one specialization. Cover: what it does, why it's impressive, technical challenges, dependency choices, and deliverables.
5. **List 5 Interview Talking Points** — For each: what you'd say (2–3 sentences), what it demonstrates, and one potential follow-up question with your answer.

There is no code to compile or tests to run. Write your answers thoughtfully — this material directly feeds your real resume, real portfolio, and real interview preparation. Sample answers are in `solutions/EXAMPLE_ANSWERS.md` for comparison.

## Further Reading
- [The Rust Job Market: What Employers Look For (blog)](https://filtra.io/rust-job-market-report) — survey data on what Rust hiring managers prioritize.
- [How to Write a Good README (GitHub guide)](https://github.com/matiassingers/awesome-readme) — a curated list of excellent project READMEs.
- [The Google Resume (book)](https://www.amazon.com/Google-Resume-Prepare-Microsoft-Companies/dp/0470927623) — practical resume-writing advice for tech roles.
- [Cracking the Coding Interview — Behavioral Questions chapter](https://www.crackingthecodinginterview.com/) — structured advice for behavioral interview preparation, applicable to Rust roles.
