```markdown
# 📘 General Best Practices: LangChain & LlamaIndex (Agentic Orchestration)

## 🔧 LangChain: Best Practices

LangChain is a framework for chaining together LLM calls with external tools,
memory, routing logic, and conditional flows.

### 1. Modularity & Composability

- **Use Chains over Monoliths:** Compose chains from atomic steps — prompt
  templates, LLM calls, tools, memory modules.
- **Prefer Declarative Routing:** Implement `MultiPromptChain`, `RouterChain`,
  or LCEL (LangChain Expression Language) for control flow over imperative
  conditionals.
- **Tool Wrapping:** Always wrap tools in `Tool` class objects with metadata
  (name, description, input schema) for reusability.

### 2. LLM Abstraction

- Use `ChatOpenAI`, `ChatAnthropic`, `ChatBedrock`, or `ChatLiteLLM` wrappers
  for provider abstraction.
- Avoid hard-coding model names or versions — make them configurable.
- Normalize I/O formats across providers.

### 3. Memory & State

- Use `ConversationBufferMemory`, `SummaryMemory`, or
  `VectorStoreRetrieverMemory` depending on:
  - Length of context required
  - Temporal relevance
  - Need for long-term storage
- Separate scratchpad (intermediate steps) from memory (user-facing history).

### 4. Agent Patterns

- Use `ReAct` or `ChatAgent` for tool-augmented reasoning.
- Avoid using `AgentExecutor` in production unless you’ve fully constrained
  input/output validation.
- Define tool-selection constraints to avoid misuse (e.g., searching when
  calculating is cheaper).

### 5. Observability

- Capture intermediate steps via callbacks (`StdOutCallbackHandler`, `Tracer`,
  `LangSmith`) for debugging and audit.
- Structure logs with stepwise execution trees, token usage, and LLM outputs.

**References:**  
[LangChain Docs](https://python.langchain.com/docs/) |
[LangGraph](https://github.com/langchain-ai/langgraph)
```
