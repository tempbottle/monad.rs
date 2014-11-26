#![crate_name="monad"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://epsilonz.github.io/monad.rs/doc/monad/")]

#![feature(phase)]
#![feature(unboxed_closures)]

//! This crate implements various monad structures.

#[phase(link, plugin)]
extern crate free_macros;
extern crate free;
extern crate tailrec;

pub mod monad;
