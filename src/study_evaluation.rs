use crate::anova::Anova;
use std::fmt;

#[derive(Debug)]
pub struct StudyEvaluation {
    pub total_gagerr: TotalGageRR,
    pub part_to_part: PartToPart,
    pub process_variation: f64,
    pub total_variation: TotalVariation,
    pub tolerance: f64,
    pub use_interaction: bool,
}

impl Default for StudyEvaluation {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl fmt::Display for StudyEvaluation {
    /// Defines the format for displaying the variance components table
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut source = "*** Variance Components ***\n".to_owned();
        let var_comp_header = format!(
            "{:<17}  {:>9}  {:>5}\n{:<17}  {:>9}  {:>5}\n",
            "", "", "%Contribution", "Source", "VarComp", "(of VarComp)"
        );
        source.push_str(&var_comp_header);
        let total_gagerr_varcomp = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "Total Gage R&R",
            self.total_gagerr.varcomp,
            self.total_gagerr.varcomp / self.total_variation.varcomp * 100.0
        );
        source.push_str(&total_gagerr_varcomp);
        let repeatability_varcomp = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "  Repeatability",
            self.total_gagerr.repeatability.varcomp,
            self.total_gagerr.repeatability.varcomp / self.total_variation.varcomp * 100.0
        );
        source.push_str(&repeatability_varcomp);
        let reproducibility_varcomp = match self.use_interaction {
            true => format!(
                "{:<17}  {:>9.7}  {:>6.2}\n",
                "  Reproducibility",
                self.total_gagerr.reproducibility.varcomp,
                self.total_gagerr.reproducibility.varcomp / self.total_variation.varcomp * 100.0
            ),
            false => "".to_owned(),
        };
        source.push_str(&reproducibility_varcomp);
        let part_to_part_varcomp = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "Part-to-Part",
            self.part_to_part.varcomp,
            self.part_to_part.varcomp / self.total_variation.varcomp * 100.0
        );
        source.push_str(&part_to_part_varcomp);
        let total_varcomp = format!(
            "{:<17}  {:>9.7}  {:>6.2}\n",
            "Total Variation", self.total_variation.varcomp, 100.00
        );
        source.push_str(&total_varcomp);
        // Gage evaluation
        source.push_str("\n*** Gage Evaluation ***\n");
        let eval_header = format!(
            "{:<17}  {:>10}  {:>10}  {:>10}  {:>10}\n{:<17}  {:>10}  {:>10}  {:>10}  {:>10}\n",
            "",
            "",
            "Study Var",
            "%Study Var",
            "%Tolerance",
            "Source",
            "StdDev(SD)",
            &format!("({} x SD)", self.process_variation),
            "(%SV)",
            "(SV/Tol)"
        );
        source.push_str(&eval_header);
        let total_gagerr_eval = format!(
            "{:<17}  {:>10.6}  {:>10.5}  {:>10.2}  {:>10.2}\n",
            "Total Gage R&R",
            self.total_gagerr.stddev,
            self.total_gagerr.stddev * self.process_variation,
            self.total_gagerr.stddev / self.total_variation.stddev * 100.0,
            self.total_gagerr.stddev * self.process_variation / self.tolerance * 100.0
        );
        source.push_str(&total_gagerr_eval);
        let repeatability_eval = format!(
            "{:<17}  {:>10.6}  {:>10.5}  {:>10.2}  {:>10.2}\n",
            "  Repeatability",
            self.total_gagerr.repeatability.stddev,
            self.total_gagerr.repeatability.stddev * self.process_variation,
            self.total_gagerr.repeatability.stddev / self.total_variation.stddev * 100.0,
            self.total_gagerr.repeatability.stddev * self.process_variation / self.tolerance
                * 100.0
        );
        source.push_str(&repeatability_eval);
        let reproducibility_eval = match self.use_interaction {
            true => format!(
                "{:<17}  {:>10.6}  {:>10.5}  {:>10.2}  {:>10.2}\n",
                "  Reproducibility",
                self.total_gagerr.reproducibility.stddev,
                self.total_gagerr.reproducibility.stddev * self.process_variation,
                self.total_gagerr.reproducibility.stddev / self.total_variation.stddev * 100.0,
                self.total_gagerr.reproducibility.stddev * self.process_variation / self.tolerance
                    * 100.0,
            ),
            false => "".to_owned(),
        };
        source.push_str(&reproducibility_eval);
        let part_to_part_eval = format!(
            "{:<17}  {:>10.6}  {:>10.5}  {:>10.2}  {:>10.2}\n",
            "Part-to-Part",
            self.part_to_part.stddev,
            self.part_to_part.stddev * self.process_variation,
            self.part_to_part.stddev / self.total_variation.stddev * 100.0,
            self.part_to_part.stddev * self.process_variation / self.tolerance * 100.0
        );
        source.push_str(&part_to_part_eval);
        let total_eval = format!(
            "{:<17}  {:>10.6}  {:>10.5}  {:>10.2}  {:>10.2}\n",
            "Total Variation",
            self.total_variation.stddev,
            self.total_variation.stddev * self.process_variation,
            100.0,
            self.total_variation.stddev * self.process_variation / self.tolerance * 100.0
        );
        source.push_str(&total_eval);

        write!(f, "{}", source)
    }
}

