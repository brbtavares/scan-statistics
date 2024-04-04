mod definitions;
use crate::definitions::*;

fn main() {

    let map: Vec<Region> = create_map();
    for v in map {
        println!("The expectation of the region {} is {}, the number of cases is {} and its centroid is ({},{})...{}", v.idx, v.expectation, v.cases, v.centroid.x, v.centroid.y, v.is_cluster);
    }
    

    
}
