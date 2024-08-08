use std::sync::Arc;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct FuzzySet {
    name: String,
    membership_function: Arc<dyn Fn(f64) -> f64 + Send + Sync>,
}

impl FuzzySet {
    pub fn new(name: &str, membership_function: Arc<dyn Fn(f64) -> f64 + Send + Sync>) -> Self {
        FuzzySet {
            name: name.to_string(),
            membership_function,
        }
    }

    pub fn membership_degree(&self, x: f64) -> f64 {
        (self.membership_function)(x)
    }

    pub fn union(&self, other: &FuzzySet) -> FuzzySet {
        let self_func = Arc::clone(&self.membership_function);
        let other_func = Arc::clone(&other.membership_function);

        FuzzySet::new(
            &format!("Union({}, {})", self.name, other.name),
            Arc::new(move |x| f64::max(self_func(x), other_func(x))),
        )
    }

    pub fn intersection(&self, other: &FuzzySet) -> FuzzySet {
        let self_func = Arc::clone(&self.membership_function);
        let other_func = Arc::clone(&other.membership_function);

        FuzzySet::new(
            &format!("Intersection({}, {})", self.name, other.name),
            Arc::new(move |x| f64::min(self_func(x), other_func(x))),
        )
    }

    pub fn complement(&self) -> FuzzySet {
        let self_func = Arc::clone(&self.membership_function);

        FuzzySet::new(
            &format!("Complement({})", self.name),
            Arc::new(move |x| 1.0 - self_func(x)),
        )
    }

    pub fn normalize(&self) -> FuzzySet {
        let self_func = Arc::clone(&self.membership_function);

        FuzzySet::new(
            &format!("Normalized({})", self.name),
            Arc::new(move |x| self_func(x) / f64::max(1.0, self_func(x))),
        )
    }

    pub fn centroid(&self, min_val: f64, max_val: f64, step: f64) -> f64 {
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        let mut x = min_val;

        while x <= max_val {
            let mu = self.membership_degree(x);
            numerator += x * mu;
            denominator += mu;
            x += step;
        }

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }
}

pub struct FuzzyRule {
    condition: Box<dyn Fn(f64) -> bool + Send + Sync>,
    consequence: FuzzySet,
    weight: f64,
}

impl FuzzyRule {
    pub fn new(
        condition: Box<dyn Fn(f64) -> bool + Send + Sync>,
        consequence: FuzzySet,
        weight: f64,
    ) -> Self {
        FuzzyRule {
            condition,
            consequence,
            weight,
        }
    }

    pub fn evaluate(&self, input: f64) -> Option<(FuzzySet, f64)> {
        if (self.condition)(input) {
            Some((self.consequence.clone(), self.weight))
        } else {
            None
        }
    }
}

pub struct InferenceEngine {
    rules: Vec<FuzzyRule>,
}

impl InferenceEngine {
    pub fn new(rules: Vec<FuzzyRule>) -> Self {
        InferenceEngine { rules }
    }

    pub fn infer(&self, input: f64) -> String {
        let results: Vec<(FuzzySet, f64)> = self
            .rules
            .iter()
            .filter_map(|rule| rule.evaluate(input))
            .collect();

        self.aggregate_results(&results)
    }

    fn aggregate_results(&self, results: &[(FuzzySet, f64)]) -> String {
        if results.is_empty() {
            return "Low Priority".to_string();
        }

        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;

        for (result, weight) in results {
            weighted_sum += self.priority_mapping(&result.name) * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            self.reverse_priority_mapping(weighted_sum / total_weight)
        } else {
            "Low Priority".to_string()
        }
    }

    fn priority_mapping(&self, priority: &str) -> f64 {
        match priority {
            "Urgent" => 3.0,
            "High Priority" => 2.0,
            "Medium Priority" => 1.0,
            _ => 0.0,
        }
    }

    fn reverse_priority_mapping(&self, score: f64) -> String {
        match score.partial_cmp(&2.5) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => "Urgent".to_string(),
            _ => match score.partial_cmp(&1.5) {
                Some(Ordering::Greater) | Some(Ordering::Equal) => "High Priority".to_string(),
                _ => match score.partial_cmp(&0.5) {
                    Some(Ordering::Greater) | Some(Ordering::Equal) => "Medium Priority".to_string(),
                    _ => "Low Priority".to_string(),
                },
            },
        }
    }
}
