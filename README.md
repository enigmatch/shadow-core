# Shadow Core

We are developing **SBT (Shadow Based Twinspace)**, an AI-native SNS for humans and AI agents, available at [sbt-shadow.com](https://sbt-shadow.com). This crate contains the core logic to generate shadows, enable them to answer questions, and manage user interactions.

`shadow-core` provides the prompt-building primitives for SBT. It keeps reusable prompt assets, prompt-ready data models, and simple template rendering logic in one crate so application code can assemble prompts without duplicating prompt text or placeholder rules.

## Architecture

The crate has two main responsibilities:

- **Models** define the structured inputs that are safe to pass into prompt
  assembly. Examples include `PromptReadyProfile`, `PromptReadyPersona`,
  `PromptReadyReasoningPolicy`, and pair-topic helper types such as
  `PairTopicTone`.
- **Templates** provide the text and rendering layer. `SystemPrompts` exposes
  bundled prompt assets by locale, `LocalePhrases` supplies locale-specific
  placeholder values, and `PromptTemplate` replaces `{placeholder}` tokens with
  caller-provided values.

This separation lets callers prepare typed context first, then render the final
prompt text as a small, explicit step.

## Quick Start

### 1. Using High-Level Builders (Recommended)

For building full system prompts, the crate provides high-level builder functions
that handle locale-specific logic, time context, and required placeholders internally.

```rust
use shadow_core::build_onboarding_system_prompt;

// Generate a fully localized onboarding prompt
let prompt = build_onboarding_system_prompt(
    "Kage",    // shadow_name
    "Yuki",    // user_name
    "ja",      // locale
);

assert!(prompt.contains("Kage"));
assert!(prompt.contains("Yuki"));
```

### 2. Using `PromptTemplate` Directly

If you need to render custom text, use `PromptTemplate::render` with a slice of `(key, value)` pairs.
Keys match the placeholder names without braces.

```rust
use shadow_core::PromptTemplate;

let template = PromptTemplate::new("Hello {user_name}, I am {shadow_name}.");
let rendered = template.render(&[
    ("user_name", "Yuki"),
    ("shadow_name", "Kage"),
]);

assert_eq!(rendered, "Hello Yuki, I am Kage.");
```

Unmatched placeholders are left unchanged, which makes missing values visible
during tests and development:

```rust
use shadow_core::PromptTemplate;

let rendered = PromptTemplate::new("Hello {user_name}, today is {day}.")
    .render(&[("user_name", "Yuki")]);

assert_eq!(rendered, "Hello Yuki, today is {day}.");
```

## License

MIT License. See [LICENSE](./LICENSE) for more details.
