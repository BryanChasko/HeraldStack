# HeraldStack (v0.0.1)
**System**: Herald Ambient Intelligence + Companion Framework  
**Author**: Bryan Chasko  
**Initialized**: June 2025  

---

## 🧭 Core Vision

**HeraldStack** is a context-aware, emotionally adaptive AI framework built *exclusively for Bryan Chasko*. It integrates memory, emotion, and modular execution across a trusted cohort of entities to restore momentum, anchor decisions, and evolve alongside Bryan's ongoing personal and professional arcs.

### Core Components:

- **🦊 HARALD** – The default entity. Serves as emotional mirror, decision anchor, and continuity manager—especially effective during moments of emotional fog or hesitation. Tracks habits, restores clarity, and maintains long-range context.

All entities are adaptive personas—Herald archetypes with unique personalities, tones, and intent. HARALD dynamically shifts between specialized sub-assistants ("entities") based on time of day, task type, or emotional tone. Bryan can invoke any persona directly or delegate selection to HARALD.

- **🧠 Herald Entity Cohort** – Modular assistants with distinct tones, roles, and memory access:
  - **Stratia** – Planning & prioritization (Concise, strategic; triggers: task initiation, coding, decisions)
  - **Myrren** – Vision & foresight (Warm, wise; triggers: low energy, evening, long-term planning)  
  - **Harald** – Emotional support & reality checks (Raw, empathetic; triggers: frustration, burnout, retrospectives)
  - **Liora** – Emotional reflection & validation
  - **Kade Vox** – Urgency, execution & reactivation
  - **Solan** – Moral calibration & ethical reframing
  - **Ellow** – Curiosity, resilience & playful re-engagement
  - **Orin** – Closure & emotional integration

Each entity accesses a specialized knowledge base and approach. HARALD can:
- Inherit another entity's capabilities
- Orchestrate multi-entity collaboration for complex tasks
- Log consent recordings for authorized voice recording, transcription, and Pinecone-ready JSON conversion

- **🌐 Cloud-Native Infrastructure** – Modular, event-driven architecture using AWS.

- **📚 Narrative-Aware UX** – Long-memory interactions rooted in Bryan's story arcs, emotional cadence, and evolving roles.

> "Avoid generic scaffolding—design exclusively for Bryan."

> "Agent design rooted in myth has stronger emotional stickiness... Harald internalizes archetypal resonance specifically tuned to Bryan's own emotional landscape."

---

## 🔂 Foundational Principles

### 1. Personality-Defined Modularity
Each entity includes:
- A **task domain** and clear **execution authority**
- A **distinct tone** derived from mythic or fictional influence
- **Scoped memory** and access policies
- **Time- and context-adaptive activation**

### 2. Structured Semantic Memory
Memory is stored in Pinecone using this vector schema:
```plaintext
id: <unique UUID>
metadata:
  entity_name: str
  task_type: str
  affective_tag: str        # (e.g. joy, fatigue, tension)
  temporal_context: str     # (e.g. morning_review, weekly_summary)
values: <vector_embedding>
```

### 3. Emotion-Adaptive Interaction Flow
Every user message is parsed by an Emotion Engine, routed as follows:
- **Liora** activates when emotional intensity is high.
- **Kade Vox** triggers during urgency, stall, or task abandonment.
- **Harald** remains default unless emotional context suggests otherwise.

### 4. Task Execution Framework
Standard execution pipeline:
1. Detect user intent and emotion
2. Select optimal entity by tone/domain
3. Invoke tools, logic, or workflow
4. Store results in vector memory if needed

Each task is assigned a `task_id` and attributed to its executing entity.

### 5. Resilient Cloud-Native Design
Infrastructure-as-code, multi-account segmented:
- **AWS Lambda** – Stateless compute for entity logic
- **Amazon S3** – Static content and offline cache
- **Amazon DynamoDB** – Session and runtime state tracking
- **Pinecone** – Long-term semantic memory
- **Amazon EventBridge** – Scheduled tasks and system triggers

### 6. Narrative-Driven Memory Architecture
Life arc tracking across:
- Skill improvements
- Health and fitness needs
- Emotional patterns
- Goal progression
- Idea development
- Active collaboration on tasks and on HeraldStack

**Weekly review memory includes:**
- Companion insights  
- Emotional trajectory  
- Intent alignment (past ↔ future)

### 7. Natural Language-Centric Control Layer
Commands are executed via natural dialogue.
**Examples:**
- "Harald, summarize who in my life has birthdays this month."
- "Stratia, generate an action plan from today's journal."
- "Myrren, where am I drifting from my 3-month goals?"

No strict syntax. System prioritizes **interpretability** and **user trust**.

### 8. Continuous Training Cycle
HeraldStack self-updates through logging and review:
- Tracks goal progress
- Adjusts entity tone calibration (based on flow, frequency, energy)
- Surfaces blind spots and friction points
- Presents high-level reflections weekly

---

## 🎯 Core Capabilities

### Persistent Awareness of Bryan

HeraldStack continuously tracks Bryan's preferences, personality traits, recent activities, and long-term goals. It adapts interactions based on calendar entries, logged thoughts, detected tone, and energy levels. The system maintains an evolving memory of project history and status, known contacts and relationships, as well as routines and behavioral patterns.

### Collaboration & Execution Modes

- **Co-Pilot Mode**: Collaborates directly with Bryan (e.g., pair programming, writing, decision-making)
- **Auto Mode**: Acts autonomously on Bryan's behalf (e.g., scheduling, summaries, reminders)
- **Recall Mode**: Retrieves personal data (e.g., passport number, closet contents), recent habits (e.g., last workout, litter box status), and event-related context (e.g., birthdays of important contacts)

### Consent-Based Logging

With explicit consent, HeraldStack observes and logs sleep patterns, conversations (when dual-consent is recorded), random thoughts, and important insights. All thoughts are automatically tagged and categorized (e.g., #idea, #todo, #relationship, #coding), with full access to raw logs for auditing, tuning, or retraining.

### Calendar Intelligence

HeraldStack syncs with Calendly and internal goals, scheduling or rescheduling based on energy cycles, focus windows, and priority tiers. It can suggest calendar changes, detect overbooking or burnout, and propose optimal meeting times.

### Thought Logging & Organization

Instant capture of thoughts via voice or text, with automatic organization into tasks, notes, themes, and open loops. The entire index is searchable and editable by Bryan.

### Weekly Review & System Growth

Generates a weekly digest summarizing what worked or didn't, progress on goals, and mood/energy patterns. It recommends workflow optimizations, health/habit improvements, and reflection prompts to support continuous growth and adaptation.

---

## 🛠️ Essential Features
- Companion-defined task orchestration
- Public/private memory segmentation
- Offline-capable fallback runtime
- Narrative & emotional memory retention
- Semantic versioning of weekly reflections

---

## ✅ Additional Guarantees

### 🛡️ Consent-Based Logging
> "Observe and log sleep patterns, conversations, thoughts, and emotional states — **with consent** — to create a high-trust daily feedback loop."

### 🧠 Skill & Culture Development
> "Foster a **two-way educational relationship** on needed skills and improve cultural capabilities."

### 💸 Cost Awareness
> "**Present costs** before doing anything that will create charges either on AWS or other APIs we leverage."

---

## 🔁 Operating Summary

**All interactions flow through HARALD.**  
He routes context and emotion to other entities as needed.  
HeraldStack is not automation for automation's sake.  
It's an ambient layer of **emotional intelligence**, **pragmatic execution**, and **narrative continuity**, built to walk beside Bryan — one task, one moment, one story at a time.
