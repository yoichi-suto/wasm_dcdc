use ccal::{CCurrent};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
mod traits;
mod plot;
mod ccal;

pub type DrawResult<T>=Result<T,Box<dyn std::error::Error>>;

#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

#[wasm_bindgen]
pub struct Point{
    pub x:f64,
    pub y:f64
}

#[wasm_bindgen]
pub struct WasmDcCalResult {
    pub duty: f64,
    pub period: f64,
    pub ontime: f64,
    pub dil_on: f64,
    pub dil_off: f64,
    pub current_top: f64,
    pub current_bottom: f64,
    pub intercept_on: f64,
    pub intercept_off: f64,
    pub ro: f64,
    pub rfb1: f64,
    pub fz: f64,
    pub cfb: f64,
    pub dvout: f64,
}

#[wasm_bindgen]
impl WasmDcCalResult {
    pub fn new(duty: f64, period: f64, ontime: f64, dil_on: f64, dil_off: f64, current_top: f64, current_bottom: f64, intercept_on: f64, intercept_off: f64, ro: f64, rfb1: f64, fz: f64, cfb: f64, dvout: f64) -> Self {
        WasmDcCalResult {
            duty,
            period,
            ontime,
            dil_on,
            dil_off,
            current_top,
            current_bottom,
            intercept_on,
            intercept_off,
            ro,
            rfb1,
            fz,
            cfb,
            dvout,
        }
    }
}

#[wasm_bindgen]
pub fn wasm_dccal_result(vin: f64, vout: f64, iout: f64, fsw: f64, ind: f64, cout: f64, rfb2: f64) -> WasmDcCalResult {
    //for comment_box, wave_chart
    let duty = vout / vin;
    let period = 1.0 / fsw;
    let ontime = duty * period;
    let dil_on = (vin - vout) / ind;
    let dil_off = (-1.0 * vout) / ind;
    let current_top = iout + dil_on * ontime / 2.0;
    let current_bottom = iout - dil_on * ontime / 2.0;
    let intercept_on = iout - dil_on * ontime / 2.0;
    let intercept_off = current_top - dil_off * ontime;
    let ro = vout / iout;
    let rfb1 =  rfb2 * (vout / 0.8 - 1.0);
    let fz = 3.94 * (1.0 / cout) * (0.8 / vout);
    let cfb =  1.0 / (2.0 * 3.14159265358979323846 * rfb1 * fz);
    let dvout = (current_top - current_bottom) / (8.0 * fsw * cout);
    WasmDcCalResult::new(duty, period, ontime, dil_on, dil_off, current_top, current_bottom, intercept_on, intercept_off, ro, rfb1, fz, cfb, dvout)
}

//Chart tran_coil_current
#[wasm_bindgen]
impl Chart{
    pub fn coil_current(canvas:HtmlCanvasElement, dil_on:f64, dil_off:f64, intercept_on:f64, intercept_off:f64, ontime_cha:f64, period:f64, _time:f64, unit_prefix:&str)->Result<(),JsValue>{
        let c_current=CCurrent{dil_on:dil_on, dil_off:dil_off, intercept_on:intercept_on, intercept_off:intercept_off, ontime_cha:ontime_cha};
        plot::draw(canvas, c_current, period, 2.5, unit_prefix).map_err(|err|err.to_string())?;
        Ok(())
    }
}

#[wasm_bindgen]
pub struct WasmEffCalResult {
    /*  pub vin: f64,
    pub vout: f64,
    pub fsw: f64,
    pub ind: f64,
    pub rl: f64,
    pub rfb1: f64,
    pub rfb2: f64,
    pub ron: f64,
    pub rd: f64,
    pub tr: f64,
    pub tf: f64,
    pub tdd1: f64,
    pub tdd2: f64,
    pub Qg1: f64,
    pub Qg2: f64, */
    xs: Vec<f64>,
    ys: Vec<f64>,
}

#[wasm_bindgen]
impl WasmEffCalResult {
    #[wasm_bindgen(constructor)]
    pub fn new(xs: Vec<f64>, ys: Vec<f64>) -> WasmEffCalResult {
        WasmEffCalResult { xs, ys }
    }

    // Getter for xs
    #[wasm_bindgen]
    pub fn get_xs(&self) -> Vec<f64> {
        self.xs.clone()
    }

    // Getter for ys
    #[wasm_bindgen]
    pub fn get_ys(&self) -> Vec<f64> {
        self.ys.clone()
    }
}

#[wasm_bindgen]
pub fn wasm_eff_result(
    vin: f64,
    vout: f64,
    fsw: f64,
    ind: f64,
    rl: f64,
    rfb1: f64,
    rfb2: f64,
    ron: f64,
    rd: f64,
    tr: f64,
    tf: f64,
    tdd1: f64,
    tdd2: f64,
    qg1: f64,
    qg2: f64,
) -> WasmEffCalResult {
    let mut xs = Vec::new(); // Io1 の値を保存
    let mut ys = Vec::new(); // P_xxx の合計値を保存

    for ix in 0..=40 {
        let io1 = 10.0f64.powf(ix as f64 / 10.0) / 1000.0; // 修正
        xs.push(io1); // Io1 の値を記録

        let vd = 0.0; //LSのダイオード成分
        let iq = 0.001;
        let ro = vout / io1; //出力抵抗
        let d = -((rl + rd + ro) * vout + ro * vd) / ((ron - rd) * vout - ro * vd - vin * ro);

        let ilpp = d / fsw / ind * (-(ron + rl) * io1 - vout + vin);
        let itop = io1 + ilpp / 2.0;
        let ibot = io1 - ilpp / 2.0;
        let iq_fb = d * vout / (rfb1 + rfb2);

        let _p_iqg1 = fsw * (qg1 + qg2) * 5.0 / 5.0; //bias en
        let p_iqg2 = fsw * (qg1 + qg2) * vout / 5.0 * vout / vin / 0.85; //bias disen
        let iq = iq * vout / vin / 0.85;

        let p_rl = (io1.powf(2.0) + (1.0 / 3.0) * (ilpp / 2.0)).powf(2.0) * rl;
        let p_ron = (io1.powf(2.0) + (1.0 / 3.0) * (ilpp / 2.0)).powf(2.0) * d * ron;
        let p_vd = (io1.powf(2.0) + (1.0 / 3.0) * (ilpp / 2.0)).sqrt().powf(2.0) * (1.0 - d) * vd; //vd=0
        let p_rd = (io1.powf(2.0) + (1.0 / 3.0) * (ilpp / 2.0)).powf(2.0) * (1.0 - d) * rd;
        let p_tdd1 = itop.abs() * 0.7 * tdd1 * fsw;
        let p_tdd2 = ibot.abs() * 0.7 * tdd2 * fsw;
        let p_tr = itop * vin / 6.0 * tr * fsw;
        let p_tf = itop * vin / 6.0 * tf * fsw;

        let p_iqfb = iq_fb * vin;
        let p_iqg = p_iqg2 * vin;
        let p_iq = iq * vin;

        let total_ploss =
            p_rl + p_ron + p_vd + p_rd + p_tdd1 + p_tdd2 + p_tr + p_tf + p_iqfb + p_iqg + p_iq;
        let eff = (io1 * vout) / (io1 * vout + total_ploss) * 100.0;
        ys.push(eff); // P_xxx の合計を記録
    }

    WasmEffCalResult::new(xs, ys)
}
