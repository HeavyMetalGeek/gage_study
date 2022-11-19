use crate::anova::AnovaTable;
use std::fmt;

pub struct VarCompTable {
    pub repeatability: f64,
    pub operator: f64,
    pub part_operator: f64,
    pub part_to_part: f64,
    pub reproducibility: f64,
    pub total_gagerr: f64,
    pub total_variation: f64,
    pub use_interaction: bool,
}

impl Default for VarCompTable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VarCompTable {
    /// Defines the format for displaying the variance components table
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut source = format!(
            "{:<17}  {:>9}  {:>5}\n{:<17}  {:>9}  {:>5}\n",
            "", "", "%Contribution", "Source", "VarComp", "(of VarComp)"
        );
        let part = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "Total Gage R&R",
            self.total_gagerr,
            self.total_gagerr / self.total_variation * 100.0
        );
        source.push_str(&part);
        let operator = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "  Repeatability",
            self.repeatability,
            self.repeatability / self.total_variation * 100.0
        );
        source.push_str(&operator);
        let interaction = match self.use_interaction {
            true => format!(
                "{:<17}  {:>9.7}  {:>6.2}\n",
                "  Reproducibility",
                self.reproducibility,
                self.reproducibility / self.total_variation * 100.0
            ),
            false => "".to_owned(),
        };
        source.push_str(&interaction);
        let repeatability = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "Part-to-Part",
            self.part_to_part,
            self.part_to_part / self.total_variation * 100.0
        );
        source.push_str(&repeatability);
        let total = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "Total Variation", self.total_variation, 100.00
        );
        source.push_str(&total);
        write!(f, "{}", source)
    }
}

impl VarCompTable {
    pub fn new() -> Self {
        Self {
            repeatability: 0.0,
            operator: 0.0,
            part_operator: 0.0,
            part_to_part: 0.0,
            reproducibility: 0.0,
            total_gagerr: 0.0,
            total_variation: 0.0,
            use_interaction: true,
        }
    }
    pub fn from_anova(anova: &AnovaTable) -> Self {
        let use_interaction = anova.use_interaction;
        let varterm = match use_interaction {
            true => anova.meansq_part_operator,
            false => anova.meansq_repeatability,
        };
        let repeatability_varcomp = anova.meansq_repeatability;
        let repeatability = match repeatability_varcomp {
            x if x < 0.0 => 0.0,
            _ => repeatability_varcomp,
        };
        let operator_varcomp =
            (anova.meansq_operators - varterm) / (anova.n_parts * anova.n_replicates) as f64;
        let operator = match operator_varcomp {
            x if x < 0.0 => 0.0,
            _ => operator_varcomp,
        };
        // Interaction
        let part_operator_varcomp = match use_interaction {
            true => {
                (anova.meansq_part_operator - anova.meansq_repeatability)
                    / anova.n_replicates as f64
            }
            false => 0.0,
        };
        let part_operator = match part_operator_varcomp {
            x if x < 0.0 => 0.0,
            _ => part_operator_varcomp,
        };
        // Part-to-part
        let part_to_part_varcomp =
            (anova.meansq_parts - varterm) / (anova.n_operators * anova.n_replicates) as f64;
        let part_to_part = match part_to_part_varcomp {
            x if x < 0.0 => 0.0,
            _ => part_to_part_varcomp,
        };
        let reproducibility_varcomp = operator + part_operator;
        let reproducibility = match reproducibility_varcomp {
            x if x < 0.0 => 0.0,
            _ => reproducibility_varcomp,
        };
        let total_gagerr_varcomp = repeatability + reproducibility;
        let total_gagerr = match total_gagerr_varcomp {
            x if x < 0.0 => 0.0,
            _ => total_gagerr_varcomp,
        };
        let total_variation_varcomp = total_gagerr + part_to_part;
        let total_variation = match total_variation_varcomp {
            x if x < 0.0 => 0.0,
            _ => total_variation_varcomp,
        };
        // If a variance component is negative, report as 0
        Self {
            repeatability,
            operator,
            part_operator,
            part_to_part,
            reproducibility,
            total_gagerr,
            total_variation,
            use_interaction,
        }
    }
}
