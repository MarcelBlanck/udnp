// Copyright (C) 2021 Marcel Blanck
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    for arg in args.iter() {
        println!("{}", arg);
    }
    Ok(())
}
