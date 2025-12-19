use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use rand::Rng;

pub struct Points<'a>(&'a Vec<Point>);

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub struct Point(Vec<f64>);

pub struct Radii(Vec<f64>);

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub struct Circle {
    point: Point,
    radius: f64,
}

#[wasm_bindgen]
pub fn simulate(dimension: usize, num_points: usize) -> Vec<Circle> {
    let mut rng = rand::rng();
    let mut points = Vec::new();
    
    for _ in 0..num_points {
        let point = Point((0..dimension).map(|_| rng.random_range(0.0..=1.0)).collect());
        points.push(point);
    }
    
    let radii = growth_maximal_system_of_non_overlapping_balls(&Points(&points));
    
    let mut circles = Vec::new();
    for (i, point) in points.into_iter().enumerate() {
        circles.push(Circle { point: point, radius: radii.0[i] });
    }
    
    circles
}

pub fn growth_maximal_system_of_non_overlapping_balls(points: &Points<'_>) -> Radii {
    let mut radii = Radii(vec![0.0; points.0.len()]);
    
    let len = points.0.len();
    
    if len == 1 {
        radii.0[0] = max_radius(&points.0[0]);
    }
    
    for _ in 0..len {
        let mut min_radius = (f64::MAX, None);
        
        for i in 0..len {
            for j in 0..len {
                if i != j && (radii.0[i] == 0.0 || radii.0[j] == 0.0) {
                    let distance = distance(&points.0[i], &points.0[j]);
                    let i_max_radius = max_radius(&points.0[i]);
                    let j_max_radius = max_radius(&points.0[j]);

                    let radius;
                    let point_pair;

                    if radii.0[i] != 0.0  {
                        radius = (distance - radii.0[i]).min(j_max_radius);
                        point_pair = (j, None);
                    } else if radii.0[j] != 0.0 {
                        radius = (distance - radii.0[j]).min(i_max_radius);
                        point_pair = (i, None);
                    } else {
                        if i_max_radius < distance / 2.0 && i_max_radius < j_max_radius {
                            radius = i_max_radius;
                            point_pair = (i, None);
                        } else if j_max_radius < distance / 2.0 && j_max_radius < i_max_radius {
                            radius = j_max_radius;
                            point_pair = (j, None);
                        } else {
                            radius = distance / 2.0;
                            point_pair = (i, Some(j));
                        }
                    }
                    
                    if radius < min_radius.0 {
                        min_radius = (radius, Some(point_pair));
                    }
                }
            }
        }
        
        if let (radius, Some((i, j))) = min_radius {
            radii.0[i] = radius;
            if let Some(k) = j {
                radii.0[k] = radius;
            }
        }
    }
    
    radii
}

fn distance(point1: &Point, point2: &Point) -> f64 {
    let mut distance = 0.0;
    for i in 0..point1.0.len() {
        distance += (point1.0[i] - point2.0[i]).powi(2);
    }
    distance.sqrt()
}

fn max_radius(point: &Point) -> f64 {
    let mut min_distance_to_wall = f64::MAX;
    
    for &coordinate in &point.0 {
        let distance_to_wall = coordinate.min(1.0 - coordinate);
        min_distance_to_wall = min_distance_to_wall.min(distance_to_wall);
    }
    
    min_distance_to_wall
}
