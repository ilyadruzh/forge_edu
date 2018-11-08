/* tslint:disable */
import * as wasm from './newton_fractal_bg';

const __wbg_log_b7d39fee430a22e0_target = console.log;

export function __wbg_log_b7d39fee430a22e0(arg0) {
    __wbg_log_b7d39fee430a22e0_target(arg0);
}
/**
* @param {number} arg0
* @returns {void}
*/
export function greet(arg0) {
    return wasm.greet(arg0);
}

/**
* @param {Complex} arg0
* @returns {number}
*/
export function norm_sqr(arg0) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error('Attempt to use a moved value');
    }
    arg0.ptr = 0;
    return wasm.norm_sqr(ptr0);
}

/**
* @param {Complex} arg0
* @param {Complex} arg1
* @returns {Complex}
*/
export function mul(arg0, arg1) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error('Attempt to use a moved value');
    }
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    if (ptr1 === 0) {
        throw new Error('Attempt to use a moved value');
    }
    arg1.ptr = 0;
    return Complex.__wrap(wasm.mul(ptr0, ptr1));
}

/**
* @param {Complex} arg0
* @param {Complex} arg1
* @returns {Complex}
*/
export function add(arg0, arg1) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error('Attempt to use a moved value');
    }
    arg0.ptr = 0;
    const ptr1 = arg1.ptr;
    if (ptr1 === 0) {
        throw new Error('Attempt to use a moved value');
    }
    arg1.ptr = 0;
    return Complex.__wrap(wasm.add(ptr0, ptr1));
}

/**
* @param {number} arg0
* @param {number} arg1
* @param {number} arg2
* @param {number} arg3
* @param {number} arg4
* @param {number} arg5
* @param {number} arg6
* @param {number} arg7
* @param {number} arg8
* @returns {void}
*/
export function draw(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    return wasm.draw(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
}

function freeComplex(ptr) {

    wasm.__wbg_complex_free(ptr);
}
/**
*/
export class Complex {

    static __wrap(ptr) {
        const obj = Object.create(Complex.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeComplex(ptr);
    }

}

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

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

