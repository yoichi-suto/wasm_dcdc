use crate::traits::Filter;
use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;



/* pub fn draw<T:Filter>(canvas:HtmlCanvasElement,lpf_rc:T,time:i32,unit_prefix:&str)->DrawResult<()>{
    let font=("sans-serif",20.0).into_font();
    let area=CanvasBackend::with_canvas_object(canvas).unwrap()
        .into_drawing_area();
    area.fill(&WHITE)?;
    let x_param=Coord::new(time, "");
    let x_max=x_param.get_value();
    let x_axis=(0f64..x_max).step(x_param.get_step());
    let y_axis=0.0..1.0;
    let mut chart=ChartBuilder::on(&area)
        .caption("時間応答", font)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(30)
        .build_cartesian_2d(x_axis, y_axis)?;
    chart.configure_mesh().x_labels(3).y_labels(3).x_desc(format!("時間[{}s]",unit_prefix)).y_desc("電圧[V]").draw()?;
    chart.draw_series(LineSeries::new(
            (0..=time).map(|time_value|{
                (time_value as f64,lpf_rc.calc_time_response(Coord::new(time_value,unit_prefix).get_value()))
            }).collect::<Vec<(f64,f64)>>(),&RED
    ))?;
    area.present()?;
    Ok(())
}
pub struct Coord{
    value:f64,
    step:f64
}
impl Coord{
    pub fn new(value:i32,unit_prefix:&str)->Self{
        let value=value as f64;
        let (value,step)=match unit_prefix{
            "k"=>(value*10f64.powi(3),10f64.powi(3)),
            "m"=>(value*10f64.powi(-3),10f64.powi(-3)),
            "u"=>(value*10f64.powi(-6),10f64.powi(-6)),
            "n"=>(value*10f64.powi(-9),10f64.powi(-9)),
            _=>(value,1.0)
        };
        Self{value:value,step:step}
    }
    pub fn get_value(&self)->f64{
        self.value
    }
    pub fn get_step(&self)->f64{
        self.step
    }
} */