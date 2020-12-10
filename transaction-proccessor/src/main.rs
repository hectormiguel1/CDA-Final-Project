use std::io::{self, BufRead};

#[derive(Debug)]
enum TransType {
    Check_Balance, Deposit, Withdraw, Error
}
#[derive(Debug)]
struct Account {
    account_id: u64,
    account_balance: f32,
}
#[derive(Debug)]
struct Transaction {
    timestamp :u64,
    account_id : u64,
    trans_type: TransType,
    amount: f32,
}

fn main() {
    let (mut trans, mut accounts) = read_from_stdin();
}

fn read_from_stdin() -> (Vec<Transaction>, Vec<Account>) {
    let input = io::stdin();
    let mut trans_vec: Vec<Transaction> = Vec::new();
    let mut account_vec: Vec<Account> = Vec::new();

    for lines in input.lock().lines() {
       let in_str = lines.unwrap();

       let input_splited = in_str.split_whitespace().collect::<Vec<&str>>();

       let trans_time = input_splited[0].parse::<u64>().unwrap();
       let trans_acc_id = input_splited[1].parse::<u64>().unwrap();
       let trans_type = input_splited[2];
       let trans_amount = input_splited[3].parse::<f32>().unwrap();
       let trnas_type_converted : TransType;

       match trans_type {
           "Check_Balance" => trnas_type_converted = TransType::Check_Balance,
           "Deposit" => trnas_type_converted = TransType::Deposit,
           "Withraw" => trnas_type_converted = TransType::Withdraw,
           _ => trnas_type_converted = TransType::Error, 

       }
       let mut contained : bool = false;
       for index in account_vec.iter_mut() {
           if index.account_id == trans_acc_id {
                contained = true;
           }
       }

       if !contained {
            account_vec.push(Account{account_id: trans_acc_id, account_balance: trans_amount});
       }
       if contained {
           trans_vec.push(Transaction{timestamp: trans_time, account_id: trans_acc_id, trans_type: trnas_type_converted, amount: trans_amount})
       }

    }

    return (trans_vec, account_vec);
}
