```markdown
# Best Practices Guide for Multi‑Agent Voice Experiences (Amazon VII)

This guide covers best practices for designing multi‑agent voice experiences
under Amazon’s Voice Interoperability Initiative (VII), integrating official
guidance, architecture recommendations, and real-world demos.

## 1. Customer Choice & Invocation

**Enable Multiple Wake Words**  
Devices should always allow simultaneous wake-word support for each agent,
except when one agent is actively processing audio—then other agents are
inactive.

**Distinct Invocation**  
Use unique, easily distinguishable wake words to prevent false activations.

**Button Invocation Preference**  
If a universal “action” button is available, allow users to configure which
agent it activates by default.

## 2. Discovery & Agent Awareness

**Visible Agent Cues**  
Each active agent must clearly indicate its attention state—listening, thinking,
speaking—via sound, colors, or visuals.

**Educative Onboarding**  
Integrate tutorials or quick start guides in-device or app-based to help users
discover available agents and their capabilities.

## 3. Smooth Agent Transfer

**Agent Arbitration & Transfer**  
If Agent A can't fulfill a request, it should invoke Agent B via a silent
handoff—without sharing private user context—and hand over the user to Agent B
seamlessly.

**Support Universal Device Commands (UDC)**  
Essential device functions (e.g., volume, alarms, lights) must be operable via
any agent, regardless of which one is in use.

## 4. Dialogue Management & Turn‑Taking

**Single‑Agent Focus**  
Enforce strictly that only one agent listens at a time; when one agent is
streaming audio, suppress detection of other wake words.

**Interruptibility**  
While an agent speaks, allow users to barge-in with another agent’s wake word to
take the floor.

**Agent Arbitrator Component**  
Implement device-level arbitration—like an attention manager—to coordinate agent
activation and manage transitions.

## 5. Privacy & Security

**Transparency & Consent**  
Clearly disclose when agents share data; obtain explicit user consent for
cross-agent transfers.

**Secure Isolation**  
Prevent agents from triggering each other (e.g., one agent’s TTS must not
pronounce another agent’s wake word).

## 6. UX & Interaction Design

**Consistent Attention States**  
Provide consistent multi-modal feedback (lights, sounds, voice) for each agent’s
listening/thinking/speaking state.

**Cognitive Simplicity**  
Use familiar conversational flows and avoid burdening users with interaction
rules or system jargon.

## 7. Architecture & SDK Integration

**Use VII Design Guide & Whitepapers**  
Base your implementation on Amazon’s Multi-Agent Design Guide and Architecture
Best Practices whitepapers.

**Leverage MAX Toolkit**  
The Multi‑Agent Experience (MAX) Toolkit provides sample agents, libraries, and
APIs to accelerate integrations.

**Implement UDC APIs**  
Build or adopt a universal command layer for common device actions per the UDC
architectural patterns.

## 8. Testing & Validation

**Simulate Multi-Agent Scenarios**  
Test interactions for wake-word collisions, barge-ins, correct transfers, and
UDC execution.

**Privacy Audit**  
Verify no cross-agent voice data or context leaks occur without user knowledge.

**Usability Testing**  
Conduct user tests to ensure users can reliably identify, switch, or mix agents
with no confusion.
```
