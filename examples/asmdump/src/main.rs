#![feature(asm)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn dump_stack() {
    let nr_elements = 70;
    for i in 0..nr_elements {
        let offset = nr_elements - i - 1;
        let mut result: u64;
        unsafe {
            asm!("movq %rsp, %rax
                  addq %rbx, %rax
                  movq (%rax), %rcx"
                 : "={rcx}"(result)
                 : "{rbx}"(8 * offset)
                 : "rax", "rbx", "rcx"
                 :)
        }
        println!("sp+{:03} ⇒ value {:016x}", 8 * offset, result);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(always)]
fn print_current_address() {
    // https://stackoverflow.com/a/52050776
    let mut result: u64;
    unsafe {
        asm!("leaq (%rip), %rax"
             : "={rax}"(result)
             :
             : "rax"
             :)
    }
    println!("rip = {:016x}", result);
}

fn sub(sub_arg: u64) -> u64 {
    print_current_address();
    let sub_local = 0xDEAD_C0DE;
    let sub_sum = sub_arg + sub_local;
    dump_stack();
    sub_sum
}

fn main() {
    let _main_a: u64 = 0xDEAD_BEEF;
    let main_arg: u64 = 0xFEED_C0DE;
    print_current_address();
    let main_ret = sub(main_arg);
    assert_eq!(main_ret, 0x0000_0001_DD9B_81BC);
    let s = sub as *const ();
    let m = main as *const ();
    println!("sub = {:016p}", s);
    println!("main = {:016p}", m);
    print_current_address();
}
