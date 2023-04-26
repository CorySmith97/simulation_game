use bevy::{prelude::*};
use std::{vec::Vec};
use rand::{prelude::*, Rng};


fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    let interpolation = (a1 - a0) * w + a0;

    return interpolation;
}
fn random_gradient( ix: i32, iy: i32) -> Vec<f32>{
    let max = f32::MAX;
    let a: i32 = ix;
    let b: i32 = iy;

    let mut rng = thread_rng();
    
    let rng_num: i32 = rng.gen_range(1..999);

    let rng_a = a * rng_num; 
    let rng_a_small: f32 = rng_a as f32 / max * 100000000000000000000000000000000000.0;
    
    let rng_b = b * rng_num;
    let rng_b_small: f32 = rng_b as f32/ max * 100000000000000000000000000000000000.0;


    let mut vec = Vec::new();
    vec.push(rng_a_small.cos());
    vec.push(rng_b_small.sin());

    return vec;
}

fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    let mut vector = Vec::new();

    vector = random_gradient(ix, iy);

    let dx: f32 = x - ix as f32;
    let dy: f32 = x - iy as f32;

    let dot_product = dx * vector[0] + dy * vector[1];

    return dot_product;
}

 pub fn perlin_noise(x: f32, y: f32) -> f32 {
     let max = f32::MAX;

    let x0_float: f32 = x.floor();
    let x0: i32 = x0_float as i32;
    let x1 = x0+1;
    let y0_float: f32 = y.floor();
    let y0: i32 = y0_float as i32;
    let y1 = y0+1;

    let interpolated_weight_x = x - (x0 as f32);
    let interpolated_weight_y = y - (y0 as f32);

    let mut n0 = dot_grid_gradient(x0, y0, x, y);
    let mut n1 = dot_grid_gradient(x1, y0, x, y);
    let ix0 = interpolate(n0, n1, interpolated_weight_x);

    n0 = dot_grid_gradient(x0, y1, x, y);
    n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1 = interpolate(n0, n1, interpolated_weight_x);

    let value: f32 = interpolate(ix0, ix1, interpolated_weight_y) / max;

    return value;
  }

fn array_proccesing()   {
    let mut vec = Vec::new();
    for _x in 1..100{
        let x = _x as f32;
        for _y in 1..100{
            let y = _y as f32;
            let perlin = perlin_noise(x, y);
            vec.push(perlin);
        }
    }
    let mut inbetween_vec = Vec::new();
    for _i in vec {
        let i = _i * 10000000000000000000000000000000000000.0;
        inbetween_vec.push(i);
    }
    let mut clean_vec = Vec::new();

    for _i in inbetween_vec {
        let cleanup = (_i *100.0).round() / 100.0;
        clean_vec.push(cleanup);
    }
    
    let mut final_vec= Vec::new();

    for _i in clean_vec {
        if _i > 1.0 {
            final_vec.push(0.0);
        } else if _i < -1.0 {
            final_vec.push(0.0);
        } else {
            final_vec.push(_i);
        }
    }
}
