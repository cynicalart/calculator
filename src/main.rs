use std::io;
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
    //Arrays which will be used to check if something is an operator or a bracket
    let brackets_arr = ['(', ')'];
    let operators_arr = ['^', '*', '/', '+', '-'];
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
        //If the current character is numerical, or if it is a period, add it to the num
        //String. The inclusion of the period in the condition accounts for decimals 
        if chr.is_numeric() || chr == '.' {
            num.push(chr);
        }
        //If the current character is an operator or a bracket
        if operators_arr.contains(&chr) || brackets_arr.contains(&chr) {
            //Checking if the character is a minus sign
            if chr == '-' {
                //If the minus sign is part of a number, push it to the num string
                if i == 0 || operators_arr.contains(&previous_chr) {
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
            //Otherwise, the character is pushed to the sign String, as it is just a
            //a multiplication sign
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
            //Pushing the current contents of the sign String to the output String
            output_vec.push(sign);
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
    let mut stack = String::new();
    //Arrays which will be used to check if something is an operator or a bracket, and
    //if it is an operator, it's precedence
    let brackets_arr = ['(', ')'];
    let operators_arr = ['^', '*', '/', '+', '-'];
    let precedence = [4, 3, 3, 2, 2];
    //Iterating over the expression Vector
    for item in &expression_vec {

        println!("Stack: {}", stack);
        //If the string is longer than 1, or in other words it is a number, add it to
        //the output Vector
        if item.len() > 1 {
            output_vec.push(item.to_string());
            //Continuing as a string that is longer than 1 cannot be cast to a character
            continue;
        } 
        //Defining the variable that is equal to the value of the current character
        let chr = item.chars().nth(0).unwrap();
        //If the current character is a number, add it to the output Vector
        if chr.is_numeric() {
            output_vec.push(item.to_string());
            //Continuing as numbers don't belong on the stack
            continue;
        }
        //Since we now know the character is definitely a sign, this wil just add the
        //character to the stack if the stack is empty 
        if stack.is_empty() {
            stack.push(chr);
        //Otherwise, if the stack is not empty
        } else {
            //Defining the variable equal to the character at the top of the stack
            let leading_stack = stack.chars().nth(0).unwrap();
            //If both the current character and the character at the top of the stack are
            //operators, their precedences must be compared
            if operators_arr.contains(&leading_stack) && operators_arr.contains(&chr) {
                //Defining the variables equal to the precedence of the character at the
                //top of the stack and that of the current character
                let leading_stack_precedence = precedence[operators_arr.iter().position(|&operator| operator == leading_stack).unwrap()];
                let chr_precedence = precedence[operators_arr.iter().position(|&operator| operator == chr).unwrap()];
                //If the precedence of the current character is the greatest
                if chr_precedence > leading_stack_precedence {
                    //Insert the current character at the top of the stack
                    stack.insert(0, chr);
                //If the characters' precedences are equal
                } else if chr_precedence == leading_stack_precedence {
                    //If the 2 characters are equal to one another
                    if chr == leading_stack {
                        //Insert the current character at the top of the stack
                        stack.insert(0, chr);
                    //If they are not the same character
                    } else {
                        //Push the character that is currently at the top of the stack to
                        //the output Vector, and replace it in the stack with the current
                        //character
                        output_vec.push(leading_stack.to_string());
                        stack = stack.replace(&leading_stack.to_string(), &chr.to_string());
                    }
                //Otherwise if the character at the top of the stack's precedence is greater
                } else if chr_precedence < leading_stack_precedence {
                    //Initiating the variable which will replace the current stack
                    let mut new_stack = String::new();
                    let mut bracket_encountered = false;
                    //Iterating over the current stack
                    for sign in stack.chars() {
                        //If the character is a bracket, add it tot the new stack
                        if brackets_arr.contains(&sign) {
                            new_stack.push(sign);
                            bracket_encountered = true;
                            continue;
                        }
                        //Defining the variable equal to the index of the current
                        //character in the operator array to determine it's precedence
                        let j = operators_arr.iter().position(|&operator| operator == sign).unwrap();
                        let sign_precedence = precedence[j];
                        //If a bracket has been encountered, add the character to the new
                        //new stack regardless of its precedence
                        if bracket_encountered == true {
                            new_stack.push(sign);
                        //Otherwise, if the current sign in the current stack has a greater
                        //precedence than that of the current character, add it to the output
                        //Vector 
                        } else if sign_precedence >= chr_precedence {
                            output_vec.push(sign.to_string());
                        //Finally if neither of the previous conditions are true, add the current
                        //sign in the old stack to the new stack
                        } else {
                            new_stack.push(sign);
                        }
                    }
                    //Replace the value of the old stack with that of the new stack
                    stack = new_stack;
                    //Since the current character has a higher precedence that that of the
                    //character at the top of the new stack, insert it at the top of the stack
                    stack.insert(0, chr);
                }
            //If the current character is an operator and the character at the top of the 
            //stack is a bracket
            } else if operators_arr.contains(&chr) && brackets_arr.contains(&leading_stack) {
                //Insert the current character at the top of the stack
                stack.insert(0, chr);
            //If the current character is an opening bracket, insert it at the top of the stack
            } else if chr == '(' {
                stack.insert(0, chr);
            //If the current character is a closing bracket, add any operators before the
            //corresponding opening bracket to the output Vector
            } else if chr == ')' {
                //Iterating over the stack
                for sign in stack.chars() {
                    //If the current character in the stack is an opening bracket, break
                    //the loop
                    if sign == '(' {
                        break;
                    //Otherwise, add the character to the output Vector
                    } else {
                        output_vec.push(sign.to_string());
                    }
                }
                //Removing any of the characters that were added to the output Vector
                //and the opening bracket from the stack
                stack = (&stack[stack.chars().position(|bracket| bracket == '(').unwrap() + 1..stack.len()]).to_string();
                println!("Stack: {}", stack);
            }
        }
    }
    //If the stack is not empty
    if !stack.is_empty() {
        //Add every character in the stack to the output Vector
        for sign in stack.chars() {
            output_vec.push(sign.to_string());
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
    //Array used to check if a character is an operator
    let operators_arr = ['^', '*', '/', '+', '-'];
    //The variable that is the value of the vector at the start of each iteration of the
    //while loop
    let mut current_vector: Vec<String> = rpn_vec;
    //The varaible that is the value of the vector at the end of each iteration of the
    //while loop  
    let mut update_vector: Vec<String> = Vec::new();
    //The index of the number that is at the start of the Reverse Polish Notation for the 
    //individual expression
    let mut position = 0;
    //While the current vector contains more than one item
    while current_vector.len() > 1 {
        //Iterating over the current vector for an operator
        for (i, item) in current_vector.iter().enumerate() {
            //If the item is longer than 1 it is clearly not an operator
            if item.len() > 1 {
                continue;
            }
            //Defining the variable that is equal to the value of the current character
            let chr = item.chars().nth(0).unwrap();
            //Checking if tha character is an operator
            if operators_arr.contains(&chr) {
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