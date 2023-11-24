use fixed::types::I2F14;
use fixed::types::I4F12;

/*
sin function approximation, using a table fixed point 0 to 0.999,
angle is fixed point I4F12 -8 to 7.999 res 0.00024 [rad]
returns fixed point I2F14 -1 to +0.999
division_shift will reduce the resolution and the table size.
if division_shift=0 the table will have full resolution of 2^-12
if division_shift=1 the table will have resolution of 2^-11 (half the size)
if division_shift=2 the table will have resolution of 2^-10 (quarter the size)
...
*/
pub fn sin_t(angle: I4F12, table: &[i16], division_shift: u16) -> I2F14 {
    /*
    angle is converted directly in table index
    the table size is proportional to angle resolution and maximum value
    the size is PI/2*resolution
    the resolution of input angle is I4F12 2^-12, in this case the table size should be
    1.57*2^-12=6400 points
    subsampling is needed in order to reduce the table size, the subsampling is
    determined by resolution parameter
     */

    let index;
    let angle_int: I4F12;
    let sign: bool;

    if angle <= -2*I4F12::PI
    {
        // less than -2PI & overflow management
        return I2F14::ZERO;
    } else if angle < -3*I4F12::FRAC_PI_2
    {
        // -2PI to -3/2PI -> 0 to PI/2
        angle_int=angle + 2 * I4F12::PI;
        sign = true;
    } else if angle < -I4F12::PI
    {
        // -3/2PI to -PI -> PI/2 to 0
        angle_int= -I4F12::PI - angle;
        sign = true;
    } else if angle < -I4F12::FRAC_PI_2
    {
        // -PI to -PI/2 -> 0 to PI/2
        angle_int=angle + I4F12::PI;
        sign = false;
    } else if angle < I4F12::ZERO
    {
        // -PI/2 to 0 -> PI/2 to 0
        angle_int = -angle;
        sign = false;
    } else if angle < I4F12::FRAC_PI_2
    {
        // 0 to PI/2 -> 0 to PI/2
        angle_int = angle;
        sign = true;
    } else if angle < I4F12::PI
    {
        // PI/2 to PI -> PI/2 to 0
        angle_int=I4F12::PI - angle;
        sign = true;
    } else if angle < 3 * I4F12::FRAC_PI_2
    {
        // PI to 3/2PI -> 0 to PI/2
        angle_int=angle - I4F12::PI;
        sign = false;
    } else if angle < 2 * I4F12::PI
    {
        // 3/2PI to 2PI -> PI/2 to 0
        angle_int=2 * I4F12::PI - angle;
        sign = false;
    } else {
        // > 2PI overflow management
        return I2F14::ZERO;
    }
    index = (angle_int.to_bits() >> division_shift) as usize;
    if sign == true {
        return I2F14::from_bits(table[index]);
    } else {
        return -I2F14::from_bits(table[index]);
    }
}

/*
cos function approximation, using a table fixed point 0 to 0.999,
angle is fixed point I4F12 -8 to 7.999 res 0.00024 [rad]
returns fixed point I2F14 -1 to +0.999
division_shift will reduce the resolution and the table size.
if division_shift=0 the table will have full resolution of 2^-12
if division_shift=1 the table will have resolution of 2^-11 (half the size)
if division_shift=2 the table will have resolution of 2^-10 (quarter the size)
...
*/
pub fn cos_t(angle: I4F12, table: &[i16], division_shift : u16) -> I2F14 {
    /*
    angle is converted directly in table index
    the table size is proportional to angle resolution and maximum value
    the size is PI/2*resolution
    the resolution of input angle is I4F12 2^-12, in this case the table size should be
    1.57*2^-12=6400 points
    subsampling is needed in order to reduce the table size, the subsampling is
    determined by resolution parameter
     */

    let index;
    let angle_int: I4F12;
    let sign: bool;

    if angle <= -2*I4F12::PI
    {
        // less than -2PI & overflow management
        return I2F14::ONE;
    } else if angle < -3*I4F12::FRAC_PI_2
    {
        // -2PI to -3/2PI -> PI/2 to 0
        angle_int= - 3 * I4F12::FRAC_PI_2 - angle;
        sign = true;
    } else if angle < -I4F12::PI
    {
        // -3/2PI to -PI -> 0 to PI/2
        angle_int= 3 * I4F12::FRAC_PI_2 + angle;
        sign = false;
    } else if angle < -I4F12::FRAC_PI_2
    {
        // -PI to -PI/2 -> PI/2 to 0
        angle_int= - I4F12::FRAC_PI_2 - angle;
        sign = false;
    } else if angle < I4F12::ZERO
    {
        // -PI/2 to 0 -> 0 to PI/2
        angle_int = angle + I4F12::FRAC_PI_2;
        sign = true;
    } else if angle < I4F12::FRAC_PI_2
    {
        // 0 to PI/2 -> PI/2 to 0
        angle_int = I4F12::FRAC_PI_2 - angle;
        sign = true;
    } else if angle < I4F12::PI
    {
        // PI/2 to PI -> 0 to PI/2
        angle_int=angle - I4F12::FRAC_PI_2;
        sign = false;
    } else if angle < 3 * I4F12::FRAC_PI_2
    {
        // PI to 3/2PI -> PI/2 to 0
        angle_int=3 * I4F12::FRAC_PI_2 - angle;
        sign = false;
    } else if angle < 2 * I4F12::PI
    {
        // 3/2PI to 2PI -> 0 to PI/2
        angle_int=angle - 3 * I4F12::FRAC_PI_2;
        sign = true;
    } else {
        // > 2PI overflow management
        return I2F14::ONE;
    }
    index = (angle_int.to_bits() >> division_shift) as usize;
    if sign == true {
        return I2F14::from_bits(table[index]);
    } else {
        return -I2F14::from_bits(table[index]);
    }
}
