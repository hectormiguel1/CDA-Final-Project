use std::io::{self, BufRead};
use threadpool::ThreadPool;
use rand::Rng;
use chrono::prelude::*;

const TRANS_TYPE :[&str;3] = ["CheckBalance", "Deposit", "Withdraw"];
const MIN_TRANS_TYPE: usize = 0;
const MIN_TRANS_AMOUNT: f32 = 0.0;
const MAX_TRANS_AMOUNT: f32 = 10000.0;
const MIN_TRANS_COUNT: u32 = 5;
const MAX_TRANS_COUNT: u32 = 20;
const MAX_NUM_THREADS: usize = 100;

#[derive(Debug, Clone, Copy)]
struct Account {
    account_id: u64,
    account_balance: f32,
}
/*
This program will read accounts from stdin, and create transactions from those accounts.
The number of transactions generated per account is controlledby MIN_TRANS_COUNT and MAX_TRAS_COUNT constants
Un order to spidite the transaction generation proccess, the application uses a threadpool of up to 100 threads 
to parellalise the account creation proccess. In instances where the number accounts is less than the Max number of threads,
then we can spun up each account on a seperate thread and have the generate the accounts. 
*/
fn main() {
    //Read the accounts from std. accounts is a Vector of Account
   let accounts = read_from_stdin();
   //Declare the size of the thread_pool variable. 
   let thread_pool_size;

   //This block of code is the logic which sets the thread pool size depending on the number of accounts read. 
   //The limit of 100 is set to not overtax the system. Keep in mind that currently i am testing on a Ryzen 7 3800x, which 
   // has  8 cores and 16 threads. so on those 8 cores, the threads will be timesliced. 
   {
       if accounts.len() < MAX_NUM_THREADS {
            thread_pool_size = accounts.len() +1 ;
       } else {
            thread_pool_size = MAX_NUM_THREADS +1;
       }
   };
   //Here we can initialize our new thread pool. 
   /*
    The reason we use a thread pool is that we can have a fixed number of threads doing the work and schedule operations 
    to be done by the threads when the complete. thus offering maxium efficiecy and removes manul thread spawning. 
   */
   let thread_pool = ThreadPool::new(thread_pool_size);
   let mut random = rand::thread_rng();

   //The loop will iterate over the accounts and enqueue operations to be exequeted by the threads. 
   //Due To rust ownership rules we have to clone each account into the gen_transaction function that will be thread, 
   //This is done to prevent dangleing pointers in the case that the main account becomes out of scope and the thread is 
   // still executing.
   for account in 0..accounts.len() {
    let num_trans = random.gen_range(MIN_TRANS_COUNT, MAX_TRANS_COUNT);
    let passed_account = accounts[account].clone();
    let passed_num_trans = num_trans.clone();
       thread_pool.execute( move || {
           gen_transactions(passed_account, passed_num_trans)
       });
   }
   //We syncronize all the threads, and wait for the to finish before ending the program, this ensures that 
   // all the transactions are properly printed on the screen before we print the final control message. 
   thread_pool.join();
   //Control message, when this is printed on the screen, target application should stop reading from STDIN. 
   println!("DONE");
}

/*
    this function reads from stdin in a predefined format:
        accountid account_balance
    after reading from stdin, the function will parse the input into u64 and f32, which are 
    unsinged int 64-but or unsigned longs, and floating point 32 bit (single pressision).
    The account will the push those accounts into a vector of type Account, and return ownership to the calling function. 
    The return of ownership ensures no dangleing pointers. as Vectors are heap allocated.  
*/
fn read_from_stdin() ->  Vec<Account> {
    //Define the new input (stdin)
    let input = io::stdin();
    //Define new Vector of type account
    let mut account_vec: Vec<Account> = Vec::new();

    //Read lines from input
    for lines in input.lock().lines() {
        //unwrap function returns the value in case there was no error reading. (This will fail if we can not read from stdin)
       let in_str = lines.unwrap();

       //As per the predefined format, the line is split on the white space and they are collected on a vector of strings. 
       let input_splited = in_str.split_whitespace().collect::<Vec<&str>>();

       //Check for control, return the current account vector. 
       if input_splited[0] == "DONE" {
           return account_vec;

       } else {
            //Parse input and push the new Account into the Account vector. 
            let account_id = input_splited[0].parse::<u64>().unwrap();
            let account_balance = input_splited[1].parse::<f32>().unwrap();

            account_vec.push(Account{account_id: account_id, account_balance: account_balance});
        }
    }
    return account_vec;
}
/*
    This is the function which will be run by multiple threads. 
    @param: account: Account which will be used to generate transactions from. 
    @param: num_trans: Unsigned 32bit intiger which will dictate how many transactions to generate per account.Account

    This function prints a predefined format for transactions:
        {time in nanoseconds} {account id of transaction} {type of transaction} {amoount}
    For the porpuse of simplicity and to determine which transaction happened when, we use time in nano seconds as it offers the highest precision. 
    Chrono package is used to get tim in nano seconds (Thread safe and has built in mutex). 
    rand is use as the random number generator, it is also thread safe. 
*/
fn gen_transactions(account: Account, num_trans: u32) {
    let mut random = rand::thread_rng();
    //This is the used to pass the new account to the other programs, defines the starting balance of the account. 
    println!("{time} {account_id} {trans_type} {amount}", time=0, account_id=account.account_id, trans_type=1, amount=account.account_balance);
    //Iterate from 0 to desired number of transactions to generate transactions using random data. 
    for _ in 0..num_trans {
        let trans_time = Utc::now().timestamp_nanos();
        let trans_type = random.gen_range(MIN_TRANS_TYPE, TRANS_TYPE.len());
        let trans_amount = random.gen_range(MIN_TRANS_AMOUNT, MAX_TRANS_AMOUNT);
        println!("{time} {account_id} {trans_type} {amount}", time=trans_time, account_id=account.account_id, trans_type=trans_type, amount=trans_amount);
    }

}
