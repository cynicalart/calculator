use std::io;

mod format;
mod basic_calculation;
mod calculus;

use format::{expression_to_vec, rpn};

fn main() {
    let mut menu_choice = String::new();

    io::stdin().read_line(&mut menu_choice)
        .expect("Failed to read line");

    let menu_choice: u32 = menu_choice.trim().parse()
        .expect("Please type a number");
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

    if menu_choice == 1 {
        //Converting the expression Vector into another Vector of Strings in which the
        //Strings have been arranged according to Reverse Polish Notation (rpn)
        let rpn_vec = rpn(expression_vec);

        println!("{:?}", rpn_vec);
        //Defining the answer as the evaluated rpn_vec
        let answer = basic_calculation::evaluate_rpn(rpn_vec);
        println!("The answer is: {}", answer);
    } else if menu_choice == 2 {
        let derivative = calculus::calculate_derivative(expression_vec);
    }
}
