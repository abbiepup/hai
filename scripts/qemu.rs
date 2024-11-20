#!/usr/bin/env cargo +nightly -Zscript

//! ```cargo
//! [package]
//! edition = "2024"
//! ```

use std::env::args;
use std::io::{Error, ErrorKind, Result};
use std::process::Command;

const CODE: &str = "./tools/ovmf/x86_64/code.fd";
const VARS: &str = "./tools/ovmf/x86_64/vars.fd";

fn main() -> Result<()> {
    let target = args().nth(1).ok_or(Error::new(
        ErrorKind::InvalidInput,
        "Missing target argument",
    ))?;

    let binary = args().nth(2).ok_or(Error::new(
        ErrorKind::InvalidInput,
        "Missing binary argument",
    ))?;

    let qemu = match target.as_str() {
        "aarch64-unknown-uefi" => "qemu-system-aarch64",
        "x86_64-unknown-uefi" => "qemu-system-x86_64",
        _ => return Err(Error::new(ErrorKind::InvalidInput, "Unsupported target")),
    };

    #[rustfmt::skip]
    Command::new(qemu)
        .args(&[
            "-serial", "stdio",
            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", CODE),
            "-drive", &format!("if=pflash,format=raw,readonly=on,file={}", VARS),
            "-kernel", &binary,
        ])
        .status()?;

    Ok(())
}
