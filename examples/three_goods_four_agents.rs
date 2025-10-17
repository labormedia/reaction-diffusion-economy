use std::f64;

fn main() {
    // Initial endowments for 4 agents, 3 goods
    let initial_endowments: Vec<Vec<f64>> = vec![
        vec![10.1, 1.1, 0.1],
        vec![10.1, 0.1, 1.1],
        vec![0.1, 10.1, 1.1],
        vec![5.1, 5.1, 5.1],
    ];

    // Net transformation matrix (rows: goods, columns: reactions)
    let net = vec![
        vec![-2.0, 1.0],  // good 1
        vec![-1.0, 0.5],  // good 2
        vec![3.0, -1.0],  // good 3
    ];

    // Phase 1: Reaction - Compute post-reaction endowments using grid search
    let mut react_endowments: Vec<Vec<f64>> = Vec::new();
    for e in &initial_endowments {
        let upper_x0 = (e[0] / 2.0).min(e[1] / 1.0);
        let upper_x1 = e[2] / 1.0;

        let mut best_obj = f64::NEG_INFINITY;
        let mut best_x = vec![0.0, 0.0];

        let steps = 100;
        for ix0 in 0..=steps {
            let x0 = (ix0 as f64 / steps as f64) * upper_x0;
            for ix1 in 0..=steps {
                let x1 = (ix1 as f64 / steps as f64) * upper_x1;

                let new_e = vec![
                    e[0] + net[0][0] * x0 + net[0][1] * x1,
                    e[1] + net[1][0] * x0 + net[1][1] * x1,
                    e[2] + net[2][0] * x0 + net[2][1] * x1,
                ];

                if new_e[0] > 0.0 && new_e[1] > 0.0 && new_e[2] > 0.0 {
                    let obj = (new_e[0].ln() + new_e[1].ln() + new_e[2].ln()) / 3.0;
                    if obj > best_obj {
                        best_obj = obj;
                        best_x = vec![x0, x1];
                    }
                }
            }
        }

        let react_e = vec![
            e[0] + net[0][0] * best_x[0] + net[0][1] * best_x[1],
            e[1] + net[1][0] * best_x[0] + net[1][1] * best_x[1],
            e[2] + net[2][0] * best_x[0] + net[2][1] * best_x[1],
        ];
        react_endowments.push(react_e.clone());

        println!("Agent {} reaction intensities: {:?}", react_endowments.len(), best_x);
        println!("Post-reaction endowment: {:?}", react_e);
    }

    // Phase 2: Diffusion - Find equilibrium prices using grid search, normalized sum p = 1
    let mut best_p = vec![0.0, 0.0, 0.0];
    let mut min_cost = f64::INFINITY;

    let steps = 200;  // Finer grid for better accuracy (200x200 ~40k iterations, fast)
    for ip1 in 1..steps {
        let p1 = (ip1 as f64) / (steps as f64);
        for ip2 in 1..(steps - ip1) {
            let p2 = (ip2 as f64) / (steps as f64);
            let p3 = 1.0 - p1 - p2;
            if p3 <= 0.0 {
                continue;
            }
            let p = vec![p1, p2, p3];

            let mut sum_s = vec![0.0; 3];
            let mut sum_z = vec![0.0; 3];

            for react_e in &react_endowments {
                let income = p[0] * react_e[0] + p[1] * react_e[1] + p[2] * react_e[2];
                let mut exp = vec![0.0; 3];
                let mut max_exp = 0.0;
                for k in 0..3 {
                    exp[k] = p[k] * react_e[k];
                    if exp[k] > max_exp {
                        max_exp = exp[k];
                    }
                }
                let min_ratio = (income / 3.0) / max_exp;
                let lambda = if min_ratio < 1.0 { 1.0 - min_ratio } else { 0.0 };
                let one_m_l = 1.0 - lambda;

                let mut z = vec![0.0; 3];
                for k in 0..3 {
                    let ideal_x = (income / 3.0) / p[k];
                    z[k] = ideal_x - one_m_l * react_e[k];
                }

                for k in 0..3 {
                    sum_z[k] += z[k];
                    sum_s[k] += lambda * react_e[k];
                }
            }

            let mut cost = 0.0;
            for k in 0..3 {
                let excess = sum_z[k] - sum_s[k];
                cost += excess * excess;
            }

            if cost < min_cost {
                min_cost = cost;
                best_p = p.clone();
            }
        }
    }

    println!("\nEquilibrium prices: {:?} (sum sq excess: {})", best_p, min_cost);

    // Compute allocations with best_p
    let p = best_p;
    let mut lambdas = vec![];
    let mut zs: Vec<Vec<f64>> = vec![];
    let mut next_endowments: Vec<Vec<f64>> = vec![];

    for react_e in &react_endowments {
        let income = p[0] * react_e[0] + p[1] * react_e[1] + p[2] * react_e[2];
        let mut exp = vec![0.0; 3];
        let mut max_exp = 0.0;
        for k in 0..3 {
            exp[k] = p[k] * react_e[k];
            if exp[k] > max_exp {
                max_exp = exp[k];
            }
        }
        let min_ratio = (income / 3.0) / max_exp;
        let lambda = if min_ratio < 1.0 { 1.0 - min_ratio } else { 0.0 };
        let one_m_l = 1.0 - lambda;

        let mut z = vec![0.0; 3];
        for k in 0..3 {
            let ideal_x = (income / 3.0) / p[k];
            z[k] = ideal_x - one_m_l * react_e[k];
        }

        let mut next_e = vec![0.0; 3];
        for k in 0..3 {
            next_e[k] = one_m_l * react_e[k] + z[k];
        }

        lambdas.push(lambda);
        zs.push(z.clone());
        next_endowments.push(next_e);
    }

    println!("\nLambdas: {:?}", lambdas);
    println!("Zs:");
    for (i, z) in zs.iter().enumerate() {
        println!("Agent {}: {:?}", i + 1, z);
    }
    println!("Next endowments:");
    for (i, e) in next_endowments.iter().enumerate() {
        println!("Agent {}: {:?}", i + 1, e);
    }
}