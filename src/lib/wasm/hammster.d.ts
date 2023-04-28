/* tslint:disable */
/* eslint-disable */
/**
* @param {number} k
* @returns {Uint8Array}
*/
export function setup_params(k: number): Uint8Array;
/**
* @param {Uint8Array} a
* @param {Uint8Array} b
* @param {Uint8Array} params_bytes
* @returns {Uint8Array}
*/
export function proof_generate(a: Uint8Array, b: Uint8Array, params_bytes: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} params_bytes
* @param {number} hamming_dist
* @param {Uint8Array} proof
* @returns {boolean}
*/
export function proof_verify(params_bytes: Uint8Array, hamming_dist: number, proof: Uint8Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly setup_params: (a: number) => number;
  readonly proof_generate: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly proof_verify: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_thread_destroy: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput, maybe_memory?: WebAssembly.Memory): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
* @param {WebAssembly.Memory} maybe_memory
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>, maybe_memory?: WebAssembly.Memory): Promise<InitOutput>;
