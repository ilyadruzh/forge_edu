/* tslint:disable */
import * as wasm from './types_bg';

/**
* @param {Complexf32} arg0
* @param {Complexf32} arg1
* @returns {Complexf32}
*/
export function mul(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf32.__wrap(wasm.mul(ptr0, ptr1));
}

/**
* @param {Complexf32} arg0
* @param {Complexf32} arg1
* @returns {Complexf32}
*/
export function add(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf32.__wrap(wasm.add(ptr0, ptr1));
}

/**
* @param {Complexf32} arg0
* @param {Complexf32} arg1
* @returns {Complexf32}
*/
export function sub(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf32.__wrap(wasm.sub(ptr0, ptr1));
}

/**
* @param {Complexf32} arg0
* @param {number} arg1
* @returns {Complexf32}
*/
export function sub_f32(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return Complexf32.__wrap(wasm.sub_f32(ptr0, arg1));
}

/**
* @param {Complexf32} arg0
* @returns {number}
*/
export function abs(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.abs(ptr0);
}

/**
* @param {Complexf32} arg0
* @returns {number}
*/
export function arg(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.arg(ptr0);
}

/**
* @param {Complexf32} arg0
* @param {number} arg1
* @returns {Complexf32}
*/
export function scale(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return Complexf32.__wrap(wasm.scale(ptr0, arg1));
}

/**
* @param {Complexf32} arg0
* @returns {number}
*/
export function norm_sqr(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.norm_sqr(ptr0);
}

/**
* @param {Complexf32} arg0
* @param {Complexf32} arg1
* @returns {Complexf32}
*/
export function div(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf32.__wrap(wasm.div(ptr0, ptr1));
}

/**
* @param {Complexf64} arg0
* @param {Complexf64} arg1
* @returns {Complexf64}
*/
export function mul_f64(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf64.__wrap(wasm.mul_f64(ptr0, ptr1));
}

/**
* @param {Complexf64} arg0
* @param {Complexf64} arg1
* @returns {Complexf64}
*/
export function add_f64(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf64.__wrap(wasm.add_f64(ptr0, ptr1));
}

/**
* @param {Complexf64} arg0
* @param {Complexf64} arg1
* @returns {Complexf64}
*/
export function sub_from_f64(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf64.__wrap(wasm.sub_from_f64(ptr0, ptr1));
}

/**
* @param {Complexf64} arg0
* @param {number} arg1
* @returns {Complexf64}
*/
export function sub_f64(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return Complexf64.__wrap(wasm.sub_f64(ptr0, arg1));
}

/**
* @param {Complexf64} arg0
* @returns {number}
*/
export function abs_f64(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.abs_f64(ptr0);
}

/**
* @param {Complexf64} arg0
* @returns {number}
*/
export function arg_64(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.arg_64(ptr0);
}

/**
* @param {Complexf64} arg0
* @param {number} arg1
* @returns {Complexf64}
*/
export function scale_f64(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return Complexf64.__wrap(wasm.scale_f64(ptr0, arg1));
}

/**
* @param {Complexf64} arg0
* @returns {number}
*/
export function norm_sqr_f64(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm.norm_sqr_f64(ptr0);
}

/**
* @param {Complexf64} arg0
* @param {Complexf64} arg1
* @returns {Complexf64}
*/
export function div_f64(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    return Complexf64.__wrap(wasm.div_f64(ptr0, ptr1));
}

function freeComplexf32(ptr) {

    wasm.__wbg_complexf32_free(ptr);
}
/**
*/
export class Complexf32 {

    static __wrap(ptr) {
        const obj = Object.create(Complexf32.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeComplexf32(ptr);
    }

    /**
    * @returns {number}
    */
    get re() {
        return wasm.__wbg_get_complexf32_re(this.ptr);
    }
    set re(arg0) {
        return wasm.__wbg_set_complexf32_re(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get im() {
        return wasm.__wbg_get_complexf32_im(this.ptr);
    }
    set im(arg0) {
        return wasm.__wbg_set_complexf32_im(this.ptr, arg0);
    }
}

function freeComplexf64(ptr) {

    wasm.__wbg_complexf64_free(ptr);
}
/**
*/
export class Complexf64 {

    static __wrap(ptr) {
        const obj = Object.create(Complexf64.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeComplexf64(ptr);
    }

    /**
    * @returns {number}
    */
    get re() {
        return wasm.__wbg_get_complexf64_re(this.ptr);
    }
    set re(arg0) {
        return wasm.__wbg_set_complexf64_re(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get im() {
        return wasm.__wbg_get_complexf64_im(this.ptr);
    }
    set im(arg0) {
        return wasm.__wbg_set_complexf64_im(this.ptr, arg0);
    }
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

