pub mod anova;
pub mod data;
pub mod dataset;
pub mod jsondata;
pub mod operators;
pub mod parts;
pub mod replicates;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use anova::AnovaTable;
    pub use data::Data;
    pub use dataset::DataSet;
    pub use jsondata::JsonData;

    #[test]
    fn multi_operator() {
        let data_path_a = "/home/heavymetalnerd/Development/rust/gage_study/operatorA.json";
        let mut jsondata_a = JsonData::new().with_source(data_path_a);
        let data_path_b = "/home/heavymetalnerd/Development/rust/gage_study/operatorB.json";
        let mut jsondata_b = JsonData::new().with_source(data_path_b);
        let data_path_c = "/home/heavymetalnerd/Development/rust/gage_study/operatorC.json";
        let mut jsondata_c = JsonData::new().with_source(data_path_c);
        let mut data = Vec::new();
        data.extend(jsondata_a.deserialize_data().unwrap());
        data.extend(jsondata_b.deserialize_data().unwrap());
        data.extend(jsondata_c.deserialize_data().unwrap());
        for d in data.chunks(3) {
            println!("{}, {}, {}", d[0].measured, d[1].measured, d[2].measured);
        }
        let dataset = DataSet::from_data("Test", &data);
        let anova = AnovaTable::from_data(&dataset);

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
}
