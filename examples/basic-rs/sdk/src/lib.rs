#![no_std]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

mod host_fns;
mod map;
mod or_abort;
mod rt;
mod vec;
mod bignum;
mod object;

mod symbol;
mod val;

pub use map::Map;
pub use vec::Vec;
pub use or_abort::OrAbort;
pub use val::{BitSet, Symbol, Val, Status};
pub use object::Object;
pub use bignum::BigNum;

#[inline(always)]
pub fn require(b: bool) {
    b.or_abort();
}

#[inline(always)]
pub fn log_value(v: Val) -> Val {
    unsafe { host_fns::host__log_value(v) }
}

#[inline(always)]
pub fn store_set(k: Val, v: Val) {
    unsafe { host_fns::host__store_set(k, v) }
}

#[inline(always)]
pub fn store_get(k: Val) -> Val {
    unsafe { host_fns::host__store_get(k) }
}

#[inline(always)]
pub fn get_current_ledger_num() -> u32 {
    unsafe { host_fns::host__get_current_ledger_num().as_u32() }
}

#[inline(always)]
pub fn create_account(src: Val, dst: Val, starting_balance: Val) -> Val {
    unsafe { host_fns::host__create_account(src, dst, starting_balance) }
}

#[inline(always)]
pub fn create_trustline(src: Val, asset: Val) -> Val {
    unsafe { host_fns::host__create_trustline(src, asset) }
}

#[inline(always)]
pub fn balance(acc: Val, asset: Val) -> Val {
    unsafe { host_fns::host__get_balance(acc, asset) }
}

#[inline(always)]
pub fn pay(src: Val, dst: Val, asset: Val, amount: Val) -> Val {
    unsafe { host_fns::host__pay(src, dst, asset, amount) }
}

#[inline(always)]
pub fn call0(contract: Val, func: Symbol) -> Val {
    unsafe { host_fns::host__call0(contract, func.into()) }
}

#[inline(always)]
pub fn call1(contract: Val, func: Symbol, a: Val) -> Val {
    unsafe { host_fns::host__call1(contract, func.into(), a) }
}

#[inline(always)]
pub fn call2(contract: Val, func: Symbol, a: Val, b: Val) -> Val {
    unsafe { host_fns::host__call2(contract, func.into(), a, b) }
}

#[inline(always)]
pub fn call3(contract: Val, func: Symbol, a: Val, b: Val, c: Val) -> Val {
    unsafe { host_fns::host__call3(contract, func.into(), a, b, c) }
}

#[inline(always)]
pub fn call4(contract: Val, func: Symbol, a: Val, b: Val, c: Val, d: Val) -> Val {
    unsafe { host_fns::host__call4(contract, func.into(), a, b, c, d) }
}