impl StudyEvaluation {
    pub fn new(tolerance: f64) -> Self {
        Self {
            total_gagerr: TotalGageRR::default(),
            part_to_part: PartToPart::default(),
            total_variation: TotalVariation::default(),
            process_variation: 6.0,
            tolerance,
            use_interaction: true,
        }
    }

    pub fn from_anova(anova: &Anova) -> Self {
        let use_interaction = anova.use_interaction;
        let total_gagerr = TotalGageRR::from_anova(anova);
        let part_to_part = PartToPart::from_anova(anova);
        let total_variation = TotalVariation::new(total_gagerr.varcomp + part_to_part.varcomp);
        Self {
            total_gagerr,
            part_to_part,
            total_variation,
            process_variation: 6.0,
            tolerance: 1.0,
            use_interaction,
        }
    }

    pub fn with_process_variation(mut self, process_variation: f64) -> Self {
        self.process_variation = process_variation;
        self
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }
}

#[derive(Debug, Default)]
pub struct TotalGageRR {
    pub varcomp: f64,
    pub stddev: f64,
    pub repeatability: Repeatability,
    pub reproducibility: Reproducibility,
    pub use_interaction: bool,
}

impl TotalGageRR {
    pub fn new() -> Self {
        Self {
            use_interaction: true,
            ..Default::default()
        }
    }
    pub fn from_anova(anova: &Anova) -> Self {
        let use_interaction = anova.use_interaction;
        let varterm = match use_interaction {
            true => anova.meansq_part_operator,
            false => anova.meansq_repeatability,
        };
        let operator_varcomp =
            (anova.meansq_operators - varterm) / (anova.n_parts * anova.n_replicates) as f64;
        let part_operator_varcomp =
            (anova.meansq_part_operator - anova.meansq_repeatability) / anova.n_replicates as f64;
        let repeatability = Repeatability::new(anova.meansq_repeatability);
        let reproducibility = Reproducibility::new(operator_varcomp + part_operator_varcomp);
        let varcomp = (repeatability.varcomp + reproducibility.varcomp).clamp(0.0, f64::MAX);

        Self {
            varcomp,
            stddev: varcomp.sqrt(),
            repeatability,
            reproducibility,
            use_interaction,
        }
    }
}

#[derive(Debug, Default)]
pub struct Repeatability {
    pub varcomp: f64,
    pub stddev: f64,
}

impl Repeatability {
    pub fn new(varcomp: f64) -> Self {
        let varcomp = varcomp.clamp(0.0, f64::MAX);
        Self {
            varcomp,
            stddev: varcomp.sqrt(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Reproducibility {
    pub varcomp: f64,
    pub stddev: f64,
}

impl Reproducibility {
    pub fn new(varcomp: f64) -> Self {
        let varcomp = varcomp.clamp(0.0, f64::MAX);
        Self {
            varcomp,
            stddev: varcomp.sqrt(),
        }
    }
}

#[derive(Debug, Default)]
pub struct PartToPart {
    pub varcomp: f64,
    pub stddev: f64,
}

impl PartToPart {
    pub fn new(varcomp: f64) -> Self {
        let varcomp = varcomp.clamp(0.0, f64::MAX);
        Self {
            varcomp,
            stddev: varcomp.sqrt(),
        }
    }
    pub fn from_anova(anova: &Anova) -> Self {
        let varterm = match anova.use_interaction {
            true => anova.meansq_part_operator,
            false => anova.meansq_repeatability,
        };
        let var = (anova.meansq_parts - varterm) / (anova.n_operators * anova.n_replicates) as f64;
        let varcomp = var.clamp(0.0, f64::MAX);

        Self {
            varcomp,
            stddev: varcomp.sqrt(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TotalVariation {
    pub varcomp: f64,
    pub stddev: f64,
}

impl TotalVariation {
    pub fn new(varcomp: f64) -> Self {
        let varcomp = varcomp.clamp(0.0, f64::MAX);
        Self {
            varcomp,
            stddev: varcomp.sqrt(),
        }
    }
}
