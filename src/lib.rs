/*
Tax Calculator
For a given input it will calculate the amount that will add to 
the target with the tax rate. 
Default tax rate is 7% but can be easily changed in the main function.
This code is adapted from the original to be able to work in a web format
and integrate with JavaScript.

Written by Matthew Thornton 
April 19, 2022
*/

// Include the `wasm_bindgen` attribute into the current namespace
use wasm_bindgen::prelude::wasm_bindgen;

fn find_tax_target(target: f64, rate: f64) -> f64 {
    let mut input = target;
    while (((input+(input*rate))*100.0).round())/100.0 != target {
        input -= 0.01;
        if input < 0.00 {
            return -0.03;
        }
    }
    return input;
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn rustInterface(input: &str, rate: &str) -> String {
    if input.parse::<f64>().is_ok() && rate.parse::<f64>().is_ok() { 
        let value: f64 = find_tax_target(input.parse().unwrap()
            , rate.parse::<f64>().unwrap()/100.0);
        if value != -0.03 {
            return format!("{:.2}", value);
        }
        else{
            return String::from("Error, could not find target amount.");
        }
    }
    else {
        return String::from("Error, please enter valid number.");
    }
}
