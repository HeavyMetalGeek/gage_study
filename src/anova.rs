use crate::dataset::DataSet;

pub trait Anova {
    fn mean(&self) -> f64;
}

pub struct AnovaTable {
    pub dof_total: usize,
    pub dof_parts: usize,
    pub dof_operators: usize,
    pub dof_repeatability: usize,
    pub dof_part_operator: usize,
    pub sumsq_total: f64,
    pub sumsq_parts: f64,
    pub sumsq_operators: f64,
    pub sumsq_repeatability: f64,
    pub sumsq_part_operator: f64,
    pub meansq_parts: f64,
    pub meansq_operators: f64,
    pub meansq_repeatability: f64,
    pub meansq_part_operator: f64,
    pub f_parts: f64,
    pub f_operators: f64,
    pub f_part_operator: f64,
}

impl Default for AnovaTable {
    fn default() -> Self {
        Self::new()
    }
}

impl AnovaTable {
    pub fn new() -> Self {
        Self {
            dof_total: 0,
            dof_parts: 0,
            dof_operators: 0,
            dof_repeatability: 0,
            dof_part_operator: 0,
            sumsq_total: 0.0,
            sumsq_parts: 0.0,
            sumsq_operators: 0.0,
            sumsq_repeatability: 0.0,
            sumsq_part_operator: 0.0,
            meansq_parts: 0.0,
            meansq_operators: 0.0,
            meansq_repeatability: 0.0,
            meansq_part_operator: 0.0,
            f_parts: 0.0,
            f_operators: 0.0,
            f_part_operator: 0.0,
        }
    }
    pub fn from_data(dataset: &DataSet) -> Self {
        let mean =
            dataset.data.iter().map(|v| v.measured).sum::<f64>() as f64 / dataset.data.len() as f64;
        let p = dataset.parts.len();
        let q = dataset.operators.len();
        let n = dataset.replicates[0].values.len();
        for r in dataset.replicates.iter() {
            println!("{:?}", r);
        }
        let dof_total = (p * q * n) - 1;
        let dof_parts = p - 1;
        let dof_operators = q - 1;
        let dof_repeatability = p * q * (n - 1);
        let dof_part_operator = dof_parts * dof_operators;
        // Calculate sum of squared deviations
        let sumsq_total = dataset
            .data
            .iter()
            .map(|d| (d.measured - mean).powi(2))
            .sum();
        let sumsq_parts = (q * n) as f64
            * dataset
                .parts
                .iter()
                .map(|p| (p.mean() - mean).powi(2))
                .sum::<f64>();
        let sumsq_operators = (p * n) as f64
            * dataset
                .operators
                .iter()
                .map(|o| (o.mean() - mean).powi(2))
                .sum::<f64>();
        let sumsq_repeatability = match q {
            1 => sumsq_total - sumsq_parts - sumsq_operators,
            2..=usize::MAX => {
                let mut sum = 0.0;
                for replicate in dataset.replicates.iter() {
                    let r_mean = replicate.mean();
                    sum += replicate
                        .values
                        .iter()
                        .map(|v| (v - r_mean).powi(2))
                        .sum::<f64>();
                }
                sum
            }
            _ => panic!(
                "dof_part_operator is an invalid value: {}",
                dof_part_operator
            ),
        };
        println!(
            "{}, {}, {}, {}",
            sumsq_total, sumsq_operators, sumsq_parts, sumsq_repeatability
        );
        let sumsq_part_operator =
            sumsq_total - (sumsq_parts + sumsq_operators + sumsq_repeatability);
        // Calculate mean squared
        let meansq_parts = sumsq_parts / dof_parts as f64;
        let meansq_operators = sumsq_operators / dof_operators as f64;
        let meansq_repeatability = sumsq_repeatability / dof_repeatability as f64;
        let meansq_part_operator = sumsq_part_operator / dof_part_operator as f64;
        // Calculate F-Statistics
        let f_parts = meansq_parts / meansq_part_operator;
        let f_operators = meansq_operators / meansq_part_operator;
        let f_part_operator = meansq_part_operator / meansq_repeatability;

        Self {
            dof_total,
            dof_parts,
            dof_operators,
            dof_repeatability,
            dof_part_operator,
            sumsq_total,
            sumsq_parts,
            sumsq_operators,
            sumsq_repeatability,
            sumsq_part_operator,
            meansq_parts,
            meansq_operators,
            meansq_repeatability,
            meansq_part_operator,
            f_parts,
            f_operators,
            f_part_operator,
        }
    }
}
