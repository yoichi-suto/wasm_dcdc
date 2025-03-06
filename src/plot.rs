use crate::traits::Filter;
use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;

pub fn draw<T:Filter>(canvas:HtmlCanvasElement, c_current:T, period:f64, time:f64, unit_prefix:&str)->DrawResult<()>{
    let _font=("sans-serif",5.0).into_font();
    let area=CanvasBackend::with_canvas_object(canvas).unwrap()
        .into_drawing_area();
    area.fill(&WHITE)?;
    let _x_param=Coord::new(time, "");
/*     let x_max=period;
    let x_axis=(0f64..x_max).step(x_param.get_step()); */
    let x_max = period;
    let x_step = x_max / 100.0; // ★ ステップ幅を適切に設定（調整可能）s
    let x_axis = (0f64..x_max).step(x_step); // ★ x_max までしっかり描画

/*  let y_min = 0.0;
    let y_max = (0..=time)
    .map(|time_value| c_current.calc_time_response(Coord::new(time_value as f64, unit_prefix).get_value()))
    .fold(f64::MIN, f64::max) * 1.1; // ★ 最大値の 1.1 倍で余裕を持たせる
    let y_axis = y_min..y_max; // ★ 適切な範囲を設定 */
    let y_axis=0.0..2.0;
    
    let mut chart=ChartBuilder::on(&area)
/*      .caption("time", font) */
        .x_label_area_size(30)
        .y_label_area_size(50)
        .margin(15)
        .build_cartesian_2d(x_axis, y_axis)?;
    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_desc(format!("time[{}s]",unit_prefix))
        .y_desc("Inductor_Current[A]")
        .label_style(("sans-serif", 10).into_font())
        .x_label_formatter(&|x| format!("{:.2}", x))  // ★ 小数点3桁にフォーマット
        .y_label_formatter(&|y| format!("{:.2}", y)) 
        .draw()?;

    //描写
    chart.draw_series(LineSeries::new(
        (0..=1000).map(|i| {
            let time_f64 = (i as f64) * (period / 1000.0); // ★ 1000 分割して小数ステップに
            let coord_value = Coord::new(time_f64, unit_prefix).get_value();
            let response = c_current.calc_time_response(coord_value);
            /* console::log_1(&format!("time_value: {}, coord_value: {}, response: {}", time_f64, coord_value, response).into());*/
            (time_f64, response)
        }).collect::<Vec<(f64, f64)>>(), 
        &BLUE
    ))?;

    area.present()?;
    Ok(())
}

pub struct Coord{
    value:f64,
    step:f64
}
impl Coord{
    pub fn new(value:f64,unit_prefix:&str)->Self{
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
}
 
#[wasm_bindgen]
pub fn chart2(xs: Vec<f64>, ys: Vec<f64>, iout:f64) {
    let canvas_id = "canvas2";
    let _font=("sans-serif",5.0).into_font();
    let backend = CanvasBackend::new(canvas_id).expect("canvas2 が見つかりません");
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let min_x = 0.000999999;
    let max_x = iout.ceil();
    let min_y = 0.0;
    let max_y = 100.0;

    let mut chart = ChartBuilder::on(&root)
        /* .caption("eff", ("sans-serif", 20)) */
        .x_label_area_size(30)
        .y_label_area_size(35)
        .margin(15)
        .build_cartesian_2d((min_x..max_x).log_scale(), min_y..max_y)
        .unwrap();

         // 軸ラベルを追加
        chart.configure_mesh()
        .x_desc("Iout[A]")    // X軸ラベル
        .y_desc("Efficiency[%]") // Y軸ラベル
        .label_style(("sans-serif", 10).into_font())
        
        .y_label_formatter(&|y| format!("{:.0}", y))
        .draw()
        .unwrap();

    // iout 以下のデータのみフィルタリング
    let filtered_data: Vec<(f64, f64)> = xs.iter()
        .zip(ys.iter())
        .filter(|(&x, _)| x <= iout) // ここでフィルタリング
        .map(|(&x, &y)| (x, y))
        .collect();

    // フィルタリング後のデータをプロット
    chart.draw_series(LineSeries::new(filtered_data, &RED))
        .unwrap()
        .label("PWM")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().border_style(&WHITE).draw().unwrap();

    root.present().unwrap();
}
