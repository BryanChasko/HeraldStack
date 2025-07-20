📘 Griptape Best Practices Guide  
Version: v0.1.0 • Date: 2025‑07‑04 • Maintainer: HARALDStack Ops

🛠️ Core Philosophy  
Modular & Composable  
**Core Philosophy**

- **Modular & Composable:** Build with Agents, Pipelines, Workflows → combine
  small, single-purpose Tasks and Tools  
   [docs.griptapenodes.com](https://docs.griptapenodes.com)

- **Tech-Agnostic ≠ Model-Agnostic:**  
   Interface with LLMs/models via PromptDriver, embed via EmbeddingDriver,
  memory with VectorStoreDriver—swap providers with no prompt logic changes

- **Data Handling:**  
   Keep large or sensitive data “off-prompt” using TaskMemory and Off‑Prompt™
  flag in tools

- **Prompting:**  
   Use Python-native prompts instead of raw strings; rely on RulesetDriver for
  guided LLM behavior

Build with Agents, Pipelines, Workflows → combine small, single-purpose Tasks
and Tools

Tech-Agnostic ≠ Model-Agnostic

Interface with LLMs/models via PromptDriver, embed via EmbeddingDriver, memory
with VectorStoreDriver—swap providers with no prompt logic changes

Keep large or sensitive data “off-prompt” using TaskMemory and Off‑Prompt™ flag
in tools

Use Python-native prompts instead of raw strings; rely on RulesetDriver for
guided LLM behavior

**Structures**  
Agent: single Task; connects to tools, memory, drivers

Pipeline: sequential Task chaining

Workflow: DAG of parallel Tasks for efficiency

Enforce structured input/output schemas, activity-level tool permissions, and
persistent traceability

Task: atomic LLM or logic operation

Tool: encapsulates external capabilities (e.g., WebScraperTool, SQLTool)

Tools support off_prompt=True, structured input/output, and tooling metadata

**Drivers**  
PromptDrivers: LLM interface (OpenAI, Claude, local models)

EmbeddingDrivers: embedding backend (OpenAI, Cohere, HuggingFace)

VectorStoreDrivers: vector DB abstraction (Pinecone, others)

RulesetDrivers: enforce behavioral policies via structured Rules

✅ Best Coding Practices

**Module & Dependency Management**  
ConversationMemory: context across chat turns

TaskMemory: off‑prompt storage for heavy outputs

MetaMemory: enriched with metadata for contextual awareness

Avoid embedding raw prompts in code — use drivers or templates

**Tool Design**  
Name tools by domain (e.g., File:MarkdownWriterTool)

Use ToolInput, ToolOutput, raise ToolException

Log tool activity for telemetry & audit

Mark sensitive tools `off_prompt=True`

Secrets: AWS Secrets Manager or .env

Invoke async with Workflow.run_async() for parallelism

**Node Development (Griptape Nodes)**  
Use official node template

Single responsibility per node

**Error Handling / Resilience**  
Retry decoupled tasks with backoff

Implement fallback drivers (e.g., local llama when API fails)

Centralize exceptions with CloudWatch/EventBridge alerts

**Embeddings & RAG**  
Use domain‑specific embeddings when possible

Chunk large content via built-in chunkers (markdown, PDF, plain text)

🧪 Observability & MonitOring  
Use official node template

Single responsibility per node

Use metrics: task latency, retries, error rates

Log off-prompt memory hashes for traceability

⚠️ Anti‑Patterns to Avoid  
Anti-Pattern Risk Mitigation

Retry decoupled tasks with backoff

Implement fallback drivers (e.g., local llama when API fails)

Centralize exceptions with CloudWatch/EventBridge alerts

Pinecone vector DB and embedding driver configured

IAM roles per agent for memory and tool access

Base Agent, Pipeline, Workflow templates in place

Use domain‑specific embeddings when possible

Chunk large content via built-in chunkers (markdown, PDF, plain text)

🛠️ Next Steps  
Generate a sample multi-agent harald_pipeline.yml for HARALD → Stratia → Liora

Scaffold a starter repo with agents, tools, memory per-name, structured I/O

Write guided RuleSet JSON schemas to enforce compliance behavior

Emit structure run events → CloudWatch + EventBridge

Attach Rulesets for guardrails (RulesetDriver)

Use metrics: task latency, retries, error rates
