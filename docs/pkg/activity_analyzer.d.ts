/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
/**
* @param {string} s
*/
export function set_world_data(s: string): void;
/**
* @param {string} s
*/
export function set_us_data(s: string): void;
/**
* @param {string} s
* @returns {string}
*/
export function analyze_gpx(s: string): string;
/**
* @param {string} s
* @returns {string}
*/
export function analyze_tcx(s: string): string;
/**
* @param {Uint8Array} s
* @returns {string}
*/
export function analyze_fit(s: Uint8Array): string;
/**
* @param {string} format
* @param {number} split_start
* @param {number} split_end
* @returns {string}
*/
export function export_data(format: string, split_start: number, split_end: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: () => void;
  readonly set_world_data: (a: number, b: number) => void;
  readonly set_us_data: (a: number, b: number) => void;
  readonly analyze_gpx: (a: number, b: number, c: number) => void;
  readonly analyze_tcx: (a: number, b: number, c: number) => void;
  readonly analyze_fit: (a: number, b: number, c: number) => void;
  readonly export_data: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
