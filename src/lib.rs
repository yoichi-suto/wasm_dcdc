use wasm_bindgen::prelude::*;
//use web_sys::HtmlCanvasElement;
//mod plot;

/* #[wasm_bindgen]
pub struct Chart{
    convert:Box<dyn Fn((i32,i32))->Option<(f64,f64)>>,
} */

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
}

#[wasm_bindgen]
impl WasmDcCalResult {
    pub fn new(duty: f64, period: f64, ontime: f64, dil_on: f64, dil_off: f64, current_top: f64, current_bottom: f64, intercept_on: f64, intercept_off: f64,) -> Self {
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
        }
    }
}

#[wasm_bindgen]
pub fn wasm_dccal_result(vin: f64, vout: f64, iout: f64, fsw: f64, ind: f64) -> WasmDcCalResult {
    let duty = vout / vin;
    let period = 1.0 / fsw;
    let ontime = duty * period;
    let dil_on = (vin - vout) / ind;
    let dil_off = (-1.0 * vout) / ind;
    let current_top = iout + dil_on * ontime / 2.0;
    let current_bottom = iout - dil_on * ontime / 2.0;
    let intercept_on = iout - dil_on * ontime / 2.0;
    let intercept_off = current_top - dil_off * ontime / 2.0;
    WasmDcCalResult::new(duty, period, ontime, dil_on, dil_off, current_top, current_bottom, intercept_on, intercept_off,)
}

//Chart tran_coil_current

