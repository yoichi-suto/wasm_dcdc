/* tslint:disable */
/* eslint-disable */
export function wasm_dccal_result(vin: number, vout: number, iout: number, fsw: number, ind: number, cout: number, rfb2: number): WasmDcCalResult;
export class Chart {
  private constructor();
  free(): void;
  static coil_current(canvas: HTMLCanvasElement, dil_on: number, dil_off: number, intercept_on: number, intercept_off: number, ontime_cha: number, period: number, time: number, unit_prefix: string): void;
}
export class Point {
  private constructor();
  free(): void;
  x: number;
  y: number;
}
export class WasmDcCalResult {
  private constructor();
  free(): void;
  static new(duty: number, period: number, ontime: number, dil_on: number, dil_off: number, current_top: number, current_bottom: number, intercept_on: number, intercept_off: number, ro: number, rfb1: number, fz: number, cfb: number, dvout: number): WasmDcCalResult;
  duty: number;
  period: number;
  ontime: number;
  dil_on: number;
  dil_off: number;
  current_top: number;
  current_bottom: number;
  intercept_on: number;
  intercept_off: number;
  ro: number;
  rfb1: number;
  fz: number;
  cfb: number;
  dvout: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chart_free: (a: number, b: number) => void;
  readonly __wbg_point_free: (a: number, b: number) => void;
  readonly __wbg_get_point_x: (a: number) => number;
  readonly __wbg_set_point_x: (a: number, b: number) => void;
  readonly __wbg_get_point_y: (a: number) => number;
  readonly __wbg_set_point_y: (a: number, b: number) => void;
  readonly __wbg_wasmdccalresult_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_ontime: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_ontime: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_dil_on: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_dil_on: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_dil_off: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_dil_off: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_current_top: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_current_top: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_current_bottom: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_current_bottom: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_intercept_on: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_intercept_on: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_intercept_off: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_intercept_off: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_ro: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_ro: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_rfb1: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_rfb1: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_fz: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_fz: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_cfb: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_cfb: (a: number, b: number) => void;
  readonly __wbg_get_wasmdccalresult_dvout: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_dvout: (a: number, b: number) => void;
  readonly wasmdccalresult_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => number;
  readonly wasm_dccal_result: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly chart_coil_current: (a: any, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => [number, number];
  readonly __wbg_get_wasmdccalresult_duty: (a: number) => number;
  readonly __wbg_get_wasmdccalresult_period: (a: number) => number;
  readonly __wbg_set_wasmdccalresult_duty: (a: number, b: number) => void;
  readonly __wbg_set_wasmdccalresult_period: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
