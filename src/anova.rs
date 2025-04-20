use crate::dataset::DataSet;
use std::fmt;
use log::debug;

#[derive(Debug, Default)]
/// Constructs all data needed for the ANOVA table
pub struct Anova {
    /// Number of parts: $p$
    pub n_parts: usize,
    /// Number of operators: $q$
    pub n_operators: usize,
    /// Number of replicates: $n$
    pub n_replicates: usize,
    /// Total degrees of freedom: $\nu\_T = N - 1$, where $N$ is the number of measurements
    pub dof_total: usize,
    /// [Part](crate::part::Part) degrees of freedom: $\nu\_p = p - 1$, where $p$ is the number of
    /// parts
    pub dof_parts: usize,
    /// [Operator](crate::operator::Operator) degrees of freedom: $\nu\_q = q - 1$, where $q$ is
    /// the number of operators
    pub dof_operators: usize,
    /// Repeatability degrees of freedom: $\nu\_R = pq(n - 1)$, where $n$ is the number of
    /// replicates.  If the part-operator term is neglected, $\nu\_R = \nu\_T - \nu\_p - \nu\_q$.
    pub dof_repeatability: usize,
    /// Part-operator interaction degrees of freedom: $\nu\_{p \cdot R} = \nu\_p \* \nu\_R$
    pub dof_part_operator: usize,
    /// Sum of squared differences between every value and the grand mean
    /// $$
    ///     SS\_T = \sum\_{i=1}^{p} \sum\_{j=1}^{q} \sum\_{k=1}^{n} \left(x\_{ijk} - \bar{x}\_{...}
    ///     \right)^2
    /// $$
    pub sumsq_total: f64,
    /// Sum of squared differences between each part mean and the grand mean
    /// $$
    ///     SS\_p = qn \sum\_{i=1}^{p} \left( \bar{x}\_{i..} - \bar{x}\_{...} \right)^2
    /// $$
    pub sumsq_parts: f64,
    /// Sum of squared differences between each operator mean and the grand mean
    /// $$
    ///     SS\_q = pn \sum\_{j=1}^{q} \left( \bar{x}\_{.j.} - \bar{x}\_{...} \right)^2
    /// $$
    pub sumsq_operators: f64,
    /// Sum of squared differences between each replicate and the replicate mean
    /// $$
    ///     SS\_R = \sum\_{i=1}^{p} \sum\_{j=1}^{t} \sum\_{k=1}^{q} \left(x\_{ijk} - \bar{x}\_{ij.}
    ///     \right)^2
    /// $$
    /// If the part-operator term is neglected, $SS\_R = SS\_T - SS\_p - SS\_q$
    pub sumsq_repeatability: f64,
    /// Sum of squared differences for the part-operator interaction:
    /// $SS\_R = SS\_T - SS\_p - SS\_q$
    pub sumsq_part_operator: f64,
    /// Mean of the squared differences for parts: $MS\_p = \frac{SS\_p}{\nu\_p}$
    pub meansq_parts: f64,
    /// Mean of the squared differences for operators: $MS\_q = \frac{SS\_q}{\nu\_q}$
    pub meansq_operators: f64,
    /// Mean of the squared differences for repeatability: $MS\_R = \frac{SS\_R}{\nu\_R}$
    pub meansq_repeatability: f64,
    /// Mean of the squared differences for the part-operator interaction:
    /// $MS\_{p \cdot R} = \frac{SS\_{p \cdot R}}{\nu\_{p \cdot R}}$
    pub meansq_part_operator: f64,
    /// F-statistic for parts: $F\_p = \frac{MS\_p}{MS\_{p \cdot R}}$.  If the part-operator term
    /// is negelected $F\_p = \frac{MS\_p}{MS\_R}$.
    pub f_parts: f64,
    /// F-statistic for operators: $F\_q = \frac{MS\_q}{MS\_{p \cdot R}}$.  If the part-operator term
    /// is negelected $F\_q = \frac{MS\_q}{MS\_R}$.
    pub f_operators: f64,
    /// F-statistic for the part-operator interaction: $F\_q = \frac{MS\_q}{MS\_R}$
    pub f_part_operator: f64,
    /// Flag for neglecting the part-operator interaction term
    pub use_interaction: bool,
}

impl fmt::Display for Anova {
    /// Defines the format for displaying the ANOVA table
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut source = "\n*** ANOVA ***\n".to_owned();
        let anova_header = format!(
            "{:<15}  {:>3}  {:>9}  {:>9}  {:>9}\n",
            "Source", "DF", "SS", "MS", "F"
        );
        source.push_str(&anova_header);
        let part = format!(
            "{:<15}  {:>3}  {:>9.5}  {:>9.5}  {:>9.5}\n",
            "Part", self.dof_parts, self.sumsq_parts, self.meansq_parts, self.f_parts
        );
        source.push_str(&part);
        let operator = format!(
            "{:<15}  {:>3}  {:>9.5}  {:>9.5}  {:>9.5}\n",
            "Operator",
            self.dof_operators,
            self.sumsq_operators,
            self.meansq_operators,
            self.f_operators
        );
        source.push_str(&operator);
        let interaction = match self.use_interaction {
            true => format!(
                "{:<15}  {:>3}  {:>9.5}  {:>9.5}  {:>9.5}\n",
                "Part * Operator",
                self.dof_part_operator,
                self.sumsq_part_operator,
                self.meansq_part_operator,
                self.f_part_operator
            ),
            false => "".to_owned(),
        };
        source.push_str(&interaction);
        let repeatability = format!(
            "{:<15}  {:>3}  {:>9.5}  {:>9.5}\n",
            "Repeatability",
            self.dof_repeatability,
            self.sumsq_repeatability,
            self.meansq_repeatability
        );
        source.push_str(&repeatability);
        let total = format!(
            "{:<15}  {:>3}  {:>9.5}\n",
            "Total", self.dof_total, self.sumsq_total
        );
        source.push_str(&total);
        write!(f, "{}", source)
    }
}

