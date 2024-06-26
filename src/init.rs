use crate::convolution::*;
use crate::growth::*;

pub struct Field {
    pub t: f64,
    pub l: usize,
    pub h: usize,
    pub k_size: usize,
    pub nb_channels: usize,
    pub m: Vec<Vec<Vec<f64>>>,
}

pub enum Kernel {
    Ring
}

impl Field {
    // A function to create an empty field
    pub fn new_field(h: usize, l: usize, nb_chan: usize) -> Field {
        Field {
            t: 0.0,
            l: l,
            h: h,
            k_size: 0,
            nb_channels: nb_chan,
            m: vec![vec![vec![0.; l]; h]; nb_chan],
        }
    }

    pub fn to_tore(mut self: Field, kernel: Vec<Vec<f64>>){
        for i in 0..self.nb_channels{
            self.m[i] = tore_format(&self.m[i], &kernel);
        }
        self.k_size = kernel.len();
    }

    pub fn get_xy(self: &Field, x: usize, y: usize, chanel: usize) -> f64{
        self.m[chanel][self.k_size + x][self.k_size + y]
    }

    pub fn fill(self: &mut Field,chan: usize, val: f64){
        for i in 0..self.h{
            for j in 0..self.l{
                self.m[chan][i][j] = val;
            }
        }
    }

    pub fn fill_deg(self: &mut Field, chan: usize, start: f64, end: f64){
        for i in 0..self.h{
            for j in 0..self.l{
                let fi = i as f64;
                let fj = j as f64;
                let fl = self.l as f64;
                let fh = self.h as f64;
                self.m[chan][i][j] = start + (fi+fj)*(end)/(fh+fl);
            }
        }
    }
}

pub fn kernel_init(k_type: Kernel, h: usize) -> Vec<Vec<f64>>{
    match k_type{
        Kernel::Ring => {
            return ring_kernel(h)
        }
    }
}


fn ring_kernel(h: usize) -> Vec<Vec<f64>>{
    let mut result = vec![vec![0.0 ; h]; h];

    let rayon = h/2;
    let mut sum = 0.0;
    
    for x in 0..h{
        for y in 0..h {
            let dx;
            let dy;
            if x > rayon { dx =  x-rayon}
            else { dx = rayon-x}
            if y > rayon { dy =  y-rayon}
            else { dy = rayon-y}

            let distance = ((dx*dx + dy*dy) as f64).sqrt()/(rayon as f64);
            if distance <= 1.0 {
                let d_gauss = gaussian(0.5,0.15,distance);
                sum += d_gauss;
                result[x][y] = d_gauss;
            }
        }
    }
    
    for i in 0..h{
        for j in 0..h{
            result[i][j] /= sum;
        }
    }

    /*
    sum = 0.0;

    for i in 0..h{
        for j in 0..h{
            sum += result[i][j];
        }
    }
    
    println!("{}\n", sum);
    */

    result
}
