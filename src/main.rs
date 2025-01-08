#![allow(unused)]       // No bullshit warnings

mod args;               // Import the args.rs module

// use std::fs;         // Allow file system

use args::TunstallArgs; // Allow TunstallArgs struct from the args.rs module
use clap::Parser;       // Allow parser from clap

fn main() 
{  
   println!("Hello World!");
   let args:TunstallArgs = TunstallArgs::parse();
}