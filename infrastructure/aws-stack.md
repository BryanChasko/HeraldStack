Character-Aware Personal Assistant Architecture – Bryan Chasko Deployment
Target: https://bryanchasko.com

/public → General AI assistant

/private → Secure dashboard + assistant with personal access

🧭 Purpose Deploy a multi-modal, emotionally expressive AI assistant with
semantic memory and character-driven guidance.

🔧 Architecture Overview Layer Service Purpose Hosting Amazon S3 + CloudFront
HTTPS static hosting with global CDN Domain Amazon Route 53 Domain management
for bryanchasko.com Agent Orchestration Griptape Handles tool routing, memory,
and reasoning Memory Pinecone (vector), DynamoDB (state) Contextual and
structured memory LLMs AWS Bedrock (Claude 3, Titan), OpenAI GPT-4o
Conditionally routed LLMs by access level Execution Layer AWS Lambda Secure tool
execution, embedding, API access Event Logging EventBridge + CloudWatch Tracks
queries, personas, tools IAM/Auth IAM, Cognito Role-based access (public vs.
private) CI/CD Terraform + GitHub Actions IaC and deployment automation

🧠 Assistant Personality Engine

Memory Vector Tags Injected via Griptape MemoryRetriever

Prompting Style:

🔐 Example Security & Access Model Role Access AuthN/AuthZ Public Claude 3
(read-only) Cognito unauthenticated Bryan (Admin) GPT-4o, tools, calendar,
health data Cognito + IAM role-based session IAM Role policies for Bedrock,
Lambda, Pinecone Scoped with Terraform

🛠 Terraform Resources Summary S3: Static site + asset blob storage

DynamoDB: Session logs, persona config

Pinecone: Vector embeddings for context

Lambda: assistantSearchExecutor, embed tools, private API bridge

EventBridge: Tracks LLM calls, persona swaps

IAM: Execution roles, scoped Bedrock & Pinecone access

🔁 Request Lifecycle

sequenceDiagram participant User participant Babylon.js participant Griptape
participant Pinecone participant DynamoDB participant Bedrock/GPT participant
Lambda participant EventBridge

User->>Babylon.js: Enter query Babylon.js->>Griptape: Send request
Griptape->>Pinecone: Retrieve memory Griptape->>DynamoDB: Load session
Griptape->>Bedrock/GPT: Construct + call LLM Bedrock/GPT->>Griptape: Return
response Griptape->>Lambda: Tool call (optional) Griptape->>Babylon.js: Output
response Griptape->>EventBridge: Log interaction

🔄 CI/CD Pipeline Frontend Build: npm run build (Vite)

Deploy via GitHub Actions:

aws s3 sync dist/ s3://entity-static-and-blob-storage aws cloudfront
create-invalidation --paths "/\*" Infra Provision: Terraform-managed, secrets
via SSM/Secrets Manager

🔭 Roadmap Goal Action Persona Memory Expansion Add Weaviate/Qdrant parallel
memory Private Tools Integrate Calendar API + FHIR/SMART Local Model Fallback
Enable Mistral for offline mode LoRA Persona Adapters Fine-tune entity personas
Multi-LLM Eval Ability to try different models and reference data on different
entities.

✅ Summary Component Public Access Private Access LLM Claude 3 (Bedrock) GPT-4o,
Claude 3, tools Frontend Babylon/Sumerian UI Same Memory Pinecone
(character-tagged) Same + health/context Auth Cognito (unauth) Cognito
(federated w/ IAM roles) Domain bryanchasko.com/public bryanchasko.com/private
