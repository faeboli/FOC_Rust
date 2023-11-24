
use fixed::types::{I2F14, I6F10};
use fixed::types::I4F12;
mod FOC_func;
use std::io;
use plotters::prelude::*;

fn main() -> ! {
    let mut angle: I4F12;
    
    let resolution = 12-7; // 2^-7
    let mut sinval: I2F14;
    let startangle: i32 = -3140;
    let endangle: i32 = 3140;

    loop {
        let mut buffer = String::new();
        println!("Enter command 'speed' or 'angle' or 'plot'");
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.as_str() {
            "speed\n" => {
                println!("Set speed:");
                buffer="".to_string();
                io::stdin().read_line(&mut buffer).unwrap();
                let speed=buffer.trim().parse::<f32>().unwrap();
                println!("speed request: {}",speed);
                for j in 1..1000 {
                    angle = I4F12::from_num(-3.14);
                    for i in startangle..endangle {
                        //sinval = FOC_func::table_trig::sin_t(angle, &sin_table , resolution); // about 400ns @ 64MHz
                        angle += I4F12::from_num(0.00101);
                    }
                }
            }
            "angle\n" => {
                println!("Set angle:");
                buffer="".to_string();
                io::stdin().read_line(&mut buffer).unwrap();
                let theta=buffer.trim().parse::<f32>().unwrap();
                println!("angle request: {}",theta);
                buffer="".to_string();
                println!("Set Vd:");
                io::stdin().read_line(&mut buffer).unwrap();
                let Vd=buffer.trim().parse::<f32>().unwrap();
                println!("Vd request: {}",Vd);
                println!("Set Vq:");
                buffer="".to_string();
                io::stdin().read_line(&mut buffer).unwrap();
                let Vq=buffer.trim().parse::<f32>().unwrap();
                println!("Vq request: {}",Vq);
                buffer="".to_string();
                io::stdin().read_line(&mut buffer).unwrap();
                let max=buffer.trim().parse::<f32>().unwrap();
                println!("Max V request: {}",max);
                let (Valpha,Vbeta)=FOC_func::inverse_park(I6F10::from_num(Vd), I6F10::from_num(Vq), I4F12::from_num(theta));
                println!("Va={},Vb={}",Valpha,Vbeta);
                let (i,j,k)=FOC_func::mod_inverse_clarke(Valpha, Vbeta);
                println!("i={},j={},k={}",i,j,k);
                let (U,V,W,sector)=FOC_func::svpwm(i,j,k,I6F10::from_num(max));
                println!("sector: {}",sector);
                println!("U={},V={},W={}",U,V,W);
            }
            "plot\n" => {
                println!("Generate plot:");
                buffer="".to_string();
                println!("Set Vd:");
                io::stdin().read_line(&mut buffer).unwrap();
                let Vd=buffer.trim().parse::<f32>().unwrap();
                println!("Vd request: {}",Vd);
                println!("Set Vq:");
                buffer="".to_string();
                io::stdin().read_line(&mut buffer).unwrap();
                let Vq=buffer.trim().parse::<f32>().unwrap();
                println!("Vq request: {}",Vq);
                println!("Max V:");
                buffer="".to_string();
                io::stdin().read_line(&mut buffer).unwrap();
                let max=buffer.trim().parse::<f32>().unwrap();
                println!("Max V request: {}",max);
                let startangle:i32=-314;
                let endangle:i32=314;
                let root_drawing_area = BitMapBackend::new("abs_UVW_plot.png", (1024, 768))
                .into_drawing_area();
                root_drawing_area.fill(&WHITE).unwrap();
                root_drawing_area.margin(10,10,10,10);
                let root_drawing_area_1 = BitMapBackend::new("linetoline_plot.png", (1024, 768))
                .into_drawing_area();
                root_drawing_area_1.fill(&WHITE).unwrap();
                root_drawing_area_1.margin(10,10,10,10);
                let mut chart = ChartBuilder::on(&root_drawing_area)
                .caption("U V W", ("sans-serif", 40).into_font())
                // Set the size of the label region
                .x_label_area_size(20)
                .y_label_area_size(40).build_cartesian_2d(-3.14..3.14, 0.0..f64::from(max))
                .unwrap();
                let mut chart_1 = ChartBuilder::on(&root_drawing_area_1)
                .caption("U-V V-W W-U", ("sans-serif", 40).into_font())
                // Set the size of the label region
                .x_label_area_size(20)
                .y_label_area_size(40).build_cartesian_2d(-3.14..3.14, -f64::from(max)..f64::from(max))
                .unwrap();
                chart.configure_mesh()
                //.x_labels(5)
                //.y_labels(5)
                .draw()
                .unwrap();
                chart_1.configure_mesh()
                //.x_labels(5)
                //.y_labels(5)
                .draw()
                .unwrap();
                let mut U_v=Vec::new();
                let mut V_v=Vec::new();
                let mut W_v=Vec::new();
                let mut UV_v=Vec::new();
                let mut VW_v=Vec::new();
                let mut WU_v=Vec::new();
                println!("Calculating...");
                for i in startangle..endangle{
                    let theta=(i as f32)/100.0;
                    let (Valpha,Vbeta)=FOC_func::inverse_park(I6F10::from_num(Vd), I6F10::from_num(Vq), I4F12::from_num(theta));
                    let (i,j,k)=FOC_func::mod_inverse_clarke(Valpha, Vbeta);
                    let (U,V,W,sector)=FOC_func::svpwm(i,j,k,I6F10::from_num(max));
                    U_v.push((theta as f64,f64::from(U)));
                    V_v.push((theta as f64,f64::from(V)));
                    W_v.push((theta as f64,f64::from(W)));
                    UV_v.push((theta as f64,f64::from(U-V)));
                    VW_v.push((theta as f64,f64::from(V-W)));
                    WU_v.push((theta as f64,f64::from(W-U)));
                }
                println!("Plotting...");
                chart.draw_series(LineSeries::new(U_v,&RED)).unwrap();                                    
                chart.draw_series(LineSeries::new(V_v,&GREEN)).unwrap();  
                chart.draw_series(LineSeries::new(W_v,&BLUE)).unwrap();  
                chart_1.draw_series(LineSeries::new(UV_v,&RED)).unwrap();                                    
                chart_1.draw_series(LineSeries::new(VW_v,&GREEN)).unwrap();  
                chart_1.draw_series(LineSeries::new(WU_v,&BLUE)).unwrap();
            }
            _ => {
                println!("Valid commands: 'speed' or 'angle' or 'plot'");
            }
        }
    }
}
