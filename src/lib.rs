//! # Gage Study
//!
//! This library provides the data structures needed for producing gage study repeatability and
//! reporducability metrics.
//!
//! # TODO list:
//!

pub mod anova;
pub mod data;
pub mod dataset;
pub mod jsondata;
pub mod operator;
pub mod part;
pub mod replicate;
pub mod study_evaluation;

#[cfg(test)]
mod tests {
    use super::*;
    pub use anova::Anova;
    pub use data::Data;
    pub use dataset::DataSet;
    pub use jsondata::JsonData;
    pub use study_evaluation::StudyEvaluation;

    pub static OP_A_JSON: &str = include_str!("../operatorA.json");
    pub static OP_B_JSON: &str = include_str!("../operatorB.json");
    pub static OP_C_JSON: &str = include_str!("../operatorC.json");
    pub static OP_A_CSV: &str = include_str!("../operatorA.csv");

    #[test]
    fn csv_test() {
        let data: Vec<Data> = Data::from_raw_csv(OP_A_CSV.as_bytes());
        let first = data[0].to_owned();
        let expect = Data {
            name: "a011".to_string(),
            part: "01".to_string(),
            operator: "A".to_string(),
            replicate: 1,
            measured: 2.78,
            nominal: 3.0,
        };
        assert_eq!(expect.name, first.name);
    }

    #[test]
    fn json_test() {
        let data: Vec<Data> = Data::from_raw_json(OP_A_JSON.as_bytes());
        let first = data[0].to_owned();
        let expect = Data {
            name: "a011".to_string(),
            part: "01".to_string(),
            operator: "A".to_string(),
            replicate: 1,
            measured: 2.78,
            nominal: 3.0,
        };
        assert_eq!(expect.name, first.name);
    }

