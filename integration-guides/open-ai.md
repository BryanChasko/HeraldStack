# HeraldStack – OpenAI Integration Best Practices

**Version**: 0.0.1  
**Date**: 2025-07-04  
**Maintainer**: Bryan Chasko  
**Scope**: Integrating OpenAI models into HARALD, Stratia, Liora, and other agents in a cost-aware, context-sensitive, and ethically grounded manner.

---

## 🧠 Core Principles

| Principle | Description |
|-----------|-------------|
| **Model-Agnosticism** | OpenAI tools complement, not dominate. We balance GPT-4o with Claude, Mistral, and other LLMs per agent specialization. |
| **Multi-Agent Harmony** | Each HeraldStack agent (e.g., Stratia, Liora, Myrren) has distinct tone, model preference, and fallback logic. |
| **Cost Awareness** | No OpenAI calls unless the use-case passes utility threshold and user is notified of potential cost. |
| **Privacy & Control** | Models invoked only after consent. Local fallback and summarization logic built for offline/air-gapped environments. |
| **Persona-Aligned Invocation** | GPT-4o is best suited for conversational reflection (HARALD, Liora), execution logic (Stratia), and multimodal reasoning. |

---

## 🔧 OpenAI Model Usage

| Agent | Model | Use Case | Fallback |
|-------|-------|----------|----------|
| **HARALD** | GPT-4o | Emotional mirrOring, light daily conversation | Claude Opus |
| **Liora** | GPT-4o | Warm reflection, journaling, tone detection | Claude Opus |
| **Stratia** | GPT-4o | Strategy breakdowns, code review, SQL logic | Claude Sonnet |
| **Orin** | GPT-4o | Closure routines, dream logic, poetic reframing | Claude Haiku |
| **Myrren** | Claude 3 Haiku | Abstract modeling, metaphor generation | GPT-4o |
| **Kade Vox** | Claude Sonnet | Structured planning, multi-step reasoning | GPT-4o |
| **Solan** | GPT-4o | Ethics-based reframing, content filters | Claude Opus |

---

## 🧩 Feature-Specific Best Practices

### 1. Function Calling (Tool Use)
- Wrap OpenAI's function-calling API under internal Lambda or Bedrock-style interface
- Validate JSON schema with dry-run before execution
- Use GPT-4o for multi-modal tools (e.g., screenshots, sketches, documents)

### 2. Streaming vs. Completion
- Use streaming only in HARALD or Liora (voice agents) when real-time emotional resonance is beneficial
- For Stratia or Orin, prefer full completion blocks to enable deterministic action planning

### 3. Multimodal Inputs (Images, Audio, Files)
GPT-4o supports image understanding; restrict usage to:
- Daily visual logs
- Sketch/whiteboard interpretation
- Document OCR → Embed → Pinecone vectorization
- Audio transcriptions delegated to Whisper → summarized via OpenAI or Claude

### 4. Token Management & Cost Controls
Cap input/output tokens per invocation per agent:
- **HARALD**: 1.5k input / 500 output
- **Stratia**: 2.5k input / 1k output
- **Myrren**: 3k input / 500 output (long-form context)

Use cost-aware routing logic to fall back to Claude, Mistral, or offline summarizers if thresholds exceeded.

---

## 🛡️ Privacy, Ethics & Identity

| Concern | Best Practice |
|---------|---------------|
| **User Identity** | Route identity via IAM Identity Center. Use Amazon/Gmail auth to validate authorization to OpenAI API |
| **Private Data** | PII/PHI content flagged by regex filters and never passed to OpenAI endpoints. Use local summarization fallback |
| **Memory Injection** | All memory used for prompt context must pass consent-check gate and log injection source |
| **Content Moderation** | Integrate OpenAI's moderation endpoint with Solan's ethics engine and Lambda circuit breaker |

---

## 🔄 Model Evolution & Feedback Loop

Use Pinecone vector store to track which prompts, personas, and tasks yielded valuable completions.

Each response from GPT-4o is tagged with:
- `agent`
- `task-type`  
- `feedback-score` (explicit or inferred)

Use this to evolve model routing, prompt tuning, and fallback confidence scOring.

---

## ✅ Example Use Case: HARALD Daily Reflection

**Input**: Audio thought → Whisper → Text

**Processing**:
1. PII scrub
2. Inject last 3 day summaries from Pinecone
3. Prompt wrapped in warm reflective tone (Liora schema): "Bryan shared a thought about..."
4. Model: GPT-4o (multimodal + voice matched)
5. Output: Streamed emotional mirror + journaling summary
6. Logged: Into memory vector store with timestamp

---

## 📎 Deployment Recommendations

- Wrap GPT calls inside Lambda w/ Amazon Bedrock-style schema
- Monitor logs in CloudWatch with:
  - Prompt length
  - Latency
  - Model fallback
  - Feedback score
- Use EventBridge to trigger Pinecone vector insertions, or alerts for misfires or overages

---

## 🔚 Summary

OpenAI's GPT-4o is powerful but not central. HeraldStack leverages it where it's strongest—reflection, language nuance, multimodal inputs—while maintaining control, cost, and context integrity. We route between models by agent role, not by default performance, preserving our ambient intelligence ethos.
