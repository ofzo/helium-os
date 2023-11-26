// use std::{env, fs};

use std::fs::{self, File};
use std::io::Write;

static TARGET_DIR: &str = "../std/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rerun-if-change=../std/src/");
    println!("cargo:rerun-if-change={}", TARGET_DIR);

    let mut f = File::create("./src/link_app.S").unwrap();
    let mut apps: Vec<_> = fs::read_dir("../std/src/bin")
        .unwrap()
        .into_iter()
        .map(|file| {
            let mut file_name = file.unwrap().file_name().into_string().unwrap();
            file_name.drain(file_name.find(".").unwrap()..file_name.len());
            file_name
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r###"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {};
    "###,
        apps.len()
    )
    .unwrap();

    for i in 0..apps.len() {
        writeln!(f, "    .quad app_{}_start", i).unwrap();
    }
    writeln!(f, "    .quad app_{}_end", apps.len() - 1).unwrap();

    for (id, app) in apps.iter().enumerate() {
        writeln!(
            f,
            r##"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:
"##,
            id, app, TARGET_DIR
        )
        .unwrap();
    }
}
