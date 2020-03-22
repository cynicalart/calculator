use crate::format::rpn;
use crate::basic_calculation::evaluate_rpn

//Arrays which will be used to check if something is an operator, bracket or a function
const OPERATORS_ARR: [char; 5] = ['^', '*', '/', '+', '-'];
const FUNCTIONS_ARR: [&str; 16] = ["log", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan", "arccsc", "arcsec", "arccot", "to_deg", "to_rad"];
//Defining mathematical constants that may be used
const EULER: f64 = std::f64::consts::E;
const PI: f64 = std::f64::consts::PI;

fn operator_order(expression_vec: Vec<String>) -> Vec<char> {
    let mut output_vec: Vec<char> = Vec::new();

    for item in &expression_vec {
        let chr = item.chars().nth(0).unwrap();

        if chr == '+' || chr == '-' {
            output_vec.push(chr);
        }
    }

    output_vec
}

fn split_variables(expression_vec: Vec<String>) -> Vec<Vec<String>> {
    let mut individual_variable_vec: Vec<String> = Vec::new();
    let mut output_vec: Vec<Vec<String>> = Vec::new();

    for (i, item) in expression_vec.iter().enumerate() {
        let chr = item.chars().nth(0).unwrap(); 

        if chr == '+' || chr == '-' {
            output_vec.push(individual_variable_vec.to_vec());
            individual_variable_vec = Vec::new();
        } else {
            individual_variable_vec.push(item.to_string());
        }

        if i + 1 == expression_vec.len() {
            output_vec.push(individual_variable_vec.to_vec());
        }
    }

    output_vec 
}

pub fn differentiate_variables(expression_vec: Vec<String>) -> Vec<String> {
    let split_variables_vec: Vec<Vec<String>> = split_variables(expression_vec.to_vec());
    let operator_order_vec: Vec<char> = operator_order(expression_vec.to_vec());
    let mut output_vec: Vec<String> = Vec::new();

    for (i, vector) in split_variables_vec.iter().enumerate() {
        let mut differentiated: Vec<String> = Vec::new();
        let mut multiplier_vec: Vec<String> = Vec::new();
        let mut exponent_vec: Vec<String> = Vec::new();
        let mut multiply_encountered = false;
        let mut exponent_encountered = false;
        let mut variable_encountered = false;
        let mut variable: char = '_';

        for item in vector.to_vec().iter() {
            let chr = item.chars().nth(0).unwrap();

            if chr == '*' {
                multiply_encountered = true;
            } 

            if exponent_encountered {
                exponent_vec.push(item.to_string());
            }

            if chr == '^' {
                exponent_encountered = true;
            }

            if chr.is_alphabetic() && item.len() == 1 {
                variable_encountered = true;
                variable = chr;
            }

            if !multiply_encountered && !exponent_encountered && !chr.is_alphabetic() {
                multiplier_vec.push(item.to_string());
            }
        }

        println!("{:?}, {:?}", exponent_vec, multiplier_vec);

        let multiplier: f64;
        let exponent: f64;

        if !multiplier_vec.is_empty() && multiplier_vec.len() > 1 {
            multiplier = evaluate_rpn(rpn(multiplier_vec));
        } else if multiplier_vec.len() == 1 {
            multiplier = multiplier_vec[0].to_string().parse().unwrap();
        } else {
            multiplier = 1.0;
        }

        if !exponent_vec.is_empty() && exponent_vec.len() > 1 {
            exponent = evaluate_rpn(rpn(exponent_vec));
        } else if exponent_vec.len() == 1 {
            exponent = exponent_vec[0].to_string().parse().unwrap();
        } else if variable_encountered {
            exponent = 1.0;
        } else {
            exponent = 0.0;
        }

        let new_multiplier: f64 = exponent * multiplier;
        let new_exponent: f64 = exponent - 1.0;

        println!("{}, {}", new_exponent, new_multiplier);

        if variable_encountered {
            if new_multiplier > 1.0 || new_multiplier < 1.0 {
                differentiated.push(new_multiplier.to_string());
            } else if new_exponent == 0.0 && new_multiplier == 1.0 {
                differentiated.push(new_multiplier.to_string());
            }

            if new_exponent == 1.0 {
                differentiated.push(String::from("*"));
                differentiated.push(variable.to_string());
            } else if new_exponent > 0.0 || new_exponent < 0.0 {
                differentiated.push(String::from("*"));
                differentiated.push(variable.to_string());
                differentiated.push(String::from("^"));
                differentiated.push(new_exponent.to_string());
            } 
        }

        println!("{:?}", differentiated);

        if i < operator_order_vec.len() {
            for item in &differentiated {
                output_vec.push(item.to_string());
            }

            output_vec.push(operator_order_vec[i].to_string());
        } else {
            for item in &differentiated {
                output_vec.push(item.to_string());
            }
        }
    }

    println!("{:?}", output_vec);
    output_vec
}