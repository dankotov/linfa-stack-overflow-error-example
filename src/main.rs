use linfa::prelude::*;
use linfa_clustering::Dbscan;
use ndarray::prelude::*;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./data/data-works.json").expect("Failed to read file");
    // Uncomment the below line to test on a dataset that produces the stack overflow. The below dataset is smaller than the above dataset, but the above dataset doesnt produce a stack overflow error.
    // let mut file = File::open("./data/data-stackoverflow.json").expect("Failed to read file");

    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read data");

    let items: Vec<Vec<f32>> = serde_json::from_str(&json_data).unwrap();

    let converted_items =
        Array2::from_shape_fn((items.len(), items[0].len()), |(i, j)| items[i][j]);

    let cluster_memberships = Dbscan::params(1500)
        .tolerance(30.0)
        .transform(&converted_items)
        .unwrap();

    println!("{:?}", cluster_memberships);
}
