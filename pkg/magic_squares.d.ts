/* tslint:disable */
/* eslint-disable */

/**
 * Represents the result of a magic square generation.
 * This struct is exported to WASM, allowing Javascript to access the grid and its order.
 */
export class MagicSquareResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Returns a copy of the flattened grid to Javascript.
     * Note: Cloning is necessary because we are passing ownership across the WASM boundary.
     */
    readonly grid: Uint32Array;
    /**
     * Returns the order (n) of the square.
     */
    readonly n: number;
}

/**
 * Main entry point for generating a magic square from Javascript.
 *
 * # Arguments
 *
 * * `n` - The order of the magic square to generate.
 *
 * # Returns
 *
 * * `Result<MagicSquareResult, JsError>` - The generated result, or an error if generation fails.
 */
export function generate_magic_square(n: number): MagicSquareResult;

/**
 * Verifies if a given grid is a valid magic square.
 * This function is exported to allow client-side verification if needed.
 */
export function verify_magic_square(n: number, flat_grid: Uint32Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_magicsquareresult_free: (a: number, b: number) => void;
    readonly generate_magic_square: (a: number, b: number) => void;
    readonly magicsquareresult_grid: (a: number, b: number) => void;
    readonly magicsquareresult_n: (a: number) => number;
    readonly verify_magic_square: (a: number, b: number, c: number) => number;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export2: (a: number, b: number) => number;
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
