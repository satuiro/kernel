use std::{env, fs};

fn main() {
    let current_exe = env::current_exe().unwrap();
    let uefi_target = current_exe.with_file_name("uefi.img");
    let bios_target = current_exe.with_file_name("bios.img");

    fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();
    fs::copy(env!("BIOS_IMAGE"), &bios_target).unwrap();

    println!("UEFI image at {}", uefi_target.display());
    println!("BIOS image at {}", bios_target.display());
}

// in src/main.rs

#![feature(custom_test_frameworks)]
#[test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}