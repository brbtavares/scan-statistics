use vega_lite_5::*;
use std::thread;

pub struct Region {
    pub(crate) index: i64,
    pub(crate) expectation: i64,
    pub(crate) cases: i64,
    pub(crate) centroid: Point,
    pub(crate) is_cluster: bool,
}

impl Region {
    pub fn plot(&self) {
        let builder = thread::Builder::new().stack_size(10 * 1024 * 1024); // 10MB stack size
        let handler = builder.spawn(|| {
            let spec = r##"{
                "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
                "data": {
                  "values": [
                    {
                      "x": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
                      "y": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
                    }
                  ]
                },
                "transform": [
                  {"flatten": ["x"]},
                  {"flatten": ["y"]},
                  {"calculate": "datum.x + ',' + datum.y", "as": "l"}
                ],
                "height": {"step": 32},
                "width": {"step": 40},
                "mark": {"type": "point", "size": 1600},
                "encoding": {
                  "x": {"field": "x", "type": "ordinal", "axis": null},
                  "y": {"field": "y", "type": "ordinal", "axis": null},
                  "shape": {
                    "condition": {
                      "test": "datum.y % 2 == 1",
                      "value": "'M0,-0.6 L1,-1 L2,-0.6 L2,0.6 L1,1 L0,0.6 Z'"
                    },
                    "value": "'M-1,-0.6 L0,-1 L1,-0.6 L1,0.6 L0,1 L-1,0.6 Z'"
                  }
                },
                "config": {"view": {"stroke": null}}
            }"##;
        
            let chart: Result<Vegalite, serde_json::Error> = serde_json::from_str(spec);
            match chart {
                Ok(chart) => {
                    let _ = chart.show();
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }).unwrap();
    
        handler.join().unwrap();
    }
}

pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}