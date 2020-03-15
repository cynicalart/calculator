use std::io;
use std::collections::HashMap;

//Arrays which will be used to check if something is an operator, bracket or a function
const BRACKETS_ARR: [char; 4] = ['(', ')', '{', '}'];
const OPERATORS_ARR: [char; 5] = ['^', '*', '/', '+', '-'];
const FUNCTIONS_ARR: [&str; 11] = ["log", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan"];
//Array containing the precedences of the operators in the same order as the operators
//in the operator array
const PRECEDENCE: [i32; 5] = [4, 3, 3, 2, 2];
//Defining mathematical constants that may be used
const EULER: f64 = std::f64::consts::E;
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

fn main() {
    //Defines the variable where the entered expression will be stored
    let mut expression = String::new();
    //Taking the user's input and moving it to the expression variable
    io::stdin().read_line(&mut expression)
        .expect("Failed to read line");
    //Removing the trailing \n from the end of the input
    expression.truncate(expression.len() - 1);
    //Converting the expression String to a Vector of Strings
    let expression_vec = expression_to_vec(expression); 

    println!("{:?}", expression_vec);
    //Converting the expression Vector into another Vector of Strings in which the
    //Strings have been arranged according to Reverse Polish Notation (rpn)
    let rpn_vec = rpn(expression_vec);

    println!("{:?}", rpn_vec);
    //Defining the answer as the evaluated rpn_vec
    let answer = evaluate_rpn(rpn_vec);
    println!("The answer is: {}", answer);
}
//Defining the expression_to_vec function, which converts the original expression
//String into a Vector of strings
fn expression_to_vec(expression: String) -> Vec<String> {
    //Initialising the variable which will be the original String with any whitespace
    //removed
    let mut cleaned_expression = String::new();
    //Iterating through the original String and only pushing non-whitespace characters
    //to the new String
    for chr in expression.chars() {
        if !chr.is_whitespace() {
            cleaned_expression.push(chr);
        }
    }
    //Initialising the variables which will be used to obtain the output
    let mut output_vec: Vec<String> = Vec::new();
    //A stack in effect, to push numerical values so that for example the '5' and '4' 
    //in "54" will be combined into the same String
    let mut num = String::new();
    let mut sign = String::new();
    //Also an effective stack, to allow for a full function term to be combined
    let mut function = String::new();
    //Iterating over the new expression String
    for (i, chr) in cleaned_expression.chars().enumerate() {
        //Initialising the variables for the previous character and the next character
        //as effective null values in case the current index is the first or the last
        //index
        let mut previous_chr: char = 'n';
        let mut next_chr: char = 'n';
        //If the index is not the first or last index
        if i > 0 && i + 1 < cleaned_expression.len() {
            //Redefine the variables for the previous and next characters as the previous
            //and next character respectively
            previous_chr = cleaned_expression.chars().nth(i - 1).unwrap();
            next_chr = cleaned_expression.chars().nth(i + 1).unwrap();
        }
        //In the case of the general log function, a comma will be used to separate the 
        //base and the value and so this accounts for that
        if chr == ',' {
            output_vec.push(num);
            num = String::new();
        }
        //If the current character is numerical, or if it is a period, add it to the num
        //String. The inclusion of the period in the condition accounts for decimals 
        if chr.is_numeric() || chr == '.' {
            num.push(chr);
        }
        //Checking if the character is a letter
        if chr.is_alphabetic() {
            //If i is 0 we can't check the previous character and so we need different 
            //conditions for that case
            if i == 0 {
                //If the first character is e, then it must be Euler's constant
                if chr == 'e' {
                    for chr in EULER.to_string().chars() {
                        num.push(chr);
                    }
                //Otherwise it must be part of a function
                } else {
                    function.push(chr);
                }
            //Now we can check the previous character
            } else {
                //If the current character is 'i' and the previous character is 'p', then
                //it must be pi
                if chr == 'i' && previous_chr == 'p' {
                    for chr in PI.to_string().chars() {
                        num.push(chr);
                    } 

                //If the current character is 'e' and the previous character is not a letter
                //then it must be Euler's constant
                } else if chr == 'e' && !previous_chr.is_alphabetic() {
                    for chr in EULER.to_string().chars() {
                        num.push(chr);
                    }
                //Otherwise it must be part of a function
                } else {
                    function.push(chr);
                }
            }
        }
        //If the current character is an operator or a bracket
        if OPERATORS_ARR.contains(&chr) || BRACKETS_ARR.contains(&chr) {
            //Checking if the character is a minus sign
            if chr == '-' {
                //If the minus sign is part of a number, push it to the num string
                if i == 0 || OPERATORS_ARR.contains(&previous_chr) || BRACKETS_ARR.contains(&previous_chr) {
                    num.push(chr);
                    //Continues, so that any further changes do not occur at this index
                    continue;
                } 
            } 
            //Checking if there are 2 multiplication signs in a row, and replacing them
            //with the alternative '^' exponent notation
            if chr == '*' && &next_chr == &chr {
                //Pushing the '^' exponent sign to the sign String
                sign.push('^');
            //Checking if the current character is the second multiplication sign in the 
            //alias for the exponent sign
            } else if i > 0 && &previous_chr == &chr && chr == '*' {
                //Continuing as nothing should happen for this character
                continue;
            //Otherwise, the character is pushed to the sign String if it is not a curly bracket
            } else { 
                sign.push(chr);
            }
            //If the num String is not empty, push the num String to the output Vector
            if !num.is_empty() {
                output_vec.push(num);
            }
            //If the sign is an opening bracket
            if chr == '(' {
                //and if the previous character was a closing bracket
                if previous_chr == ')' {
                    //Push a multiplication sign to the output vector, as this is an 
                    //implied multiplication
                    output_vec.push(String::from("*"));
                }
            }
            //If the sign is an opening curly bracket, it means we are at the end of the
            //function string
            if chr == '{' {
                output_vec.push(function);
                function = String::new();
            }
            //Pushing the current contents of the sign String to the output String
            if !sign.is_empty() {
                output_vec.push(sign);
            }
            //Emptying both the sign and num Strings ready for the next character
            num = String::new();
            sign = String::new();
        }
        //if the current index plus 1 is more than or equal to the length of the expression String
        if i + 1 >= cleaned_expression.len() {
            //If the num STring is not empty
            if !num.is_empty() {
                //Add it to the output Vector
                output_vec.push(num);
            }
            //Emptying the num String for the next character
            num = String::new();
        }

    }
    //Returning the output Vector
    output_vec 
}
//Defining the rpn function, which will rearrange the items in the expression Vector into
//Reverse Polish Notation so that it can be evaluated easier later
fn rpn(expression_vec: Vec<String>) -> Vec<String> {
    //Initiating the output Vector variable
    let mut output_vec: Vec<String> = Vec::new();
    //Itiating the stack variable, where any operators and brackets will go initially
    let mut stack: Vec<String> = Vec::new();
    //Iterating over the expression Vector
    for (i, item) in expression_vec.iter().enumerate() {

        println!("Stack: {:?}", stack);
        //If the string is a number, add it to the output Vector
        if item.parse::<f64>().is_ok() {
            if i > 0 {
                if expression_vec[i - 1].parse::<f64>().is_ok() {
                    output_vec.push(stack[0].to_string());
                    stack.remove(0);
                }
            }

            output_vec.push(item.to_string());
            //Continuing as we no longer need to do anything with this item
            continue;
        } 
        //If the item is a function it cannot be converted to a character
        if item.len() > 1 && FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
            stack.insert(0, item.to_string());
            continue;
        }
        //Defining the variable that is equal to the value of the current character
        let chr = item.chars().nth(0).unwrap();

        //Since we now know the character is definitely a sign, this wil just add the
        //character to the stack if the stack is empty 
        if stack.is_empty() {
            stack.push(chr.to_string());
        //Otherwise, if the stack is not empty
        } else {
            //Defining the variable equal to the item at the top of the stack
            let leading_stack_item = &stack[0];
            //If the item at the top of the stack is a function, add the item to the stack
            if leading_stack_item.len() > 1 {
                stack.insert(0, chr.to_string());
                continue;
            }
            //Otherwise, turn it into a character
            let leading_stack = leading_stack_item.chars().nth(0).unwrap();
            //If both the current character and the character at the top of the stack are
            //operators, their precedences must be compared
            if OPERATORS_ARR.contains(&leading_stack) && OPERATORS_ARR.contains(&chr) {
                //Defining the variables equal to the PRECEDENCE of the character at the
                //top of the stack and that of the current character
                let leading_stack_precedence = PRECEDENCE[OPERATORS_ARR.iter().position(|&operator| operator == leading_stack).unwrap()];
                let chr_precedence = PRECEDENCE[OPERATORS_ARR.iter().position(|&operator| operator == chr).unwrap()];
                //If the PRECEDENCE of the current character is the greatest
                if chr_precedence > leading_stack_precedence {
                    //Insert the current character at the top of the stack
                    stack.insert(0, chr.to_string());
                //If the characters' precedences are equal
                } else if chr_precedence == leading_stack_precedence {
                    //If the 2 characters are equal to one another
                    if chr == leading_stack {
                        //Insert the current character at the top of the stack
                        stack.insert(0, chr.to_string());
                    //If they are not the same character
                    } else {
                        //Push the character that is currently at the top of the stack to
                        //the output Vector, and replace it in the stack with the current
                        //character
                        output_vec.push(leading_stack.to_string());

                        let mut position = 0;

                        for j in 0..stack.len() {
                            if stack[i] == leading_stack.to_string() {
                                position = j;
                                break;
                            }
                        }

                        let mut new_stack: Vec<String> = Vec::new();

                        for (j, item) in stack.iter().enumerate() {
                            if !j == position {
                                new_stack.push(item.to_string())
                            }
                        }

                        stack = new_stack;
                    }
                //Otherwise if the character at the top of the stack's PRECEDENCE is greater
                } else if chr_precedence < leading_stack_precedence {
                    //Initiating the variable which will replace the current stack
                    let mut new_stack: Vec<String> = Vec::new();
                    let mut bracket_encountered = false;
                    //Iterating over the current stack
                    for item in &stack {
                        //If the item is a function, add it to the new stack and continue
                        if item.len() > 1 && FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
                            new_stack.push(item.to_string());
                            continue;
                        }

                        let sign = item.chars().nth(0).unwrap();
                        //If the character is a bracket, add it to the new stack and continue
                        if BRACKETS_ARR.contains(&sign) {
                            new_stack.push(sign.to_string());
                            bracket_encountered = true;
                            continue;
                        }
                        //Defining the variable equal to the index of the current
                        //character in the operator array to determine it's PRECEDENCE
                        let j = OPERATORS_ARR.iter().position(|&operator| operator == sign).unwrap();
                        let sign_precedence = PRECEDENCE[j];
                        //If a bracket has been encountered, add the character to the new
                        //new stack regardless of its PRECEDENCE
                        if bracket_encountered == true {
                            new_stack.push(sign.to_string());
                        //Otherwise, if the current sign in the current stack has a greater
                        //PRECEDENCE than that of the current character, add it to the output
                        //Vector 
                        } else if sign_precedence >= chr_precedence {
                            output_vec.push(sign.to_string());
                        //Finally if neither of the previous conditions are true, add the current
                        //sign in the old stack to the new stack
                        } else {
                            new_stack.push(sign.to_string());
                        }
                    }
                    //Replace the value of the old stack with that of the new stack
                    stack = new_stack;
                    //Since the current character has a higher PRECEDENCE that that of the
                    //character at the top of the new stack, insert it at the top of the stack
                    stack.insert(0, chr.to_string());
                }
            //If the current character is an operator and the character at the top of the 
            //stack is a bracket
            } else if OPERATORS_ARR.contains(&chr) && BRACKETS_ARR.contains(&leading_stack) {
                //Insert the current character at the top of the stack
                stack.insert(0, chr.to_string());
            //If the current character is an opening bracket, insert it at the top of the stack
            } else if chr == '(' || chr == '{' {
                stack.insert(0, chr.to_string());
            //If the current character is a closing bracket, add any operators before the
            //corresponding opening bracket to the output Vector
            } else if chr == ')' || chr == '}' {
                //Iterating over the stack
                for item in &stack {
                    //If the item is a function, add it to the output and continue
                    if item.len() > 1 && FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
                        output_vec.push(item.to_string());
                        continue;
                    }

                    let sign = item.chars().nth(0).unwrap();
                    //If the current character in the stack is an opening bracket, break
                    //the loop
                    if sign == '(' || sign == '{' {
                        break;
                    //Otherwise, add the character to the output Vector
                    } else {
                        output_vec.push(sign.to_string());
                    }
                }
                //Removing any of the characters that were added to the output Vector
                //and the opening bracket from the stack

                for i in 0..stack.len() {
                    let current_character = stack[i].chars().nth(0).unwrap();
                    if stack[i].len() == 1 && BRACKETS_ARR.contains(&current_character) {
                        stack.remove(i);
                        break;
                    } else {
                        stack.remove(i);
                    }
                }
                println!("Stack: {:?}", stack);
            }
        }
    }
    //If the stack is not empty
    if !stack.is_empty() {
        //Add every item in the stack to the output Vector
        for item in &stack {
            output_vec.push(item.to_string());
        }
    }
    //Returning the output Vector
    output_vec
}
//Defining the function that will evaluate the Reverse Polish Notation Vector produced
fn evaluate_rpn(rpn_vec: Vec<String>) -> f64 {
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
    //While the current vector contains more than one item
    while current_vector.len() > 1 {
        //Iterating over the current vector for an operator
        for (i, item) in current_vector.iter().enumerate() {
            //Defining the variable that is equal to the value of the current character
            let chr = item.chars().nth(0).unwrap();
            //Checking if tha character is an operator
            if OPERATORS_ARR.contains(&chr) {
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
            }

            if item.len() > 1 {
                //Checking if the item is a function
                if FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
                    //The general log is the only function that has more than one argument 
                    //and so it can't fit inside the functions HashMap
                    if item == "log" {
                        let base: f64;

                        if i == 1 {
                            base = 10.0;
                        } else {
                            base = current_vector[i - 2].eval();
                        } 

                        let arg = current_vector[i - 1].eval();
                        value = arg.log(base);
                    //If it's not a general log, it must be in the HashMap and so we use the 
                    //HashMap
                    } else {
                        let func_name = item;
                        let func_arg = &current_vector[i - 1].eval();
                        let target_func = functions.get(func_name).unwrap();
                        value = target_func(*func_arg);
                    }

                    println!("Value: {}", value);
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
                i += 3 
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