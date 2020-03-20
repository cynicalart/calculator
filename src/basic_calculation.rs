use std::collections::HashMap;

//Arrays which will be used to check if something is an operator, bracket or a function
const OPERATORS_ARR: [char; 5] = ['^', '*', '/', '+', '-'];
const FUNCTIONS_ARR: [&str; 16] = ["log", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan", "arccsc", "arcsec", "arccot", "to_deg", "to_rad"];
//Defining mathematical constants that may be used
const PI: f64 = std::f64::consts::PI;

//Defining the Evaluable trait, which will allow a datatype to be evaluated to a float
trait Evaluable {
    fn eval(&self) -> f64;
}
//Defining the Operation enum, which defines the Operations which will be used 
enum Operation<'a> {
    Exponent(&'a dyn Evaluable, &'a dyn Evaluable),
    Multiply(&'a dyn Evaluable, &'a dyn Evaluable),
    Divide(&'a dyn Evaluable, &'a dyn Evaluable),
    Add(&'a dyn Evaluable, &'a dyn Evaluable),
    Subtract(&'a dyn Evaluable, &'a dyn Evaluable)
}
//Implements the Evaluable trait for the Operation enum, and how the Operations
//work
impl Evaluable for Operation<'_> {
    fn eval(&self) -> f64 {
        match self {
            Operation::Exponent(base, power) => base.eval().powf(power.eval()),
            Operation::Multiply(left, right) => left.eval() * right.eval(),
            Operation::Divide(left, right) => left.eval() / right.eval(),
            Operation::Add(left, right) => left.eval() + right.eval(),
            Operation::Subtract(left, right) => left.eval() - right.eval()
        }
    }
}
//Implements the Evaluable trait for Strings so that a String can be cast to a float 
impl Evaluable for String {
    fn eval(&self) -> f64 {
        self.parse().unwrap()
    }
}
//Defining the reciprocal trigonometric functions and the other functions so that they
//are in scope
fn natural_log(value: f64) -> f64 {
    value.ln()
}

fn sine(value: f64) -> f64 {
    value.sin()
}

fn cosine(value: f64) -> f64 {
    value.cos()
}

fn tangent(value: f64) -> f64 {
    value.tan()
}

fn cosec(value: f64) -> f64 {
    value.sin().recip()
}

fn sec(value: f64) -> f64 {
    value.cos().recip()
}

fn cot(value: f64) -> f64 {
    value.tan().recip()
}

fn arcsin(value: f64) -> f64 {
    value.asin()
}

fn arccos(value: f64) -> f64 {
    value.acos()
}

fn arctan(value: f64) -> f64 {
    value.atan()
}

fn arccosec(value: f64) -> f64 {
    value.recip().asin()
}

fn arcsec(value: f64) -> f64 {
    value.recip().acos()
}

fn arccot(value: f64) -> f64 {
    value.recip().atan()
}

fn to_degrees(value: f64) -> f64 {
    let multiplier = 180.0 / PI;
    value * multiplier
}

fn to_radians(value: f64) -> f64 {
    let multiplier = PI / 180.0;
    value * multiplier
}

