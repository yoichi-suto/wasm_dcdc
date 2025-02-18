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
    let intercept_off = current_top - dil_off * ontime;
    WasmDcCalResult::new(duty, period, ontime, dil_on, dil_off, current_top, current_bottom, intercept_on, intercept_off,)
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
