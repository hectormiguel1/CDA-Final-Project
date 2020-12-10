use std::io::{self, BufRead};
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
enum TransType {
    Check_Balance, Deposit, Withdraw, Error
}
#[derive(Debug, Clone)]
struct Account {
    account_id: u64,
    account_balance: f32,
}
#[derive(Debug, Clone)]
struct Transaction {
    timestamp :u64,
    account_id : u64,
    trans_type: TransType,
    amount: f32,
}
#[derive(Debug, Clone)] 
struct Job {
    account: Account, 
    transactions: Vec<Transaction>,
    total_num: usize
}

fn main() {
    let (trans, accounts) = read_from_stdin();
    let num_accounts = accounts.len();
    let jobs = get_jobs(accounts, trans);
    let thread_pool_size;
    {
        if jobs.len() < 10000 {
             thread_pool_size = (jobs.len() / 2) +1;
        } else {
             thread_pool_size = (jobs.len() / 10) +1;
        }
    };
    let thread_pool = ThreadPool::new(thread_pool_size);

    for job in 0..jobs.len() {
        let mut passed_job = jobs[job].clone();
        thread_pool.execute(move || {process_transactions(&mut passed_job)});
        thread_pool.join();
    }
    let mut total_transactions = 0;
    for job in 0..jobs.len() {
        total_transactions += jobs[job].total_num;
    }
    thread_pool.join();
    println!("Proccessed {total_transactions} Transactions, for {num_accounts} Accounts", total_transactions=total_transactions, num_accounts=num_accounts);
}

fn read_from_stdin() -> (Vec<Transaction>, Vec<Account>) {
    let input = io::stdin();
    let mut trans_vec: Vec<Transaction> = Vec::new();
    let mut account_vec: Vec<Account> = Vec::new();

    for lines in input.lock().lines() {
       let in_str = lines.unwrap();

       let input_splited = in_str.split_whitespace().collect::<Vec<&str>>();

       if input_splited[0] == "DONE" {
           return (trans_vec,account_vec);
       }

       let trans_time = input_splited[0].parse::<u64>().unwrap();
       let trans_acc_id = input_splited[1].parse::<u64>().unwrap();
       let trans_type = input_splited[2].parse::<u64>().unwrap();
       let trans_amount = input_splited[3].parse::<f32>().unwrap();
       let trnas_type_converted : TransType;

       match trans_type {
           0 => trnas_type_converted = TransType::Check_Balance,
           1 => trnas_type_converted = TransType::Deposit,
           2 => trnas_type_converted = TransType::Withdraw,
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

fn get_jobs(accounts: Vec<Account>, transactions: Vec<Transaction>) -> Vec<Job> {
    let mut job_vec : Vec<Job> = Vec::new();
    for account in 0..accounts.len() {
        let _account = accounts[account].clone();
        let mut _transactions: Vec<Transaction> = Vec::new();
        for transaction in 0..transactions.len() {
            if transactions[transaction].account_id == _account.account_id {
                _transactions.push(transactions[transaction].clone());
            }
        }
        job_vec.push(Job{account: _account, transactions: _transactions.clone(), total_num: _transactions.len()});
    }
    return job_vec;
}

fn process_transactions(mut job: &mut Job) {
    let mut processed_trans = 0;
    for transaction in 0..job.transactions.len() {
        match job.transactions[transaction].trans_type {
            TransType::Deposit => {job.account.account_balance += job.transactions[transaction].amount; processed_trans += 1;}
            TransType::Withdraw => {
                if job.transactions[transaction].amount > job.account.account_balance {
                    println!("Trnsactions: {:?} Declined!, Transaction amount exeeds account balace", job.transactions[transaction]);
                }
                else {
                    job.account.account_balance -= job.transactions[transaction].amount;
                    processed_trans += 1;
                }
            }
            TransType::Check_Balance => {/* Do Nothing */}
            _ => panic!("Arrived at imposible state")
        }
    }
    println!("Proccessed {processed_trans} transactions out of {num_transactions}.\nAccount: {account_id},\nFinal balance {balance}",
            processed_trans=processed_trans,num_transactions = job.total_num, account_id=job.account.account_id, balance=job.account.account_balance);
}