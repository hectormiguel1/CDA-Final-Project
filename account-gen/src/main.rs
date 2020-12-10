use std::env;
use rand::Rng;

const MIN_ARGS :usize = 3;
const ARG_INDEX :usize = 1;
const EXPECTED_ARG: &str = "-n";
const NUM_ACCOUNTS_INDEX: usize = 2;
const MIN_ACCOUNT_RANGE :i32 = 100000;
const MAX_ACCOUNT_RANGE: i32 = 10000000;
const MIN_ACCOUNT_BALANCE: f32 = 100.00;
const MAX_ACCOUNT_BALANCE: f32 = 10000.00;

/*
Main function will accept 1 argument (the number of accounts to generate) and will
generate said number of accounts with random account numbers and starting balances.
 */
fn main() {
   let args: Vec<String> = env::args().collect();
    if args.len() < MIN_ARGS as usize {
        usage();
        return;
    } else if args[ARG_INDEX] != EXPECTED_ARG {
        usage();
        return;
    } else {
        let num_accounts_to_gen = args[NUM_ACCOUNTS_INDEX].parse::<i32>();
        match num_accounts_to_gen {
            Ok(number) => gen_accounts(number),
            Err(_) => error_parsing_int(args[NUM_ACCOUNTS_INDEX].clone())
        }
    }
}

//Prints program usage.
fn usage() {
    eprintln!("Program Usage: \n\
    ./account_generator [options] \n\
    [OPTIONS]: \n\
    -n [NUMBER]: number of accounts to generate. \n");
    println!("DONE");
}

//Prints error to STDERR if unable to parse passed in integer
fn error_parsing_int(error_string: String) {
    eprintln!("Error Parsing {}, into integer. Exiting...", error_string);
}
//Generates Random accounts with random initial balances.
//num_to_gen unique accounts will be generated.
//Acount numbers are verified to be unique.
fn gen_accounts(num_to_gen: i32) {
    let mut accounts = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..num_to_gen   {
        let mut account_number = rng.gen_range(MIN_ACCOUNT_RANGE,MAX_ACCOUNT_RANGE);
        let account_balance = rng.gen_range(MIN_ACCOUNT_BALANCE,MAX_ACCOUNT_BALANCE);
        while accounts.contains(&account_number) {
            account_number = rng.gen_range(MIN_ACCOUNT_RANGE,MAX_ACCOUNT_RANGE);
        }
        accounts.push(account_number);
        println!("{} {:.2}",account_number, account_balance);
    }
    println!("DONE");
}