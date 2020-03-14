use std::io;

trait Evaluable {
    fn eval(&self) -> f64;
}

enum Operation<'a> {
    Exponent(&'a dyn Evaluable, &'a dyn Evaluable),
    Multiply(&'a dyn Evaluable, &'a dyn Evaluable),
    Divide(&'a dyn Evaluable, &'a dyn Evaluable),
    Add(&'a dyn Evaluable, &'a dyn Evaluable),
    Subtract(&'a dyn Evaluable, &'a dyn Evaluable)
}

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

impl Evaluable for String {
    fn eval(&self) -> f64 {
        self.parse().unwrap()
    }
}

fn main() {
    let mut expression = String::new();

    io::stdin().read_line(&mut expression)
        .expect("Failed to read line");

    expression.truncate(expression.len() - 1);

    let expression_vec = expression_to_vec(expression); 

    println!("{:?}", expression_vec);

    let rpn_vec = rpn(expression_vec);

    println!("{:?}", rpn_vec);

    let answer = evaluate_rpn(rpn_vec);
    println!("The answer is: {}", answer);
}

fn expression_to_vec(expression: String) -> Vec<String> {

    let mut cleaned_expression = String::new();

    for chr in expression.chars() {
        if chr.is_whitespace() == false {
            cleaned_expression.push(chr);
        }
    }

    let mut output_vec: Vec<String> = Vec::new();
    let mut num = String::new();
    let mut sign = String::new();
    let brackets_arr = ['(', ')'];
    let operators_arr = ['^', '*', '/', '+', '-'];

    for (i, chr) in cleaned_expression.chars().enumerate() {

        if chr.is_numeric() || chr == '.' {
            num.push(chr);
        }

        if operators_arr.contains(&chr) || brackets_arr.contains(&chr) {

            if chr == '-' {
                if i == 0 || operators_arr.contains(&cleaned_expression.chars().nth(i - 1).unwrap()) {
                    num.push(chr);
                    continue;
                } 
            } 

            if chr == '*' && &cleaned_expression.chars().nth(i + 1).unwrap() == &chr {
                sign.push('^');
            } else if i > 0 && &cleaned_expression.chars().nth(i - 1).unwrap() == &chr && chr == '*' {
                continue;
            } else {
                sign.push(chr);
            }

            if !num.is_empty() {
                output_vec.push(num);
            }

            if chr == '(' {
                if i > 0 && !operators_arr.contains(&cleaned_expression.chars().nth(i - 1).unwrap()) && cleaned_expression.chars().nth(i - 1).unwrap() == ')' {
                    output_vec.push(String::from("*"));
                }
            }

            output_vec.push(sign);
            num = String::new();
            sign = String::new();
        }

        if i + 1 >= cleaned_expression.len() {
            if !num.is_empty() {
                output_vec.push(num);
            }

            num = String::new();
        }

    }

    output_vec 
}

fn rpn(expression_vec: Vec<String>) -> Vec<String> {
    let mut output_vec: Vec<String> = Vec::new();
    let mut stack = String::new();
    let brackets_arr = ['(', ')'];
    let operators_arr = ['^', '*', '/', '+', '-'];
    let precedence = [4, 3, 3, 2, 2];

    for item in &expression_vec {

        println!("Stack: {}", stack);

        if item.len() > 1 {
            output_vec.push(item.to_string());
            continue;
        } 

        let chr = item.chars().nth(0).unwrap();

        if chr.is_numeric() {
            output_vec.push(item.to_string());
            continue;
        }

        if stack.is_empty() {
            stack.push(chr);
        } else {

            let leading_stack = stack.chars().nth(0).unwrap();

            if operators_arr.contains(&leading_stack) && operators_arr.contains(&chr) {

                let leading_stack_precedence = precedence[operators_arr.iter().position(|&operator| operator == leading_stack).unwrap()];
                let chr_precedence = precedence[operators_arr.iter().position(|&operator| operator == chr).unwrap()];

                if chr_precedence > leading_stack_precedence {
                    stack.insert(0, chr);
                } else if chr_precedence == leading_stack_precedence {
                    if chr == leading_stack {
                        stack.insert(0, chr);
                    } else {
                        output_vec.push(leading_stack.to_string());
                        stack = stack.replace(&leading_stack.to_string(), &chr.to_string());
                    }
                } else if chr_precedence < leading_stack_precedence {

                    let mut new_stack = String::new();
                    let mut bracket_encountered = false;

                    for sign in stack.chars() {
                        if brackets_arr.contains(&sign) {
                            new_stack.push(sign);
                            bracket_encountered = true;
                            continue;
                        }

                        let j = operators_arr.iter().position(|&operator| operator == sign).unwrap();
                        let sign_precedence = precedence[j];
                        if bracket_encountered == true {
                            new_stack.push(sign);
                        } else if sign_precedence >= chr_precedence {
                            output_vec.push(sign.to_string());
                        } else {
                            new_stack.push(sign);
                        }
                    }

                    stack = new_stack;

                    stack.insert(0, chr);
                }
            } else if operators_arr.contains(&chr) && brackets_arr.contains(&leading_stack) {
                stack.insert(0, chr);
            } else if chr == '(' {
                stack.insert(0, chr);
            } else if chr == ')' {
                for sign in stack.chars() {

                    if sign == '(' {
                        break;
                    } else {
                        output_vec.push(sign.to_string());
                    }
                }

                stack = (&stack[stack.chars().position(|bracket| bracket == '(').unwrap() + 1..stack.len()]).to_string();
                println!("Stack: {}", stack);
            }
        }
    }

    if !stack.is_empty() {
        for sign in stack.chars() {
            output_vec.push(sign.to_string());
        }
    }

    output_vec
}

fn evaluate_rpn(rpn_vec: Vec<String>) -> f64 {
    let mut value: f64 = 0.0;
    let answer: f64;
    let operators_arr = ['^', '*', '/', '+', '-'];
    let mut current_vector: Vec<String> = rpn_vec;
    let mut update_vector: Vec<String> = Vec::new();
    let mut position = 0;

    while current_vector.len() > 1 {
        for (i, item) in current_vector.iter().enumerate() {
            if item.len() > 1 {
                continue;
            }

            let chr = item.chars().nth(0).unwrap();

            if operators_arr.contains(&chr) {
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
                position = i - 2;
                println!("Position: {}", position);
                break;
            }
        }


        let mut i = 0;

        while i < current_vector.len() {
            if i == position {
                update_vector.push(value.to_string());
                i += 3 
            } else {
                update_vector.push(current_vector[i].to_string());
                i += 1;
            }
        }

        current_vector = Vec::new();

        for item in &update_vector {
            current_vector.push(item.to_string());
        }

        update_vector = Vec::new();

        println!("Current Vector: {:?}", current_vector);

    }

    answer = current_vector[0].eval();

    answer
}