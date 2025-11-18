declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	export function evaluate(options: EvaluateOptions): EvaluateResponse;
	export interface EvaluateResponse {
	    /**
	     * depth of search
	     */
	    depth: number;
	    /**
	     * number of times the evaluation function was executed
	     */
	    evaluations: number;
	    /**
	     * ordered list of possible sans, sorted by score best to worst
	     */
	    sans: ScoredSan[];
	}
	
	export interface SearchResult {
	    /**
	     * depth of search
	     */
	    depth: number;
	    /**
	     * number of times the evaluation function was executed
	     */
	    evaluations: number;
	    /**
	     * ordered list of possible sans, sorted by score best to worst
	     */
	    sans: ScoredSan[];
	}
	
	export interface ScoredSan {
	    /**
	     * fen of the position
	     */
	    san: San;
	    /**
	     * score of the position
	     */
	    score: number;
	}
	
	export interface ScoredSan {
	    /**
	     * fen of the position
	     */
	    san: San;
	    /**
	     * score of the position
	     */
	    score: number;
	}
	
	export interface EvaluateOptions {
	    depth: number;
	    position: string;
	}
	
	export interface EvalOptions {
	    bishop_value: number;
	    king_value: number;
	    knight_value: number;
	    pawn_value: number;
	    queen_value: number;
	    rook_value: number;
	    check_value: number;
	    checkmate_value: number;
	    stalemate_value: number;
	}
	
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly evaluate: (a: any) => any;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
