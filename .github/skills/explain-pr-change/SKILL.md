---
name: explain-pr-change
description: "Explain why a Pull Request reviewer, including Copilot, is requesting a change and what happens if it is ignored. Use only for PR review comments when the user wants the rationale or consequences explained."
argument-hint: "Describe the PR change request you want explained"
user-invocable: false
disable-model-invocation: false
---

# Explain PR Change

## Outcome

Explain in plain terms why a Pull Request reviewer is requesting a specific change and what would happen if the change is not made. Focus on the rationale and practical consequences.

## When to Use

Use this skill only when a PR review comment from Copilot or a human requests a change and the user wants to understand why or what happens if it is ignored.

Do not use it for general code explanation, walkthroughs, or debugging unrelated to a PR review.

## Procedure

1. Identify the exact change requested in the PR review.
2. Locate the relevant code in the diff and surrounding context.
3. Determine whether the concern is about correctness, maintainability, security, performance, or style.
4. Explain the causal chain: current behavior, triggering scenario, and why the proposed change addresses it.
5. Explain the consequences of ignoring the request and classify the severity as a blocker, warning, or suggestion.
6. State any tradeoff introduced by making the change.
7. Recommend accepting, discussing, or rejecting the request only when the code and review context provide enough evidence; otherwise state what remains uncertain and how to verify it.

## Decision Rules

- Prioritize why the change is requested and what happens if it is ignored.
- Do not speculate beyond the review comment and available code; label assumptions clearly.
- Say plainly when the consequence is a crash, data loss, security exposure, or other serious failure.
- If the request is only style or naming, say so explicitly.
- Preserve the user's requested level of detail.

## Quality Check

Before responding, verify that the explanation:

- States exactly what change is requested.
- Explains why the reviewer wants it.
- Explains what happens if it is ignored.
- Distinguishes blockers from suggestions.
- Separates observed facts from assumptions.
- Ends with a practical recommendation or verification step.
