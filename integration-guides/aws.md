# AWS Well-Architected Considerations

**Version:** 0.0.1

---

## 1. Purpose

Design and deploy a multi-agent, identity-aware ambient intelligence system
(**HARALD**) on AWS, aligned with:

- **ISO/IEC 27001** (Information Security)
- **ISO/IEC 25010** (Software Quality)
- **ISO/IEC 42010** (System Architecture)
- **ISO/IEC 38505** (Data Governance)

**Stack priorities:**

- Federated identity (Google/Amazon)
- Agentic GenAI orchestration
- Offline-first capabilities
- Cost-aware execution
- Multi-account governance

---

## 2. System Overview

- **Stack Codename:** HARALDSTACK
- **Primary Region:** us-east-1
- **Multi-Account Strategy:** Purpose-based accounts (e.g., dev, prod, training,
  observability)

**Core Capabilities:**

- Modular agents (e.g., Gandalf, Athena, BoJack)
- Semantic memory (Pinecone)
- GenAI tool planning (Bedrock Agents)
- OIDC login via Google & Amazon
- Offline fallback with cached inference
- Observability, guardrails, and usage gating

---

## 3. Architecture Diagram (Textual)

```
User (Browser or Mobile)
    |
OIDC (Google, Amazon)
    |
IAM Identity Center (Federated Access)
    |
[App Load Balancer] -- Route53 + CloudFront
    |
[API Gateway (Private)]
    |
[Lambda Router Layer]
    |
    |-- [Bedrock Agent (per archetype)]
    |-- [Pinecone Vector Search (semantic memory)]
    |-- [DynamoDB (short-term cache)]
    |-- [Step Functions (agent workflows)]
    |-- [S3 + CloudWatch + EventBridge (logs, triggers)]
    |-- [Offline Inference Engine (local or on-device)]
```

---

## 4. Modules & Services

### 4.1 Identity & Access

- **IAM Identity Center:** OIDC federation with Amazon and Google
- **ABAC Policies:** Attribute-based access for agent permissions
- **Guardrails:** Session scope, role tagging, policy boundary enforcement

### 4.2 GenAI Core

- **Amazon Bedrock Agents:** Tool-planning, knowledge base access, action group
  orchestration
- **LlamaIndex + LangGraph (Hybrid):** Agentic RAG for external sources
- **Guardrails:** Content moderation, prompt filtering, token quota limits

### 4.3 Memory

- **Pinecone:** Long-term semantic memory
- **DynamoDB (TTL):** Short-term context & working memory
- **S3 Glacier:** Archived memory/state snapshots

### 4.4 Offline Support

- **Local SQLite or S3 Sync Cache**
- **LLM Execution:** (ONNX/PyTorch Mobile) with fallback templates for key agent
  behaviors
- **RAG subset prefetching:** via Lambda scheduler (EventBridge)

### 4.5 Multi-Agent Flow

- **AWS Step Functions:** Supervisory planning
- **EventBridge:** Reflex-driven agent scheduling
- **Lambda:** Stateless agent endpoints

### 4.6 Observability & Cost Controls

- **CloudWatch Logs + Metrics**
- **AWS Cost Anomaly Detection + Budgets**
- **Arize AI (via Bedrock):** Agent tracing and RAG quality scores

### 4.7 Multi-Account Control

- **Organizations & SCPs:** Service Control Policies
- **CodePipeline + Terraform/CDK:** Standardized rollout
- **Audit Manager:** GenAI Framework v2 for ISO alignment

---

## 5. Data Classification & Governance

- **Public Interactions:** Sanitized, anonymized, ephemeral
- **Private Agent States:** Encrypted with KMS + expiration metadata
- **PII Flow:** Federated identity tokens are never persisted
- **Inference Logs:** Redacted in transit; stored encrypted at rest

---

## 6. Compliance Mapping

| ISO/IEC Standard | Control         | HARALDSTACK Implementation              |
| ---------------- | --------------- | --------------------------------------- |
| 27001            | A.9, A.10       | IAM Identity Center, OIDC, Guardrails   |
| 25010            | Maintainability | Modular agents + IaC via Terraform      |
| 42010            | Architecture    | Layered cloud-native service model      |
| 38505            | Data Governance | Cost-aware exec, PII policy, audit logs |

---

## 7. Known Risks & Future Additions

- **Pending:** Bedrock multi-agent collaboration integration
- **Planned:** Real-time vector updates from WebSocket input
- **Research:** Whisper/Suno-style local audio preproc on edge

---

## 8. Versioning

- **0.0.1:** Baseline structure and security/compliance scaffolding
- **Next:** Implement IaC modules, offline inference harness, cost alert guards

---

## 9. Maintainers

- Bryan Chasko (Owner, Architect)

---

## References

Authoritative AWS references for understanding best practices in building
Generative AI systems:

1. **Best Practices for Building Generative AI Applications on AWS**  
   [Read](https://aws.amazon.com/blogs/machine-learning/best-practices-to-build-generative-ai-applications-on-aws/)

2. **Best Practices for Building Robust GenAI Applications with Amazon Bedrock
   Agents (Part 1)**  
   [Read](https://aws.amazon.com/blogs/machine-learning/best-practices-for-building-robust-generative-ai-applications-with-amazon-bedrock-agents-part-1/)

3. **Best Practices for Building Robust GenAI Applications with Amazon Bedrock
   Agents (Part 2)**  
   [Read](https://aws.amazon.com/blogs/machine-learning/best-practices-for-building-robust-generative-ai-applications-with-amazon-bedrock-agents-part-2/)

4. **AWS Audit Manager Framework: Generative AI Best Practices (v2)**  
   [Read](https://docs.aws.amazon.com/audit-manager/latest/userguide/aws-generative-ai-best-practices.html)

5. **Architectural Guidelines & Best Practices for Amazon Bedrock**  
   [Read](https://www.linkedin.com/pulse/architectural-guidelines-best-practices-aws-bedrock-choo-yang-tan-ilgqc)
