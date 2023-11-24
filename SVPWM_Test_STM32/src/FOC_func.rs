use fixed::types::I6F10;
use fixed::types::I4F12;
mod table_trig;
mod sin_table;

/*
Transforms a couple of vectors in fixed domain Vd and Vq in a pair of vectors in
rotating domain Valpha Vbeta at an angle theta
Voltages fixed point I6F10 from -32 to 31.99V resolution 0.00097V
*/
pub fn inverse_park(Vd:I6F10,Vq:I6F10,theta:I4F12) -> (I6F10, I6F10) {
let Valpha=Vd * I6F10::from_num(table_trig::cos_t(theta,&sin_table::SIN_TABLE, 12-7)) - Vq * I6F10::from_num(table_trig::sin_t(theta, &sin_table::SIN_TABLE, 12-7));
let Vbeta=Vd * I6F10::from_num(table_trig::sin_t(theta,&sin_table::SIN_TABLE, 12-7)) + Vq * I6F10::from_num(table_trig::cos_t(theta, &sin_table::SIN_TABLE, 12-7));
return (Valpha,Vbeta)
}

/*
mofified inverse clarke transformation, calculates i j k coefficients useful for SVPWM
calculation, from Valpha and Vbeta in rotating domain
*/
pub fn mod_inverse_clarke(Valpha:I6F10,Vbeta:I6F10) -> (I6F10,I6F10,I6F10){
    let i=I6F10::SQRT_3/2 * Valpha-Vbeta/2;
    let j=Vbeta;
    let k=-I6F10::SQRT_3/2 * Valpha-Vbeta/2;
    return (i,j,k)
}

/*
svpwm transforms i j k coefficients to U V W voltage values to be used for setting
pwm registers for the 3 legs.
*/
pub fn svpwm(i:I6F10,j:I6F10,k:I6F10,max:I6F10) -> (I6F10,I6F10,I6F10,u8){
    let mut N:u8=0;
    let mut sector:u8;
    let (T0,T1,T2,U,V,W);

    if i>= I6F10::ZERO {N=N+1;}
    if j>= I6F10::ZERO {N=N+2;}
    if k>= I6F10::ZERO {N=N+4;}
    match N {
        1=>{
            sector=6;
            T1=I6F10::from_num(-j);
            T2=I6F10::from_num(-k);
            T0=max-T1-T2;
            U=T1+T2+T0/2;
            V=T0/2;
            W=T1+T0/2;
        }
        2=>{
            sector=2;
            T1=I6F10::from_num(-k);
            T2=I6F10::from_num(-i);
            T0=max-T1-T2;
            U=T1+T0/2;
            V=T1+T2+T0/2;
            W=T0/2;
        }
        3=>{
            sector=1;
            T1=I6F10::from_num(i);
            T2=I6F10::from_num(j);
            T0=max-T1-T2;
            U=T1+T2+T0/2;
            V=T2+T0/2;
            W=T0/2;
        }
        4=>{
            sector=4;
            T1=I6F10::from_num(-i);
            T2=I6F10::from_num(-j);
            T0=max-T1-T2;
            U=T0/2;
            V=T1+T0/2;
            W=T1+T2+T0/2;
        }
        5=>{
            sector=5;
            T1=I6F10::from_num(k);
            T2=I6F10::from_num(i);
            T0=max-T1-T2;
            U=T2+T0/2;
            V=T0/2;
            W=T1+T2+T0/2;
        }
        6=>{
            sector=3;
            T1=I6F10::from_num(j);
            T2=I6F10::from_num(k);
            T0=max-T1-T2;
            U=T0/2;
            V=T1+T2+T0/2;
            W=T2+T0/2;
        }
        _=>{U=I6F10::ZERO;V=I6F10::ZERO;W=I6F10::ZERO;sector=0;T0=U;T1=V;T2=W;}
    }
    //println!("T0={} T1={} T2={}",T0,T1,T2);
    return(U,V,W,sector)
}