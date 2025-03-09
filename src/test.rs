use serde::Serialize;

use crate::kernels::{
    three_body::{ThreeBodyKernel, Yoshida4RelativeKernel},
    PhysicsState,
};

#[derive(Serialize)]
struct DataPoint {
    kernel: String,
    dt: u64,
    total_time: u64,
    p_std: f64,
    v_std: f64,
    p_diff_max: f64,
    v_diff_max: f64,
}

pub fn test_error<T: ThreeBodyKernel>(state: &PhysicsState, outfile: &str) {
    let kernal_name = std::any::type_name::<T>();

    let max_k = 25;
    let total_time = 2u64.pow(max_k);
    let mut ground_truth = state.clone();
    Yoshida4RelativeKernel::simulate(&mut ground_truth, 1, total_time, 1);

    let mut data: Vec<DataPoint> = vec![];

    for i in 1..=max_k {
        let dt = 2u64.pow(i);
        let mut state1 = state.clone();

        println!("--------------------------------");
        println!("{}", kernal_name);
        println!("dt = {}", dt);

        let timer = std::time::Instant::now();
        T::simulate(&mut state1, 1, total_time / dt, dt);
        println!("time = {}ns", timer.elapsed().as_nanos());
        let (p_std, v_std, p_diff_max, v_diff_max) = state1.print_deviation(&ground_truth);
        data.push(DataPoint {
            kernel: kernal_name.to_string(),
            dt,
            total_time,
            p_std,
            v_std,
            p_diff_max,
            v_diff_max,
        });

        println!("--------------------------------");
    }

    let json = serde_json::to_string(&data).expect("Failed to serialize data");
    std::fs::write(outfile, json).expect("Failed to write to file");
}
