use std::io::{self, BufRead};
use threadpool::ThreadPool;
use rand::Rng;
use chrono::prelude::*;

const TRANS_TYPE :[&str;3] = ["Check_Balance", "Deposit", "Withdraw"];
const MIN_TRANS_TYPE: usize = 0;
const MIN_TRANS_AMOUNT: f32 = 0.0;
const MAX_TRANS_AMOUNT: f32 = 10000.0;
const MIN_TRANS_COUNT: u32 = 5;
const MAX_TRANS_COUNT: u32 = 20;

#[derive(Debug, Clone, Copy)]
struct Account {
    account_id: u64,
    account_balance: f32,
}

fn main() {
   let accounts = read_from_stdin();
   let thread_pool_size;
   {
       if accounts.len() < 10000 {
            thread_pool_size = (accounts.len() / 2) +1;
       } else {
            thread_pool_size = (accounts.len() / 10) +1;
       }
   };
   let thread_pool = ThreadPool::new(thread_pool_size);
   let mut random = rand::thread_rng();

   for account in 0..accounts.len() {
    let num_trans = random.gen_range(MIN_TRANS_COUNT, MAX_TRANS_COUNT);
    let passed_account = accounts[account].clone();
    let passed_num_trans = num_trans.clone();
       thread_pool.execute( move || {
           gen_transactions(passed_account, passed_num_trans)
       });
       thread_pool.join();
   }
   thread_pool.join();
   println!("DONE");

}

fn read_from_stdin() ->  Vec<Account> {
    let input = io::stdin();
    let mut account_vec: Vec<Account> = Vec::new();

    for lines in input.lock().lines() {
       let in_str = lines.unwrap();

       let input_splited = in_str.split_whitespace().collect::<Vec<&str>>();

       if input_splited[0] == "DONE" {
           return account_vec;

       } else {
            let account_id = input_splited[0].parse::<u64>().unwrap();
            let account_balance = input_splited[1].parse::<f32>().unwrap();

            account_vec.push(Account{account_id: account_id, account_balance: account_balance});
        }
    }
    return account_vec;
}

fn gen_transactions(account: Account, num_trans: u32) {
    let mut random = rand::thread_rng();
    for _ in 0..num_trans {
        let trans_time = Utc::now().timestamp_nanos();
        let trans_type = random.gen_range(MIN_TRANS_TYPE, TRANS_TYPE.len());
        let trans_amount = random.gen_range(MIN_TRANS_AMOUNT, MAX_TRANS_AMOUNT);
        println!("{time} {account_id} {trans_type} {amount}", time=trans_time, account_id=account.account_id, trans_type=trans_type, amount=trans_amount);
    }

}
