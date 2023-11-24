use std::io::prelude::*;
use std::fs::OpenOptions;
use fixed::types::I2F14;
use fixed::types::I4F12;

fn main() -> std::io::Result<()>{
    let sin_table;
    let file_res = OpenOptions::new()
    .append(true)
    .create_new(true)
    .open("sin_table.rs");
    let mut fileh;

    match file_res {
        Ok(file_handl) => fileh=file_handl,
        Err(error) => {println!("{}",error); return Err(error)}
    }

    // create a sin table from 0 to PI with a resolution of 2^-7 radians
    // I4F12 type resolution is 2^-12 while I want 2^-7 resolution, so I need to shift right 
    // of 12-7=5 times
    sin_table=create_table(I4F12::ZERO, I4F12::FRAC_PI_2, 12-7);

    println!("created table with {} points in {:?}",sin_table.len(),fileh);

    print!("[");
    for elements in sin_table.iter(){
        print!("I2F14::from_num({}),",elements);
    }
    print!("]");

    // write the array in file
    writeln!(fileh,"pub const SIN_TABLE:[i16;{}]=[",sin_table.len())?;
    for elements in sin_table.iter(){
        writeln!(fileh,"{}, //I2F14::from_num({})",elements.to_bits(),elements).unwrap();
    }
    writeln!(fileh,"];")?;

Ok(())
}

/* 
create a fixed point I2F14 sin table -1 to 0.999 res 2^-14
from start_angle to end_angle, fixed point I4F12, -8 to 7.999, resolution 2^-12, [Rad]
division_shift will reduce the resolution and the table size.
if division_shift=0 the table will have full resolution of 2^-12
if division_shift=1 the table will have resolution of 2^-11 (half the size)
if division_shift=2 the table will have resolution of 2^-10 (quarter the size)
...
*/
fn create_table(start_angle:I4F12, end_angle:I4F12, division_shift:u16)-> Vec<I2F14> {
    
    let mut sin_table_fixed=Vec::new();
    let mut angle: I4F12=start_angle; // initialize starting angle
    let mut angle_f: f64;
    let mut sin_value_f: f64;
    let max_fixed: f64=I2F14::MAX.to_num(); // maximum representable value in fixed point type

    while angle < end_angle {
        angle_f=I4F12::to_num(angle);
        sin_value_f=angle_f.sin();
        if sin_value_f > max_fixed{  // limit to maximum representable value for fixed point type
            sin_value_f = max_fixed;
            }
        sin_table_fixed.push(I2F14::from_num(sin_value_f));
        //println!("angle {:?}",angle);
        angle=angle + I4F12::DELTA*(2<<(division_shift-1)); // take one sample every 2^division_shift steps
    }   
    return sin_table_fixed;
}