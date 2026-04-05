// Copyright (c) Facebook, Inc. and its affiliates.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use fastpay_core::{error, messages, serialize};
use serde_reflection::{Registry, Result, Samples, Tracer, TracerConfig};
use std::{fs::File, io::Write};

fn get_registry() -> Result<Registry> {
    let mut tracer = Tracer::new(TracerConfig::default());
    let samples = Samples::new();
    // 1. Record samples for types with custom deserializers.
    // tracer.trace_value(&mut samples, ...)?;

    // 2. Trace the main entry point(s) + every enum separately.
    tracer.trace_type::<messages::Address>(&samples)?;
    tracer.trace_type::<error::FastPayError>(&samples)?;
    tracer.trace_type::<serialize::SerializedMessage>(&samples)?;
    tracer.registry()
}

#[derive(Debug, clap::ValueEnum, Clone, Copy)]
enum Action {
    Print,
    Test,
    Record,
}

#[derive(Debug, Parser)]
#[command(
    name = "FastPay format generator",
    about = "Trace serde (de)serialization to generate format descriptions for FastPay types"
)]
struct Options {
    #[arg(value_enum, default_value_t = Action::Print)]
    action: Action,
}

const FILE_PATH: &str = "fastpay_core/tests/staged/fastpay.yaml";

fn main() {
    let options = Options::parse();
    let registry = get_registry().unwrap();
    match options.action {
        Action::Print => {
            let content = serde_yaml::to_string(&registry).unwrap();
            println!("{}", content);
        }
        Action::Record => {
            let content = serde_yaml::to_string(&registry).unwrap();
            let mut f = File::create(FILE_PATH).unwrap();
            writeln!(f, "{}", content).unwrap();
        }
        Action::Test => {
            let reference = std::fs::read_to_string(FILE_PATH).unwrap();
            let content = serde_yaml::to_string(&registry).unwrap() + "\n";
            assert_eq!(&reference, &content);
        }
    }
}
