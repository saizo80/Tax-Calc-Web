/*
Tax Calculator
For a given input it will calculate the amount that will add to 
the target with the tax rate. 
Default tax rate is 7% but can be easily changed in the main function.
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
            println!("Target could not be calculated");
            std::process::exit(1);
        }
    }
    return input;
}

#[wasm_bindgen]
pub fn rustInterface(input: &str) -> String {
    let rate: f64 = 0.07;
    if input.parse::<f64>().is_ok() { 
        let number = input.parse().unwrap();
            //println!("${:.2}", find_tax_target(number, rate));
            return format!("${:.2}",find_tax_target(number, rate));
    }
    else {
        return String::from("None");
    }
}