    #[test]
    fn multi_operator() {
        let mut data = Vec::new();
        data.extend(Data::from_raw_json(OP_A_JSON.as_bytes()));
        data.extend(Data::from_raw_json(OP_B_JSON.as_bytes()));
        data.extend(Data::from_raw_json(OP_C_JSON.as_bytes()));
        let dataset = DataSet::from_data("Test", &data);
        let anova = Anova::from_data(&dataset);
        println!("{}", anova);
        let varcomp = StudyEvaluation::from_anova(&anova)
            .with_tolerance(1.0)
            .with_process_variation(5.15);
        println!("{}", varcomp);

        let float_tol = 0.001;
        let sumsq_total = 120.682;
        let sumsq_parts = 116.5294;
        let sumsq_operators = 0.09990;
        let sumsq_part_operator = 0.44681;
        let sumsq_repeatability = 3.60600;
        let meansq_parts = 12.94771;
        let meansq_operators = 0.04996;
        let meansq_part_operator = 0.02482;
        let meansq_repeatability = 0.06010;
        let f_parts = 521.61;
        let f_operators = 2.013;
        let f_part_operator = 0.413;

        let sumsq_total_delta = (anova.sumsq_total - sumsq_total).abs();
        let sumsq_parts_delta = (anova.sumsq_parts - sumsq_parts).abs();
        let sumsq_operators_delta = (anova.sumsq_operators - sumsq_operators).abs();
        let sumsq_part_operator_delta = (anova.sumsq_part_operator - sumsq_part_operator).abs();
        let sumsq_repeatability_delta = (anova.sumsq_repeatability - sumsq_repeatability).abs();

        let meansq_parts_delta = (anova.meansq_parts - meansq_parts).abs();
        let meansq_operators_delta = (anova.meansq_operators - meansq_operators).abs();
        let meansq_part_operator_delta = (anova.meansq_part_operator - meansq_part_operator).abs();
        let meansq_repeatability_delta = (anova.meansq_repeatability - meansq_repeatability).abs();

        let f_parts_delta = (anova.f_parts - f_parts).abs();
        let f_operators_delta = (anova.f_operators - f_operators).abs();
        let f_part_operator_delta = (anova.f_part_operator - f_part_operator).abs();

        assert!(
            anova.dof_total == 89,
            "dof_total Expected: {}, Found: {}",
            89,
            anova.dof_total
        );
        assert!(
            anova.dof_parts == 9,
            "dof_parts Expected: {}, Found: {}",
            9,
            anova.dof_parts
        );
        assert!(
            anova.dof_operators == 2,
            "dof_operators Expected: {}, Found: {}",
            2,
            anova.dof_operators
        );
        assert!(
            anova.dof_repeatability == 60,
            "dof_repeatability Expected: {}, Found: {}",
            60,
            anova.dof_repeatability
        );
        assert!(
            anova.dof_part_operator == 18,
            "dof_part_operator Expected: {}, Found: {}",
            18,
            anova.dof_part_operator
        );
        assert!(
            sumsq_total_delta < float_tol,
            "sumsq_total Expected: {}, Found: {}",
            sumsq_total,
            anova.sumsq_total
        );
        assert!(
            sumsq_parts_delta < float_tol,
            "sumsq_parts Expected: {}, Found: {}",
            sumsq_parts,
            anova.sumsq_parts
        );
        assert!(
            sumsq_operators_delta < float_tol,
            "sumsq_operators Expected: {}, Found: {}",
            sumsq_operators,
            anova.sumsq_operators
        );
        assert!(
            sumsq_part_operator_delta < float_tol,
            "sumsq_part_operator Expected: {}, Found: {}",
            sumsq_part_operator,
            anova.sumsq_part_operator
        );
        assert!(
            sumsq_repeatability_delta < float_tol,
            "sumsq_repeatability Expected: {}, Found: {}",
            sumsq_repeatability,
            anova.sumsq_repeatability
        );
        assert!(
            meansq_parts_delta < float_tol,
            "meansq_parts Expected: {}, Found: {}",
            meansq_parts,
            anova.meansq_parts
        );
        assert!(
            meansq_operators_delta < float_tol,
            "meansq_operators Expected: {}, Found: {}",
            meansq_operators,
            anova.meansq_operators
        );
        assert!(
            meansq_part_operator_delta < float_tol,
            "meansq_part_operator Expected: {}, Found: {}",
            meansq_part_operator,
            anova.meansq_part_operator
        );
        assert!(
            meansq_repeatability_delta < float_tol,
            "meansq_repeatability Expected: {}, Found: {}",
            meansq_repeatability,
            anova.meansq_repeatability
        );
        assert!(
            f_parts_delta < float_tol,
            "f_parts Expected: {}, Found: {}",
            f_parts,
            anova.f_parts
        );
        assert!(
            f_operators_delta < float_tol,
            "f_operators Expected: {}, Found: {}",
            f_operators,
            anova.f_operators
        );
        assert!(
            f_part_operator_delta < float_tol,
            "f_part_operator Expected: {}, Found: {}",
            f_part_operator,
            anova.f_part_operator
        );
    }
    #[test]
    fn multi_operator_no_interaction() {
        let mut data = Data::from_raw_json(OP_A_JSON.as_bytes());
        data.extend(Data::from_raw_json(OP_B_JSON.as_bytes()));
        data.extend(Data::from_raw_json(OP_C_JSON.as_bytes()));
        let dataset = DataSet::from_data("Test", &data).ignore_interaction();
        let anova = Anova::from_data(&dataset);
        println!("\n{}\n", anova);
        let varcomp = StudyEvaluation::from_anova(&anova)
            .with_tolerance(1.0)
            .with_process_variation(5.15);
        println!("\n{}\n", varcomp);

        let float_tol = 0.001;
        let sumsq_total = 120.682;
        let sumsq_parts = 116.5294;
        let sumsq_operators = 0.09990;
        let sumsq_part_operator = 0.0;
        let sumsq_repeatability = 4.053;
        let meansq_parts = 12.94771;
        let meansq_operators = 0.04996;
        let meansq_part_operator = 0.0;
        let meansq_repeatability = 0.05196;
        let f_parts = 249.202;
        let f_operators = 0.9615;
        let f_part_operator = 0.0;

        let sumsq_total_delta = (anova.sumsq_total - sumsq_total).abs();
        let sumsq_parts_delta = (anova.sumsq_parts - sumsq_parts).abs();
        let sumsq_operators_delta = (anova.sumsq_operators - sumsq_operators).abs();
        let sumsq_part_operator_delta = (anova.sumsq_part_operator - sumsq_part_operator).abs();
        let sumsq_repeatability_delta = (anova.sumsq_repeatability - sumsq_repeatability).abs();

        let meansq_parts_delta = (anova.meansq_parts - meansq_parts).abs();
        let meansq_operators_delta = (anova.meansq_operators - meansq_operators).abs();
        let meansq_part_operator_delta = (anova.meansq_part_operator - meansq_part_operator).abs();
        let meansq_repeatability_delta = (anova.meansq_repeatability - meansq_repeatability).abs();

        let f_parts_delta = (anova.f_parts - f_parts).abs();
        let f_operators_delta = (anova.f_operators - f_operators).abs();
        let f_part_operator_delta = (anova.f_part_operator - f_part_operator).abs();

        assert!(
            anova.dof_total == 89,
            "dof_total Expected: {}, Found: {}",
            89,
            anova.dof_total
        );
        assert!(
            anova.dof_parts == 9,
            "dof_parts Expected: {}, Found: {}",
            9,
            anova.dof_parts
        );
        assert!(
            anova.dof_operators == 2,
            "dof_operators Expected: {}, Found: {}",
            2,
            anova.dof_operators
        );
        assert!(
            anova.dof_repeatability == 78,
            "dof_repeatability Expected: {}, Found: {}",
            60,
            anova.dof_repeatability
        );
        assert!(
            anova.dof_part_operator == 0,
            "dof_part_operator Expected: {}, Found: {}",
            18,
            anova.dof_part_operator
        );
        assert!(
            sumsq_total_delta < float_tol,
            "sumsq_total Expected: {}, Found: {}",
            sumsq_total,
            anova.sumsq_total
        );
        assert!(
            sumsq_parts_delta < float_tol,
            "sumsq_parts Expected: {}, Found: {}",
            sumsq_parts,
            anova.sumsq_parts
        );
        assert!(
            sumsq_operators_delta < float_tol,
            "sumsq_operators Expected: {}, Found: {}",
            sumsq_operators,
            anova.sumsq_operators
        );
        assert!(
            sumsq_part_operator_delta < float_tol,
            "sumsq_part_operator Expected: {}, Found: {}",
            sumsq_part_operator,
            anova.sumsq_part_operator
        );
        assert!(
            sumsq_repeatability_delta < float_tol,
            "sumsq_repeatability Expected: {}, Found: {}",
            sumsq_repeatability,
            anova.sumsq_repeatability
        );
        assert!(
            meansq_parts_delta < float_tol,
            "meansq_parts Expected: {}, Found: {}",
            meansq_parts,
            anova.meansq_parts
        );
        assert!(
            meansq_operators_delta < float_tol,
            "meansq_operators Expected: {}, Found: {}",
            meansq_operators,
            anova.meansq_operators
        );
        assert!(
            meansq_part_operator_delta < float_tol,
            "meansq_part_operator Expected: {}, Found: {}",
            meansq_part_operator,
            anova.meansq_part_operator
        );
        assert!(
            meansq_repeatability_delta < float_tol,
            "meansq_repeatability Expected: {}, Found: {}",
            meansq_repeatability,
            anova.meansq_repeatability
        );
        assert!(
            f_parts_delta < float_tol,
            "f_parts Expected: {}, Found: {}",
            f_parts,
            anova.f_parts
        );
        assert!(
            f_operators_delta < float_tol,
            "f_operators Expected: {}, Found: {}",
            f_operators,
            anova.f_operators
        );
        assert!(
            f_part_operator_delta < float_tol,
            "f_part_operator Expected: {}, Found: {}",
            f_part_operator,
            anova.f_part_operator
        );
    }
}
