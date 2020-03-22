use crate::format::rpn;
use crate::basic_calculation::evaluate_rpn;

//Arrays which will be used to check if something is an operator, bracket or a function
const OPERATORS_ARR: [char; 5] = ['^', '*', '/', '+', '-'];
const FUNCTIONS_ARR: [&str; 16] = ["log", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan", "arccsc", "arcsec", "arccot", "to_deg", "to_rad"];
//Defining mathematical constants that may be used
const EULER: f64 = std::f64::consts::E;
const PI: f64 = std::f64::consts::PI;

pub fn calculate_derivative(expression_vec: Vec<String>) -> Vec<String> {
    let split_variables_vec = split_variables(expression_vec.to_vec());
    let operator_order_vec = operator_order(expression_vec.to_vec());
    let mut output_vec: Vec<String> = Vec::new();
    let mut product_rule_vec: Vec<Vec<String>> = Vec::new();

    for (i, vector) in split_variables_vec.iter().enumerate() {
        let differentiated = differentiate_variable(vector.to_vec());

        if i < operator_order_vec.len() {
            if i > 0 {
                if operator_order_vec[i - 1] == '*' {
                    product_rule_vec.push(vector.to_vec());
                    product_rule_vec.push(differentiated.to_vec());
                    let new_derivative = product_rule(product_rule_vec.to_vec());

                    for item in &new_derivative {
                        output_vec.push(item.to_string());
                    }

                    output_vec.push(operator_order_vec[i].to_string());
                }
            }

            if operator_order_vec[i] == '*' {
                product_rule_vec.push(vector.to_vec());
                product_rule_vec.push(differentiated.to_vec());
            }

            if operator_order_vec[i] == '+' || operator_order_vec[i] == '-' {
                for item in &differentiated {
                    output_vec.push(item.to_string());
                }

                output_vec.push(operator_order_vec[i].to_string());
            }

        } else {
            if i > 0 {
                if operator_order_vec[i - 1] == '*' {
                    product_rule_vec.push(vector.to_vec());
                    product_rule_vec.push(differentiated.to_vec());
                    let new_derivative = product_rule(product_rule_vec.to_vec());

                    for item in &new_derivative {
                        output_vec.push(item.to_string());
                    }

                    continue;
                }
            }

            for item in &differentiated {
                output_vec.push(item.to_string());
            }
        }
    }

    println!("{:?}", output_vec);
    output_vec
} 

fn operator_order(expression_vec: Vec<String>) -> Vec<char> {
    let mut output_vec: Vec<char> = Vec::new();
    let mut variable_encountered = false;

    for (i, item) in expression_vec.iter().enumerate() {
        let chr = item.chars().nth(0).unwrap();

        let mut previous_item: String = String::new();
        let mut next_item: String = String::new();
        //If the index is not the first or last index
        if i > 0 {
            //Redefine the variable for the previous character
            previous_item = expression_vec[i - 1].to_string();
        }

        if i + 1 < expression_vec.len() {
            //Redefine the variable for the next character
            next_item = expression_vec[i + 1].to_string();
        }

        if item.len() == 1 && chr.is_alphabetic() {
            variable_encountered = true;
        }

        if chr == '+' || chr == '-' {
            output_vec.push(chr);
        } 

        if chr == '*' || chr == '/' {
            if previous_item.parse::<f64>().is_ok() || previous_item == String::from(")") {
                if next_item.parse::<f64>().is_ok() || next_item == String::from("(") {
                    if variable_encountered {
                        output_vec.push(chr);
                        variable_encountered = false;
                    }
                } else {
                    if variable_encountered {
                        output_vec.push(chr);
                        variable_encountered = false;
                    }
                }
            }
        }
    }

    output_vec
}

fn split_variables(expression_vec: Vec<String>) -> Vec<Vec<String>> {
    let mut individual_variable_vec: Vec<String> = Vec::new();
    let mut output_vec: Vec<Vec<String>> = Vec::new();
    let mut variable_encountered = false;

    for (i, item) in expression_vec.iter().enumerate() {
        let chr = item.chars().nth(0).unwrap(); 

        let mut previous_item: String = String::new();
        let mut next_item: String = String::new();
        //If the index is not the first or last index
        if i > 0 {
            //Redefine the variable for the previous character
            previous_item = expression_vec[i - 1].to_string();
        }

        if i + 1 < expression_vec.len() {
            //Redefine the variable for the next character
            next_item = expression_vec[i + 1].to_string();
        }

        if item.len() == 1 && chr.is_alphabetic() {
            variable_encountered = true;
        }

        if chr == '+' || chr == '-' {
            output_vec.push(individual_variable_vec.to_vec());
            individual_variable_vec = Vec::new();
            continue;
        }
        
        if chr == '*' || chr == '/' {
            if previous_item.parse::<f64>().is_ok() || previous_item == String::from(")") {
                if next_item.parse::<f64>().is_ok() || next_item == String::from("(") {
                    if variable_encountered {
                        output_vec.push(individual_variable_vec.to_vec());
                        individual_variable_vec = Vec::new();
                        variable_encountered = false;
                        continue;
                    }
                } else {
                    if variable_encountered {
                        output_vec.push(individual_variable_vec.to_vec());
                        individual_variable_vec = Vec::new();
                        variable_encountered = false;
                        continue;
                    }
                }
            }

        } 

        individual_variable_vec.push(item.to_string());

        if i + 1 == expression_vec.len() {
            output_vec.push(individual_variable_vec.to_vec());
        }
    }

    output_vec 
}

fn find_variable(expression_vec: Vec<String>) -> char {
    let mut output: char = '_';

    for item in &expression_vec {
        let chr = item.chars().nth(0).unwrap();

        if item.len() == 1 && chr.is_alphabetic() {
            output = chr;
        }
    }

    output
}

fn simplify_multiplier(expression_vec: Vec<String>) -> f64 {
    let mut multiplier_vec: Vec<String> = Vec::new();
    let mut multiplier_encountered = false;
    let mut exponent_encountered = false;    

    for (i, item) in expression_vec.iter().enumerate() {
        let chr = item.chars().nth(0).unwrap();

        let mut previous_item: String = String::new();
        let mut next_item: String = String::new();

        if i > 0 {
            //Redefine the variable for the previous item
            previous_item = expression_vec[i - 1].to_string();
        }

        if i + 1 < expression_vec.len() {
            //Redefine the variable for the next item
            next_item = expression_vec[i + 1].to_string();
        }

        if chr == '^' && !previous_item.parse::<f64>().is_ok() {
            exponent_encountered = true;
        }

        if chr == '*' && !next_item.parse::<f64>().is_ok() {
            multiplier_encountered = true;
        } else if !multiplier_encountered && !exponent_encountered && !chr.is_alphabetic() {
            multiplier_vec.push(item.to_string());
        }
    }

    let output: f64;

    if multiplier_vec.is_empty() {
        output = 1.0
    } else if multiplier_vec.len() == 1 {
        output = multiplier_vec[0].to_string().parse().unwrap();
    } else {
        output = evaluate_rpn(rpn(multiplier_vec.to_vec()));
    }

    println!("Mult {:?}", output);

    output
}

fn simplify_exponent(expression_vec: Vec<String>) -> f64 {
    let mut exponent_vec: Vec<String> = Vec::new();
    let mut exponent_encountered = false;
    let mut variable: char = '_';

    for (i, item) in expression_vec.iter().enumerate() {
        let chr = item.chars().nth(0).unwrap();

        let mut previous_item: String = String::new();

        if i > 0 {
            //Redefine the variable for the previous item
            previous_item = expression_vec[i - 1].to_string();
        }

        if item.len() == 1 && chr.is_alphabetic() {
            variable = chr;
        }

        if exponent_encountered {
            exponent_vec.push(item.to_string());
        }

        if chr == '^' && !previous_item.parse::<f64>().is_ok() {
            exponent_encountered = true;
        }
    }

    let output: f64;

    if exponent_vec.is_empty() {
        if variable.is_alphabetic() {
            output = 1.0;
        } else {
            output = 0.0;
        }
    } else if exponent_vec.len() == 1 {
        output = exponent_vec[0].to_string().parse().unwrap();
    } else {
        output = evaluate_rpn(rpn(exponent_vec.to_vec()));
    }

    println!("Exp {:?}", output);

    output
}

fn differentiate_variable(expression_vec: Vec<String>) -> Vec<String> {
    let variable = find_variable(expression_vec.to_vec());
    let original_multiplier = simplify_multiplier(expression_vec.to_vec()); 
    let original_exponent = simplify_exponent(expression_vec.to_vec());

    let new_multiplier = original_multiplier * original_exponent;
    let new_exponent = original_exponent - 1.0;
    println!("Mult {:?}", new_multiplier);
    println!("Exp {:?}", new_exponent);
    println!("Var {:?}", variable);

    let mut differentiated: Vec<String> = Vec::new();

    if variable.is_alphabetic() {
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

    differentiated
}

fn product_rule(product_rule_vec: Vec<Vec<String>>) -> Vec<String> {
    let mut multiplier: f64 = 0.0;
    let mut exponent: f64 = 0.0;
    let mut multipliers_vec: Vec<f64> = Vec::new();
    let mut exponents_vec: Vec<f64> = Vec::new();
    let mut new_multiplier: f64 = 0.0;
    let mut new_exponent: f64 = 0.0;

    for (i, vector) in product_rule_vec.iter().enumerate() {
        multiplier = simplify_multiplier(vector.to_vec());
        exponent = simplify_exponent(vector.to_vec());
        multipliers_vec.push(multiplier);
        exponents_vec.push(exponent);
        multiplier = 0.0;
        exponent = 0.0;
    }

    println!("Mult {:?}", multipliers_vec);
    println!("Exp {:?}", exponents_vec);

    let mut full_product_derivative: Vec<String> = Vec::new();
    let mut multiplied_derivative: Vec<String> = Vec::new(); 
    let mut variable: char = '_';

    if find_variable(product_rule_vec[0].to_vec()).is_alphabetic() {
        variable = find_variable(product_rule_vec[0].to_vec());
    } else if find_variable(product_rule_vec[3].to_vec()).is_alphabetic() {
        variable = find_variable(product_rule_vec[3].to_vec());
    }

    new_multiplier = multipliers_vec[0] * multipliers_vec[3];
    new_exponent = exponents_vec[0] + exponents_vec[3];

    println!("New mult {}", new_multiplier);
    println!("New exp {}", new_exponent);

    if variable.is_alphabetic() {
        if new_multiplier > 1.0 || new_multiplier < 1.0 {
            multiplied_derivative.push(new_multiplier.to_string());
        } else if new_exponent == 0.0 && new_multiplier == 1.0 {
            multiplied_derivative.push(new_multiplier.to_string());
        }

        if new_exponent == 1.0 {
            multiplied_derivative.push(String::from("*"));
            multiplied_derivative.push(variable.to_string());
        } else if new_exponent > 0.0 || new_exponent < 0.0 {
            multiplied_derivative.push(String::from("*"));
            multiplied_derivative.push(variable.to_string());
            multiplied_derivative.push(String::from("^"));
            multiplied_derivative.push(new_exponent.to_string());
        } 
    }

    println!("{:?}", multiplied_derivative);

    for item in &multiplied_derivative {
        full_product_derivative.push(item.to_string());
    }

    full_product_derivative.push(String::from("+"));
    multiplied_derivative = Vec::new();

    if find_variable(product_rule_vec[1].to_vec()).is_alphabetic() {
        variable = find_variable(product_rule_vec[1].to_vec());
    } else if find_variable(product_rule_vec[2].to_vec()).is_alphabetic() {
        variable = find_variable(product_rule_vec[2].to_vec());
    }

    new_multiplier = multipliers_vec[1] * multipliers_vec[2];
    new_exponent = exponents_vec[1] + exponents_vec[2];

    if variable.is_alphabetic() {
        if new_multiplier > 1.0 || new_multiplier < 1.0 {
            multiplied_derivative.push(new_multiplier.to_string());
        } else if new_exponent == 0.0 && new_multiplier == 1.0 {
            multiplied_derivative.push(new_multiplier.to_string());
        }

        if new_exponent == 1.0 {
            multiplied_derivative.push(String::from("*"));
            multiplied_derivative.push(variable.to_string());
        } else if new_exponent > 0.0 || new_exponent < 0.0 {
            multiplied_derivative.push(String::from("*"));
            multiplied_derivative.push(variable.to_string());
            multiplied_derivative.push(String::from("^"));
            multiplied_derivative.push(new_exponent.to_string());
        } 
    }

    println!("{:?}", multiplied_derivative);

    for item in &multiplied_derivative {
        full_product_derivative.push(item.to_string());
    }

    multiplied_derivative = Vec::new();

    println!("{:?}", full_product_derivative);

    full_product_derivative
}