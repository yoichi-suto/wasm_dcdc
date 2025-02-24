use ccal::{CCurrent};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
mod traits;
mod plot;
mod ccal;

pub type DrawResult<T>=Result<T,Box<dyn std::error::Error>>;

#[wasm_bindgen]
pub struct Chart{
    convert:Box<dyn Fn((i32,i32))->Option<(f64,f64)>>,
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
    pub fn coil_current(canvas:HtmlCanvasElement, dil_on:f64, dil_off:f64, intercept_on:f64, intercept_off:f64, ontime_cha:f64, period:f64, time:f64, unit_prefix:&str)->Result<(),JsValue>{
        let c_current=CCurrent{dil_on:dil_on, dil_off:dil_off, intercept_on:intercept_on, intercept_off:intercept_off, ontime_cha:ontime_cha};
        plot::draw(canvas, c_current, period, 2.5, unit_prefix).map_err(|err|err.to_string())?;
        Ok(())
    }
}

/* 
#[wasm_bindgen]
pub struct WasmEffCalResult {
    pub tr1: f64,
    pub tr2: f64,
    pub tdd1: f64,
    pub tdd2: f64,
    pub Qg1: f64,
    pub Qg2: f64,
} */


/* #[wasm_bindgen]
pub fn wasm_eff_result(vin: f64, vout: f64, iout: f64, fsw: f64, ind: f64) -> WasmDcCalResult {
/*     R = Vo /Io
    D = - ((RL + RD ++R )* vout +R * Vd )/ ((Ron -Rd) *Vo-Vg *R) */
    let P_IRFB = duty * vout / (RFB1 + RFB2);
    let P_IQg1 = fsw * (Qg1 + Qg2) * VREG/5.0;
    let P_IQg2 = fsw * (Qg1 + Qg2) * vout/5.0 * vout/vin/0.85;
    let P_IQ = 0.001 * Vout / vin / 0.85;
    let P_RL = (iout^2.0 + 1.0/3.0 * current_ripple/2.0)^2 * RL;
    let P_RAC = ((vin - vout))^2.0 * duty + Vout^2.0 * (1-duty) / RAC;
    let P_RON = (iout^2.0 + 1.0/3.0 * current_ripple/2.0)^2 * Ron;
    let P_VD = sqrt(iout^2.0 + 1.0/3.0 * current_ripple/2.0)^2 * (1-duty) * VD;
    let P_RD = (iout^2.0 + 1.0/3.0 * current_ripple/2.0)^2 * (1-duty) * RD;
    let P_tdd1 = current_top * 0.7 * tdd1 * fsw;
    let P_tdd2 = current_bottom * 0.7 * tdd2 * fsw;
    let P_tr = current_top * vin/6.0 * tr * fsw;
    let P_tf = current_bottom * Vg/6.0 * tf * fsw;
    let P_IQFB = IQFB * vin;
    let P_IQg = IQg * vin;
    let P_IQ = IQ * vin;

    WasmDcCalResult::new(duty, period, ontime, dil_on, dil_off, current_top, current_bottom, intercept_on, intercept_off,)
} */ */