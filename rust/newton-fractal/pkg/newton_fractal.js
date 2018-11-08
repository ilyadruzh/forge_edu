/* tslint:disable */
import * as wasm from './newton_fractal_bg';

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

export function __wbg_alert_3aba29641da247cc(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}
/**
* @returns {void}
*/
export function greet() {
    return wasm.greet();
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

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}
/**
* @param {number} arg0
* @param {number} arg1
* @param {BigInt} arg2
* @param {number} arg3
* @param {number} arg4
* @param {number} arg5
* @param {number} arg6
* @param {number} arg7
* @param {number} arg8
* @returns {void}
*/
export function draw(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {

    uint64CvtShim[0] = arg2;
    const low2 = u32CvtShim[0];
    const high2 = u32CvtShim[1];

    return wasm.draw(arg0, arg1, low2, high2, arg3, arg4, arg5, arg6, arg7, arg8);
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

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

