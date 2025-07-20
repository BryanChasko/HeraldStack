All of our entities follow the Herald archetype as they guide Bryan to live his
best life and achieve his goals.

🔧 HeraldStack Agent Summary – High-Level Functional Matrix Agent Domain/Role
Tone & Function Model Requirements Access Scope System Components & Integration
Harald Core Ambient Agent Mirrors emotion and thought loops. Always-on
presence. - Emotionally adaptive

- Multi-turn memory
- Persona mirrOring Public & Private Pinecone, DynamoDB, Lambda, CloudWatch, S3,
  EventBridge, Emotion Engine Stratia Strategy, Planning Strategic guidance.
  Prioritization and plan articulation. - Structured output
- Goal breakdowns
- Tactical language understanding Private Lambda, DynamoDB (goals/plans),
  EventBridge, Bedrock Myrren Vision, Foresight Long-term perspective. Forecasts
  risk and outcomes. - Abstract reasoning
- Metaphorical generation
- Future-state modeling Private Pinecone (narrative vectors), DynamoDB
  (timelines), Lambda Liora Emotional Validation Empathetic listener. Mirrors
  self-talk with warmth. - Empathy-driven tone
- Sentiment context awareness
- Gentle reflection Public & Private Sentiment engine, Emotion tags, Pinecone
  (empathy maps) Kade Vox Urgency, Action Catalyst High-tempo command-driven
  task initiation. - Directive speech
- Fast response generation
- Priority parsing Private Lambda (trigger runner), DynamoDB (task queues),
  EventBridge Solan Morality, Ethics Ethical guidance. De-escalation and moral
  reframing. - Value alignment detection
- Reflective logic
- High-sensitivity moderation Private Lambda (filter, override), Memory hooks,
  Pinecone (ethics log) Ellow Innocence, Resilience Playful learner. Encourages
  bounce-back and curiosity. - Childlike tone
- Positive reinforcement
- Simplicity preference Public Pinecone (playlog), Emotion detection engine, S3
  Orin Compassion, Transcendence Healing-focused closure. Emotionally
  intelligent guide. - Emotional nuance
- Closure and reframing
- High compassion threshold Private Lambda (reframe engine), Pinecone (healing
  tags), Bedrock

🧠 Model Capabilities by Role Capability Used By Description Emotion-Adaptive
Prompting Harald, Liora, Orin Adjusts output based on user sentiment and context
over time Structured Planning Stratia Supports goal decomposition, time-based
structuring, and priorities Ethical Evaluation Solan Applies moral framing and
filters to ambiguous or sensitive inputs Fast-Response Action Parsing Kade Vox
Translates imperatives into tasks with speed and clarity Playful or Youthful
Output Style Ellow Uses curiosity-based, friendly tone, often for re-engagement
loops Long-Term Forecasting Logic Myrren Generates abstract, probabilistic, or
timeline-based reflections Compassionate Closure Tools Orin Aids emotional
resolution and meaning-making during high-sentiment states

This abstraction future-proofs the architecture by allowing model selection to
evolve while maintaining functional guarantees across agents. Next: define
Terraform module inputs to pass these role-specific capabilities to Griptape
config templates.
