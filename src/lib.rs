#![crate_name="monad"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/monad.rs/doc/monad/")]

#![feature(overloaded_calls)]
#![feature(phase)]
#![feature(unboxed_closures)]

//! This crate implements various monad structures.

#[phase(link, plugin)]
extern crate free;
extern crate tailrec;

pub mod monad;
