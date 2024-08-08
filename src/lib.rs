use std::collections::HashMap;

pub struct FuzzySet {
    pub name: String,
    membership_function: Box<dyn Fn(f64) -> f64>,
}

impl FuzzySet {
    pub fn new(name: &str, membership_function: Box<dyn Fn(f64) -> f64>) -> Self {
        FuzzySet {
            name: name.to_string(),
            membership_function,
        }
    }

    pub fn membership_degree(&self, x: f64) -> f64 {
        (self.membership_function)(x)
    }

    pub fn union(&self, other_set: &FuzzySet) -> FuzzySet {
        FuzzySet::new(
            &format!("Union({}, {})", self.name, other_set.name),
            Box::new(move |x| f64::max((self.membership_function)(x), (other_set.membership_function)(x))),
        )
    }

    pub fn intersection(&self, other_set: &FuzzySet) -> FuzzySet {
        FuzzySet::new(
            &format!("Intersection({}, {})", self.name, other_set.name),
            Box::new(move |x| f64::min((self.membership_function)(x), (other_set.membership_function)(x))),
        )
    }

    pub fn complement(&self) -> FuzzySet {
        FuzzySet::new(
            &format!("Complement({})", self.name),
            Box::new(move |x| 1.0 - (self.membership_function)(x)),
        )
    }

    pub fn normalize(&self) -> FuzzySet {
        FuzzySet::new(
            &format!("Normalized({})", self.name),
            Box::new(move |x| {
                let value = (self.membership_function)(x);
                value / f64::max(1.0, value)
            }),
        )
    }

    pub fn centroid(&self, min_val: f64, max_val: f64, step: f64) -> f64 {
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        let mut x = min_val;
        while x <= max_val {
            let mu = (self.membership_function)(x);
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
    pub condition: Box<dyn Fn(&HashMap<String, f64>) -> bool>,
    pub consequence: Box<dyn Fn(&HashMap<String, f64>) -> Result<String, FuzzySet>>,
    pub weight: f64,
}

impl FuzzyRule {
    pub fn new(
        condition: Box<dyn Fn(&HashMap<String, f64>) -> bool>,
        consequence: Box<dyn Fn(&HashMap<String, f64>) -> Result<String, FuzzySet>>,
        weight: f64,
    ) -> Self {
        FuzzyRule {
            condition,
            consequence,
            weight,
        }
    }

    pub fn evaluate(&self, inputs: &HashMap<String, f64>) -> Option<(Result<String, FuzzySet>, f64)> {
        if (self.condition)(inputs) {
            Some(((self.consequence)(inputs), self.weight))
        } else {
            None
        }
    }
}

pub struct InferenceEngine {
    pub rules: Vec<FuzzyRule>,
}

impl InferenceEngine {
    pub fn new(rules: Vec<FuzzyRule>) -> Self {
        InferenceEngine { rules }
    }

    pub fn infer(&self, inputs: &HashMap<String, f64>) -> String {
        let mut results = vec![];
        for rule in &self.rules {
            if let Some((result, weight)) = rule.evaluate(inputs) {
                results.push((result, weight));
            }
        }
        self.aggregate_results(results)
    }

    fn aggregate_results(&self, results: Vec<(Result<String, FuzzySet>, f64)>) -> String {
        if results.is_empty() {
            return "Low Priority".to_string();
        }

        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;
        for (result, weight) in results {
            let priority_value = match result {
                Ok(priority) => self.priority_mapping(&priority),
                Err(_) => 0.0,
            };
            weighted_sum += priority_value * weight;
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
        if score >= 2.5 {
            "Urgent".to_string()
        } else if score >= 1.5 {
            "High Priority".to_string()
        } else if score >= 0.5 {
            "Medium Priority".to_string()
        } else {
            "Low Priority".to_string()
        }
    }

    fn get_fuzzy_set_consequences(&self) -> Vec<FuzzySet> {
        self.rules
            .iter()
            .filter_map(|rule| {
                if let Err(fuzzy_set) = (rule.consequence)(&HashMap::new()) {
                    Some(fuzzy_set)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn defuzzify_centroid(&self, min_val: f64, max_val: f64, step: f64) -> f64 {
        let fuzzy_sets = self.get_fuzzy_set_consequences();
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        let mut x = min_val;
        while x <= max_val {
            let mu = fuzzy_sets.iter()
                .map(|fs| fs.membership_degree(x))
                .fold(0.0, f64::max);
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

    pub fn defuzzify_mom(&self, min_val: f64, max_val: f64, step: f64) -> f64 {
        let fuzzy_sets = self.get_fuzzy_set_consequences();
        let mut max_mu = 0.0;
        let mut sum_x = 0.0;
        let mut count = 0.0;
        let mut x = min_val;
        while x <= max_val {
            let mu = fuzzy_sets.iter()
                .map(|fs| fs.membership_degree(x))
                .fold(0.0, f64::max);
            if mu > max_mu {
                max_mu = mu;
                sum_x = x;
                count = 1.0;
            } else if mu == max_mu {
                sum_x += x;
                count += 1.0;
            }
            x += step;
        }
        if count == 0.0 {
            0.0
        } else {
            sum_x / count
        }
    }

    pub fn defuzzify_bisector(&self, min_val: f64, max_val: f64, step: f64) -> f64 {
        let fuzzy_sets = self.get_fuzzy_set_consequences();
        let mut total_area = 0.0;
        let mut left_area = 0.0;
        let mut bisector = min_val;
        let mut x = min_val;
        while x <= max_val {
            let mu = fuzzy_sets.iter()
                .map(|fs| fs.membership_degree(x))
                .fold(0.0, f64::max);
            total_area += mu * step;
            x += step;
        }
        x = min_val;
        while x <= max_val {
            let mu = fuzzy_sets.iter()
                .map(|fs| fs.membership_degree(x))
                .fold(0.0, f64::max);
            left_area += mu * step;
            if left_area >= total_area / 2.0 {
                bisector = x;
                break;
            }
            x += step;
        }
        bisector
    }
}


