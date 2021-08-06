// Copyright (C) 2021 Marcel Blanck
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_doc_code_examples)]

#[cfg(test)]
mod tests {
    #[test]
    fn always_fail() {
        panic!("fail");
    }
}
