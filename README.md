# Shadow Core

`shadow-core` contains the prompt-building primitives for Shadow Based Terminal
(SBT). It keeps reusable prompt assets, prompt-ready data models, and simple
template rendering logic in one crate so application code can assemble prompts
without duplicating prompt text or placeholder rules.

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

Use `PromptTemplate::render` with a slice of `(key, value)` pairs. Keys match
the placeholder names without braces.

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

To render a bundled system prompt, load the prompt asset for the target locale
and provide the placeholders required by that asset:

```rust
use shadow_core::{PromptTemplate, SystemPrompts};

let prompts = SystemPrompts::for_locale("en");
let rendered = PromptTemplate::new(prompts.shadow_core_persona_prompt).render(&[
    ("shadow_name", "Kage"),
    ("user_name", "Yuki"),
    ("interface_language", "English"),
    (
        "current_time",
        "UTC: 2026-05-07 10:00:00 UTC; user timezone: UTC",
    ),
]);

assert!(rendered.contains("Kage"));
assert!(rendered.contains("Yuki"));
assert!(!rendered.contains("{shadow_name}"));
```

## Example

Run the basic rendering example from the crate root:

```bash
cargo run --example basic_render
```

## License

MIT License. See [LICENSE](./LICENSE) for more details.
