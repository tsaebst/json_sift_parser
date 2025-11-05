#![allow(unused_imports)]
//for clippy, as i dont use funcs yet
use pest_derive::Parser;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SiftParser;

//error handling!!
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse JSON: {0}")] // json parsing error
    JsonError(String),
    #[error("Unsupported structure: {0}")] // structure error
    StructureError(String),
}

pub fn print_structure(_value: &Value, _indent: usize) {}

// json
pub fn parse_json(_input: &str) {
    //TODO: implement JSON parsing logic
}

pub fn convert_to_csv(_value: &Value) {
    //TODO: implement conversion
}

//flatterning function
pub fn flatten_json(_value: &Value, _prefix: String, _out: &mut HashMap<String, String>) {
    //TODO: flattening
}
// metar str parsing function
pub fn parse_raw_ob(_raw: &str) {
    // TODO: impleent METAR parsing logic
}
