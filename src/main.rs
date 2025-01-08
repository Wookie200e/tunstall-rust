#![allow(unused)]       // No bullshit warnings

mod args;               // Import the args.rs module

// use std::fs;         // Allow file system

use args::TunstallArgs; // Allow TunstallArgs struct from the args.rs module
use clap::Parser;       // Allow parser from clap

fn main() 
{  
   let args = TunstallArgs::parse();      // Add the passed arguments into a struct called args
   
   let tunstall_width = args.tunstall_width;
   let file_name = args.file_name;

   println!("Tunstall width: {}", tunstall_width);
   println!("File name: {}", file_name);
}