//Defining the function that will evaluate the Reverse Polish Notation Vector produced
pub fn evaluate_rpn(rpn_vec: Vec<String>) -> f64 {
    //Defining the variable that will be equal to the value of the individual expressions
    let mut value: f64 = 0.0;
    //Defining the variable that will be equal to the value of the whole expression
    let answer: f64;
    //The variable that is the value of the vector at the start of each iteration of the
    //while loop
    let mut current_vector: Vec<String> = rpn_vec;
    //The varaible that is the value of the vector at the end of each iteration of the
    //while loop  
    let mut update_vector: Vec<String> = Vec::new();
    //The index of the number that is at the start of the Reverse Polish Notation for the 
    //individual expression
    let mut position = 0;
    //HashMap relating the String for each function to their actual function
    let mut functions: HashMap<String, &dyn Fn(f64) -> f64> = HashMap::new();
    functions.insert("ln".to_string(), &natural_log);
    functions.insert("sin".to_string(), &sine);
    functions.insert("cos".to_string(), &cosine);
    functions.insert("tan".to_string(), &tangent);
    functions.insert("csc".to_string(), &cosec);
    functions.insert("sec".to_string(), &sec);
    functions.insert("cot".to_string(), &cot);
    functions.insert("arcsin".to_string(), &arcsin);
    functions.insert("arccos".to_string(), &arccos);
    functions.insert("arctan".to_string(), &arctan);
    functions.insert("arccsc".to_string(), &arccosec);
    functions.insert("arcsec".to_string(), &arcsec);
    functions.insert("arccot".to_string(), &arccot);
    functions.insert("to_deg".to_string(), &to_degrees);
    functions.insert("to_rad".to_string(), &to_radians);
    //While the current vector contains more than one item
    while current_vector.len() > 1 {
        //Iterating over the current vector for an operator
        for (i, item) in current_vector.iter().enumerate() {
            //If the item is a number it shouldn't go through any of the following code
            if item.parse::<f64>().is_ok() {
                continue;
            }
            //Defining the variable that is equal to the value of the current character
            let chr = item.chars().nth(0).unwrap();
            //Checking if tha character is an operator
            if OPERATORS_ARR.contains(&chr) {
                if i > 1 {
                //Checking which operator the character is and using the appropiate Operation
                //in the Operation enum to evaluate the individual expression, making the
                //value variable equal to it
                    if chr == '^' {
                        value = Operation::Exponent(&current_vector[i - 2], &current_vector[i - 1]).eval();
                    }

                    if chr == '*' {
                        value = Operation::Multiply(&current_vector[i - 2], &current_vector[i - 1]).eval();
                    }

                    if chr == '/' {
                        value = Operation::Divide(&current_vector[i - 2], &current_vector[i - 1]).eval();
                    }

                    if chr == '+' {
                        value = Operation::Add(&current_vector[i - 2], &current_vector[i - 1]).eval();
                    }

                    if chr == '-' {
                        value = Operation::Subtract(&current_vector[i - 2], &current_vector[i - 1]).eval();
                    }

                    println!("Value: {}", value);
                    //Setting the value of position to the index of the character at the start
                    //of the individual expression, and breaking the loop
                    position = i - 2;
                    println!("Position: {}", position);
                    break;
                } else {
                    panic!("Syntax error: cannot evaluate the erroneous expression.");
                }
            }

            if item.len() > 1 {
                //Checking if the item is a function
                if FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
                    //The general log is the only function that has more than one argument 
                    //and so it can't fit inside the functions HashMap
                    if item == "log" {
                        if i > 1 {
                            let base: f64;

                            if i == 1 {
                                base = 10.0;
                            } else {
                                base = current_vector[i - 2].eval();
                            } 

                            let arg = current_vector[i - 1].eval();
                            value = arg.log(base);
                            position = i - 2;
                        } else {
                            panic!("Syntax error: cannot evaluate the erroneous expression.");
                        }
                    //If it's not a general log, it must be in the HashMap and so we use the 
                    //HashMap
                    } else {
                        if i > 0 {
                            let func_name = item;
                            let func_arg = &current_vector[i - 1].eval();
                            let target_func = functions.get(func_name).unwrap();
                            value = target_func(*func_arg);
                            position = i - 1;
                        } else {
                            panic!("Syntax error: cannot evaluate the erroneous expression.");
                        }
                    }

                    println!("Value: {}", value);
                    println!("Position: {}", position);
                    break;
                }
            }
        }

        let mut i = 0;
        //Iterating over the current vector, adding the values that were not in the individual
        //expression to the updated vector, and adding the value of the individual expression
        //instead of the ones that were
        while i < current_vector.len() {
            if i == position {
                update_vector.push(value.to_string());
                if FUNCTIONS_ARR.contains(&&current_vector[i + 1].to_string()[..]) {
                    i += 2;
                } else {
                    i += 3;
                } 
            } else {
                update_vector.push(current_vector[i].to_string());
                i += 1;
            }
        }
        //Emptying tha value of the current vector
        current_vector = Vec::new();
        //Making the current vector equal to the updated vector
        for item in &update_vector {
            current_vector.push(item.to_string());
        }
        //Emptying the updated vector variable
        update_vector = Vec::new();

        println!("Current Vector: {:?}", current_vector);

    }
    //Making the answer variable equal to the value of the entire expression
    answer = current_vector[0].eval();
    //Returning the answer variable
    answer
}