impl Anova {
    /// Makes a new AnovaTable
    pub fn new() -> Self {
        Self {
            use_interaction: true,
            ..Default::default()
        }
    }
    /// Performs all necessary ANOVA calculations using data from a [DataSet] and populates the
    /// AnovaTable fields
    pub fn from_data(dataset: &DataSet) -> Self {
        let mean = dataset.data.iter().map(|v| v.measured).sum::<f64>() / dataset.data.len() as f64;
        debug!(target: "anova", "mean: {mean}");
        let n_parts = dataset.parts.len();
        debug!(target: "anova", "# Parts: {n_parts}");
        let n_operators = dataset.operators.len();
        debug!(target: "anova", "# Operators: {n_operators}");
        let n_replicates = dataset.replicates[0].values.len();
        debug!(target: "anova", "# Replicates: {n_replicates}");
        let dof_total = (n_parts * n_operators * n_replicates) - 1;
        debug!(target: "anova", "DOF Total: {dof_total}");
        let dof_parts = n_parts - 1;
        debug!(target: "anova", "DOF Parts: {dof_parts}");
        let dof_operators = n_operators - 1;
        debug!(target: "anova", "DOF Operators: {dof_operators}");
        let dof_part_operator = match (dataset.use_interaction, n_operators) {
            (true, 2..) => dof_parts * dof_operators,
            (_, _) => 0,
        };
        debug!(target: "anova", "DOF Part-Operator: {dof_part_operator}");
        let dof_repeatability = match (dataset.use_interaction, n_operators) {
            (true, 2..) => n_parts * n_operators * (n_replicates - 1),
            (_, _) => dof_total - dof_parts - dof_operators,
        };
        debug!(target: "anova", "DOF Repeatability: {dof_repeatability}");
        // Calculate sum of squared deviations
        let sumsq_total = dataset
            .data
            .iter()
            .map(|d| (d.measured - mean).powi(2))
            .sum();
        debug!(target: "anova", "Sum Sq Total: {sumsq_total}");
        let sumsq_parts = (n_operators * n_replicates) as f64
            * dataset.parts.iter().map(|p| p.sqdiff(mean)).sum::<f64>();
        debug!(target: "anova", "Sum Sq Parts: {sumsq_parts}");
        let sumsq_operators = (n_parts * n_replicates) as f64
            * dataset
                .operators
                .iter()
                .map(|o| o.sqdiff(mean))
                .sum::<f64>();
        debug!(target: "anova", "Sum Sq Operators: {sumsq_operators}");

        let sumsq_repeatability = match (dataset.use_interaction, n_operators) {
            (true, 2..) => dataset.replicates.iter().map(|r| r.sqdiff()).sum(),
            (_, _) => sumsq_total - sumsq_parts - sumsq_operators,
        };
        debug!(target: "anova", "Sum Sq Repeatability: {sumsq_repeatability}");
        let sumsq_part_operator = match (dataset.use_interaction, n_operators) {
            (true, 2..) => {
                sumsq_total - (sumsq_parts + sumsq_operators + sumsq_repeatability)
            }
            (_, _) => 0.0,
        };
        debug!(target: "anova", "Sum Sq Part-Operator: {sumsq_part_operator}");
        // Calculate mean squared
        let meansq_parts = sumsq_parts / dof_parts as f64;
        debug!(target: "anova", "Mean Sq Parts: {meansq_parts}");
        let meansq_operators = match n_operators {
            0 | 1 => 0.0,
            _ => sumsq_operators / dof_operators as f64,
        };
        debug!(target: "anova", "Mean Sq Operators: {meansq_operators}");
        let meansq_repeatability = sumsq_repeatability / dof_repeatability as f64;
        debug!(target: "anova", "Mean Sq Repeatability: {meansq_repeatability}");
        let meansq_part_operator = match (dataset.use_interaction, n_operators) {
            (true, 2..) => sumsq_part_operator / dof_part_operator as f64,
            (_, _) => 0.0,
        };
        debug!(target: "anova", "Mean Sq Part-Operator: {meansq_part_operator}");
        // Calculate F-Statistics
        let f_parts = match (dataset.use_interaction, n_operators) {
            (true, 2..) => meansq_parts / meansq_part_operator,
            (_, _) => meansq_parts / meansq_repeatability,
        };
        debug!(target: "anova", "F-value Parts: {f_parts}");
        let f_operators = match (dataset.use_interaction, n_operators) {
            (true, 2..) => meansq_operators / meansq_part_operator,
            (_, _) => meansq_operators / meansq_repeatability,
        };
        debug!(target: "anova", "F-value Operators: {f_parts}");
        let f_part_operator = match (dataset.use_interaction, n_operators) {
            (true, 2..) => meansq_part_operator / meansq_repeatability,
            (_, _) => 0.0,
        };
        debug!(target: "anova", "F-value Part-Operator: {f_part_operator}");

        Self {
            n_parts,
            n_operators,
            n_replicates,
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
            use_interaction: dataset.use_interaction && n_operators > 1,
        }
    }
}
