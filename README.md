
### Fuzzy Logic Library

This library provides a robust framework for implementing fuzzy logic systems in Rust. It includes classes for defining fuzzy sets and rules, as well as an inference engine for evaluating those rules to determine priorities or other outcomes. The library supports various fuzzy set operations and provides methods for combining fuzzy sets, evaluating membership degrees, and calculating centroids.

For example, you can use this library to build a fuzzy logic-based priority assessment system for handling customer support tickets. In this system, fuzzy sets might represent urgency levels (such as "Urgent", "High Priority", "Medium Priority", "Low Priority"), and rules could evaluate ticket scores to determine the appropriate priority level. The inference engine will process these rules and aggregate the results to suggest the most suitable priority for each ticket. This allows you to handle tickets more effectively by automating priority determination based on fuzzy logic.

### Installation
Add the library to your Cargo.toml:
```rust
[dependencies]
rsfuzzymind = "0.1"
```

### Features

- **Fuzzy Sets**: Define fuzzy sets with membership functions and perform operations like union, intersection, complement, and normalization.
- **Fuzzy Rules**: Create and evaluate rules that map conditions to fuzzy sets with associated weights.
- **Inference Engine**: Aggregate rule evaluations to infer priority levels based on weighted results.

### Components

#### `FuzzySet`

Represents a fuzzy set with a name and a membership function. Key methods include:

- `new(name: &str, membership_function: Arc<dyn Fn(f64) -> f64 + Send + Sync>) -> Self`
- `membership_degree(x: f64) -> f64`
- `union(&self, other: &FuzzySet) -> FuzzySet`
- `intersection(&self, other: &FuzzySet) -> FuzzySet`
- `complement(&self) -> FuzzySet`
- `normalize(&self) -> FuzzySet`
- `centroid(min_val: f64, max_val: f64, step: f64) -> f64`

#### `FuzzyRule`

Represents a rule with a condition, consequence fuzzy set, and weight. Key methods include:

- `new(condition: Box<dyn Fn(f64) -> bool + Send + Sync>, consequence: FuzzySet, weight: f64) -> Self`
- `evaluate(input: f64) -> Option<(FuzzySet, f64)>`

#### `InferenceEngine`

Aggregates results from multiple `FuzzyRule` instances to infer a priority level. Key methods include:

- `new(rules: Vec<FuzzyRule>) -> Self`
- `infer(input: f64) -> String`

##### Example Usage

Here's a basic example demonstrating how to set up and use the library to assess ticket priorities:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use rsfuzzymind::{FuzzySet, FuzzyRule, InferenceEngine};

fn create_urgency_set() -> FuzzySet {
    FuzzySet::new(
        "Urgent",
        Arc::new(|x| if x > 0.8 { 1.0 } else { 0.0 }),
    )
}

fn create_complexity_set() -> FuzzySet {
    FuzzySet::new(
        "Complex",
        Arc::new(|x| if x > 0.5 { 1.0 } else { 0.0 }),
    )
}

fn create_priority_rules() -> Vec<FuzzyRule> {
    vec![
        FuzzyRule::new(
            Box::new(|x| x > 0.9),
            create_urgency_set(),
            1.0,
        ),
    ]
}

fn create_inference_engine() -> InferenceEngine {
    InferenceEngine::new(create_priority_rules())
}

fn infer_ticket_priority(engine: &InferenceEngine, ticket: &HashMap<String, f64>) -> String {
    let score = ticket.get("score").cloned().unwrap_or(0.0);
    engine.infer(score)
}

fn main() {
    let engine = create_inference_engine();
    let mut ticket = HashMap::new();
    ticket.insert("score".to_string(), 0.95);
    let priority = infer_ticket_priority(&engine, &ticket);
    println!("Ticket Priority: {}", priority);
}
```

##### License
This library is licensed under the MIT License.
