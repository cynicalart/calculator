//Arrays which will be used to check if something is an operator, bracket or a function
const BRACKETS_ARR: [char; 4] = ['(', ')', '{', '}'];
const OPERATORS_ARR: [char; 5] = ['^', '*', '/', '+', '-'];
const FUNCTIONS_ARR: [&str; 16] = ["log", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan", "arccsc", "arcsec", "arccot", "to_deg", "to_rad"];
//Array containing the precedences of the operators in the same order as the operators
//in the operator array
const PRECEDENCE: [i32; 5] = [4, 3, 3, 2, 2];
//Defining mathematical constants that may be used
const EULER: f64 = std::f64::consts::E;
const PI: f64 = std::f64::consts::PI;

//Defining the expression_to_vec function, which converts the original expression
//String into a Vector of strings
pub fn expression_to_vec(expression: String) -> Vec<String> {
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
    let mut character_string = String::new();
    //Iterating over the new expression String
    for (i, chr) in cleaned_expression.chars().enumerate() {
        //Initialising the variables for the previous character and the next character
        //as effective null values in case the current index is the first or the last
        //index
        let mut previous_chr: char = '_';
        let mut next_chr: char = '_';
        //If the index is not the first or last index
        if i > 0 {
            //Redefine the variable for the previous character
            previous_chr = cleaned_expression.chars().nth(i - 1).unwrap();
        }

        if i + 1 < cleaned_expression.len() {
            //Redefine the variable for the next character
            next_chr = cleaned_expression.chars().nth(i + 1).unwrap();
        }
        //In the case of the general log function, a comma will be used to separate the 
        //base and the value and so this accounts for that
        if chr == ',' {
            if !num.is_empty() {
                output_vec.push(num);
                num = String::new();
            }
        }
        //This character will only ever be part of a function name
        if chr == '_' {
            character_string.push(chr);
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
                    for digit in EULER.to_string().chars() {
                        num.push(digit);
                    }
                //If the first character is p and the next character is i, it's pi
                } else if chr == 'p' && next_chr == 'i' {
                    continue;
                //Otherwise it must be part of a function or an algebraic term 
                } else {
                    character_string.push(chr);
                }
            //Now we can check the previous character and last character
            } else if i + 1 < cleaned_expression.len() {
                //If the current character is 'i' and the previous character is 'p', then
                //it must be pi
                if chr == 'i' && previous_chr == 'p' {
                    for digit in PI.to_string().chars() {
                        num.push(digit);
                    } 
                //If the current character is 'e' and the previous character is not a letter
                //then it must be Euler's constant
                } else if chr == 'e' && !previous_chr.is_alphabetic() {
                    for digit in EULER.to_string().chars() {
                        num.push(digit);
                    }
                } else if chr == 'p'  && next_chr == 'i' {
                    continue;
                //Otherwise it must be part of a function or an algebraic term
                } else {
                    character_string.push(chr);
                }
            //If we are at the end of the string we can only check for the previous character
            } else {
                //If the current character is 'i' and the previous character is 'p', then
                //it must be pi
                if chr == 'i' && previous_chr == 'p' {
                    for digit in PI.to_string().chars() {
                        num.push(digit);
                    } 
                //If the current character is 'e' and the previous character is not a letter
                //then it must be Euler's constant
                } else if chr == 'e' && !previous_chr.is_alphabetic() {
                    for digit in EULER.to_string().chars() {
                        num.push(digit);
                    }
                //Otherwise it must be part of a function or an algebraic term
                } else {
                    character_string.push(chr);
                }
            }
        }
        //If the charcter infront of the function or algebraic expression is a number, its an implied 
        //multiplication
        if previous_chr.is_numeric() && !character_string.is_empty() {
            output_vec.push(num.to_string());
            output_vec.push(String::from("*"));
            num = String::new();
        } else if previous_chr == '}' && !character_string.is_empty() {
            output_vec.push(String::from("*"));

        }
        //If the current character is an operator or a bracket
        if OPERATORS_ARR.contains(&chr) || BRACKETS_ARR.contains(&chr) {
            //Checking if the character is a minus sign
            if chr == '-' {
                //If the minus sign is part of a number, push it to the num string
                if i == 0 || OPERATORS_ARR.contains(&previous_chr) || BRACKETS_ARR.contains(&previous_chr) {
                    if !(previous_chr == ')' || previous_chr == '}') {
                        num.push(chr);
                        //Continues, so that any further changes do not occur at this index
                        continue;
                    }
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
                //and if the previous character was a closing bracket, or a number, or an algebraic
                //term
                if i > 0 {
                    if previous_chr == ')' || previous_chr.is_numeric() || previous_chr.is_alphabetic() {
                        //Push a multiplication sign to the output vector, as this is an 
                        //implied multiplication
                        output_vec.push(String::from("*"));
                    }
                }
            }
            //If the sign is an opening curly bracket, it means we are at the end of the
            //function string
            if chr == '{' && FUNCTIONS_ARR.contains(&&character_string.to_string()[..]) {
                output_vec.push(character_string);
                character_string = String::new();
            }

            if !chr.is_alphabetic() && !FUNCTIONS_ARR.contains(&&character_string.to_string()[..]) {
                for (j, algebraic_term) in character_string.chars().enumerate() {
                    output_vec.push(algebraic_term.to_string());

                    if j + 1 < character_string.len() {
                        output_vec.push(String::from("*"));
                    }
                }

                character_string = String::new();
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
            //If the num String is not empty
            if !num.is_empty() {
                //Add it to the output Vector
                output_vec.push(num);
            }
            //This must be an algebraic expression as a function should end in a closing curly bracket
            if !character_string.is_empty() {
                for (j, algebraic_term) in character_string.chars().enumerate() {
                    output_vec.push(algebraic_term.to_string());

                    if j + 1 < character_string.len() {
                        output_vec.push(String::from("*"));
                    }
                }

                character_string = String::new();
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
pub fn rpn(expression_vec: Vec<String>) -> Vec<String> {
    //Initiating the output Vector variable
    let mut output_vec: Vec<String> = Vec::new();
    //Itiating the stack variable, where any operators and brackets will go initially
    let mut stack: Vec<String> = Vec::new();
    //Iterating over the expression Vector
    for (i, item) in expression_vec.iter().enumerate() {

        println!("Stack: {:?}", stack);
        //Defining the variable that is equal to the value of the current character
        let chr = item.chars().nth(0).unwrap();
        //If the string is a number, add it to the output Vector
        if item.parse::<f64>().is_ok() || (chr.is_alphabetic() && item.len() == 1) {
            if i > 0 {
                if expression_vec[i - 1].parse::<f64>().is_ok() {
                    let leading_stack = stack[0].chars().nth(0).unwrap();

                    if !(leading_stack == '{' || leading_stack == '(') {
                        output_vec.push(stack[0].to_string());
                        stack.remove(0);
                    }
                }
            }

            output_vec.push(item.to_string());
            //Continuing as we no longer need to do anything with this item
            continue;
        } 

        //If the item is a function
        if item.len() > 1 && FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
            stack.insert(0, item.to_string());
            continue;
        }

        //Since we now know the character is definitely a sign, this wil just add the
        //character to the stack if the stack is empty 
        if stack.is_empty() {
            stack.push(chr.to_string());
        //Otherwise, if the stack is not empty
        } else {
            //Defining the variable equal to the item at the top of the stack
            let leading_stack_item = &stack[0];
            //If the item at the top of the stack is a function, add the item to the stack
            if FUNCTIONS_ARR.contains(&&leading_stack_item.to_string()[..]) {
                if OPERATORS_ARR.contains(&chr) {
                    output_vec.push(leading_stack_item.to_string());
                    stack.remove(0);
                    stack.insert(0, chr.to_string());
                } else if BRACKETS_ARR.contains(&chr) {
                    if chr == '(' || chr == '{' {
                        stack.insert(0, chr.to_string());
                    }
                }
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

                        let mut new_stack: Vec<String> = Vec::new();

                        for (j, item) in stack.iter().enumerate() {
                            if j > 0 {
                                new_stack.push(item.to_string())
                            }
                        }

                        new_stack.insert(0, chr.to_string());
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
                        //If the character is a bracket, add it to the new stack
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
                for (j, item) in stack.iter().enumerate() {
                    //If the item is a function, add it to the output and continue
                    if item.len() > 1 && FUNCTIONS_ARR.contains(&&item.to_string()[..]) {
                        output_vec.push(item.to_string());
                        continue;
                    }

                    let sign = item.chars().nth(0).unwrap();
                    //If the current character in the stack is an opening bracket, break
                    //the loop
                    if sign == '(' || sign == '{' {
                        if j + 1 < stack.len() && FUNCTIONS_ARR.contains(&&stack[j + 1].to_string()[..]) {
                            output_vec.push(stack[j + 1].to_string());
                        }
                        break;
                    //Otherwise, add the character to the output Vector
                    } else {
                        output_vec.push(sign.to_string());
                    }
                }
                //Removing any of the characters that were added to the output Vector
                //and the opening bracket from the stack
                let mut new_stack: Vec<String> = Vec::new();
                let mut position = 0;

                for j in 0..stack.len() {
                    let current_character = stack[j].chars().nth(0).unwrap();
                    if stack[j].len() == 1 && BRACKETS_ARR.contains(&current_character) {
                        if j + 1 < stack.len() {
                            if FUNCTIONS_ARR.contains(&&stack[j + 1].to_string()[..]) {
                                position = j + 2; 
                            } else {
                                position = j + 1;
                            }

                            break;
                        } else {
                            position = j;

                            break;
                        }
                    } 
                }

                for j in position..stack.len() {
                    new_stack.push(stack[j].to_string());
                }

                stack = new_stack;
                println!("Stack: {:?}", stack);
            }
        }
    }
    //If the stack is not empty
    if !stack.is_empty() {
        //Add every item in the stack to the output Vector
        for item in &stack {
            if !BRACKETS_ARR.contains(&item.chars().nth(0).unwrap()) {
                output_vec.push(item.to_string());
            }
        }
    }
    //Returning the output Vector
    output_vec
}