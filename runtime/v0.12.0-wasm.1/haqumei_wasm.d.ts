/* tslint:disable */
/* eslint-disable */

export class Dictionary {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    static from_bytes(system: Uint8Array, chars: Uint8Array, matrix: Uint8Array): Dictionary;
}

export class DictionaryBlobLoader {
    free(): void;
    [Symbol.dispose](): void;
    append(chunk: Uint8Array): void;
    expected_bytes(): number;
    finish(): Dictionary;
    constructor(system_len: number, char_len: number, matrix_len: number);
    received_bytes(): number;
}

export class Haqumei {
    free(): void;
    [Symbol.dispose](): void;
    extract_fullcontext(text: string): any;
    extract_fullcontext_batch(input: any): any;
    extract_fullcontext_string(text: string): any;
    extract_fullcontext_string_batch(input: any): any;
    static from_dictionary(dict: Dictionary, options?: any | null): Haqumei;
    g2k(text: string): string;
    g2k_batch(input: any): any;
    g2k_per_word(text: string): any;
    g2k_per_word_batch(input: any): any;
    g2p(text: string): any;
    g2p_batch(input: any): any;
    g2p_candidates(text: string, options?: any | null): any;
    g2p_candidates_batch(input: any, options?: any | null): any;
    g2p_candidates_detailed(text: string, options?: any | null): any;
    g2p_candidates_prosody(text: string, options?: any | null): any;
    g2p_detailed(text: string): any;
    g2p_detailed_batch(input: any): any;
    g2p_mapping(text: string): any;
    g2p_mapping_batch(input: any): any;
    g2p_mapping_detailed(text: string): any;
    g2p_mapping_detailed_batch(input: any): any;
    g2p_mapping_prosody(text: string): any;
    g2p_mapping_prosody_batch(input: any): any;
    g2p_per_word(text: string): any;
    g2p_per_word_batch(input: any): any;
    g2p_prosody(text: string, format?: string | null): any;
    g2p_prosody_batch(input: any, format?: string | null): any;
    constructor(options?: any | null);
    run_frontend(text: string): any;
    run_frontend_batch(input: any): any;
    run_frontend_detailed(text: string): any;
    run_frontend_detailed_batch(input: any): any;
}

export function capabilities(): any;

export function update_global_dictionary(dict: Dictionary): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_dictionary_free: (a: number, b: number) => void;
    readonly __wbg_dictionaryblobloader_free: (a: number, b: number) => void;
    readonly __wbg_haqumei_free: (a: number, b: number) => void;
    readonly capabilities: () => [number, number, number];
    readonly dictionary_from_bytes: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
    readonly dictionaryblobloader_append: (a: number, b: number, c: number) => [number, number];
    readonly dictionaryblobloader_expected_bytes: (a: number) => number;
    readonly dictionaryblobloader_finish: (a: number) => [number, number, number];
    readonly dictionaryblobloader_new: (a: number, b: number, c: number) => number;
    readonly dictionaryblobloader_received_bytes: (a: number) => number;
    readonly haqumei_extract_fullcontext: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_extract_fullcontext_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_extract_fullcontext_string: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_extract_fullcontext_string_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_from_dictionary: (a: number, b: number) => [number, number, number];
    readonly haqumei_g2k: (a: number, b: number, c: number) => [number, number, number, number];
    readonly haqumei_g2k_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2k_per_word: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2k_per_word_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2p_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p_candidates: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly haqumei_g2p_candidates_batch: (a: number, b: any, c: number) => [number, number, number];
    readonly haqumei_g2p_candidates_detailed: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly haqumei_g2p_candidates_prosody: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly haqumei_g2p_detailed: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2p_detailed_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p_mapping: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2p_mapping_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p_mapping_detailed: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2p_mapping_detailed_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p_mapping_prosody: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2p_mapping_prosody_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p_per_word: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_g2p_per_word_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_g2p_prosody: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
    readonly haqumei_g2p_prosody_batch: (a: number, b: any, c: number, d: number) => [number, number, number];
    readonly haqumei_new: (a: number) => [number, number, number];
    readonly haqumei_run_frontend: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_run_frontend_batch: (a: number, b: any) => [number, number, number];
    readonly haqumei_run_frontend_detailed: (a: number, b: number, c: number) => [number, number, number];
    readonly haqumei_run_frontend_detailed_batch: (a: number, b: any) => [number, number, number];
    readonly update_global_dictionary: (a: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
