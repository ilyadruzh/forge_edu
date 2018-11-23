/* tslint:disable */
export function mul(arg0: Complexf32, arg1: Complexf32): Complexf32;

export function add(arg0: Complexf32, arg1: Complexf32): Complexf32;

export function sub(arg0: Complexf32, arg1: Complexf32): Complexf32;

export function sub_f32(arg0: Complexf32, arg1: number): Complexf32;

export function abs(arg0: Complexf32): number;

export function arg(arg0: Complexf32): number;

export function scale(arg0: Complexf32, arg1: number): Complexf32;

export function norm_sqr(arg0: Complexf32): number;

export function div(arg0: Complexf32, arg1: Complexf32): Complexf32;

export function mul_f64(arg0: Complexf64, arg1: Complexf64): Complexf64;

export function add_f64(arg0: Complexf64, arg1: Complexf64): Complexf64;

export function sub_from_f64(arg0: Complexf64, arg1: Complexf64): Complexf64;

export function sub_f64(arg0: Complexf64, arg1: number): Complexf64;

export function abs_f64(arg0: Complexf64): number;

export function arg_64(arg0: Complexf64): number;

export function scale_f64(arg0: Complexf64, arg1: number): Complexf64;

export function norm_sqr_f64(arg0: Complexf64): number;

export function div_f64(arg0: Complexf64, arg1: Complexf64): Complexf64;

export class Complexf64 {
free(): void;
re: number
im: number

}
export class Complexf32 {
free(): void;
re: number
im: number

}
