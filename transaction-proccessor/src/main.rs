use std::io::{self, BufRead};
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
enum TransType {
    CheckBalance, Deposit, Withdraw, Error
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

const MAX_NUM_THREADS: usize = 100;

/*
    This program contains the actual transaction processing bits. 
    In this program we have read transactions from stdin, this transactions
    are used to recreate accounts internally and also to create "Jobs", these jobs will then
    be worked on by threads. 
    A Job represent an account and all the pending transactions for that account, transactions are 
    ordered in a FIFO fashion and thus, hazards such as Read after Write are not a concern, transactions are
    proccessed in a loop on at a time in a threaded fashion.  There is currently a limit of 100 threads that can be created by this program.
    Disclaimer:  Currently the biggest buttleneck in the program is reading all the transactions from stdin, this currently can only be done single 
    threaded. With more time a chanel implementation into the threads could make this more efficent, as we could have the main thread, read and the 
    dispatch work to the threads when they read ecah line, thus making the program much more efficient. 
    Disclaimer 2: An attempt to make this program on a GPU was made, i tried, Vulkan compute shaders, OpenCL, and HIP (Amd open source competitor to CUDA). 
                    There are many limitations which prevented the use of either implementation, mostly time contrains, as both Vulkan compute shaders and OpenCL 
                    require learning an entire new programming language, and even so i kept getting segfaults when coping data over to the gpu, sometimes reuqireing 
                    a hard reset of the hardware. For the second part HIP, well this is a brand new language for GPGPU (General Porpuse GPU) and there is very little 
                    documentation as of yet, and most importantly is misshing shared memory from CUDA which would have solved the segfaults when copying data to GPU. 
                    Lastly is CUDA (Nvidia Only), i only have AMD hardware and thus could not write an application and test it in cuda. 
    Proccess for this function is to read all the transactions and accounts from stdin, this are stored in Transaction and Account vectors.
    After we have the accounts and the transactions we will build a vector of Jobs. 
    Spawn a Thread pool depending on the number of jobs (Capped at maximum 100 threads). 
    Enqueue jobs to the thread pool by iterating through the jobs vector. 
    Lastly The appliation will print a message explaining the number of transactions that were proccessed and the number of accounts. 
*/
fn main() {
    //Read transactions and accounts 
    let (trans, accounts) = read_from_stdin();
    //This is used for printing at the end, thus must be done now as the ownership is lost to the function that creates jobs. 
    let num_accounts = accounts.len();
    //Create the jobs
    let jobs = create_jobs(accounts, trans);
    //This and the declaration, variable is initialized in the code block bellow. 
    let thread_pool_size;
    let mut total_transactions = 0;
    //Code block to initialize the thread_pool_size
    {
        if jobs.len() < MAX_NUM_THREADS {
             thread_pool_size = jobs.len() + 1;
        } else {
             thread_pool_size = MAX_NUM_THREADS + 1;
        }
    }
    //Create the theradpool (Capped at 100 threads)
    let thread_pool = ThreadPool::new(thread_pool_size);
    //Iterate through jobs and enqueue the jobs on the thread pool.
    for job in 0..jobs.len() {
        let mut passed_job = jobs[job].clone();
        thread_pool.execute(move || {process_transactions(&mut passed_job)});
        total_transactions += jobs[job].total_num;
    }
    //Wait for all the threads to finish.
    thread_pool.join();
    //Print final message. 
    println!("Proccessed {total_transactions} Transactions, for {num_accounts} Accounts", total_transactions=total_transactions, num_accounts=num_accounts);
}
/*
    This function is in charge of reading the input from STDIN
    New accounts are created when they are first encountered, the program that generates the accounts, 
    will always print a depost at time 0 to indigate that this a new account. 
    @return (Vec<Transaction>, Vec<Account>): Touple of Transaction Vector and Account vector, this are heap allocated structures
    and thus their ownership is passed to the calling function. 
    Disclaimer: This is an area for improvement, currently the reading is done single threaded and on large input sizes (ie millions) it can take a few minutes to read
    all the input. 
*/
fn read_from_stdin() -> (Vec<Transaction>, Vec<Account>) {
    //Define input from stdin
    let input = io::stdin();
    //Define transactions Vector (mut means mutable by default rust is imutable data)
    let mut trans_vec: Vec<Transaction> = Vec::new();
    //define Account vector (mut means mutable by default rust is imutable data)
    let mut account_vec: Vec<Account> = Vec::new();

    //Loop through each line in the input
    for lines in input.lock().lines() {
        //If grab the entire line, and grab value if no error
       let in_str = lines.unwrap();
        //Split the input line along the white spaces, collect them into a vector of strings.
       let input_splited = in_str.split_whitespace().collect::<Vec<&str>>();
        //Check for control singal.
       if input_splited[0] == "DONE" {
           return (trans_vec,account_vec);
       }
       //Parse all the input from the lines vector (if no error grab the value)
       let trans_time = input_splited[0].parse::<u64>().unwrap();
       let trans_acc_id = input_splited[1].parse::<u64>().unwrap();
       let trans_type = input_splited[2].parse::<u64>().unwrap();
       let trans_amount = input_splited[3].parse::<f32>().unwrap();
       let trnas_type_converted : TransType;

       //Match the converted intiger for trans_type with the expected transaction type. 
       match trans_type {
           0 => trnas_type_converted = TransType::CheckBalance,
           1 => trnas_type_converted = TransType::Deposit,
           2 => trnas_type_converted = TransType::Withdraw,
           _ => trnas_type_converted = TransType::Error, 

       }
       //Check if the account exists in our accounts vector
       let mut contained : bool = false;
       for index in account_vec.iter_mut() {
           if index.account_id == trans_acc_id {
                contained = true;
           }
       }
       //If the account does not exist in the account vector, add the new account. 
       if !contained {
            account_vec.push(Account{account_id: trans_acc_id, account_balance: trans_amount});
       }
       //If it was found, then add the transaction into the transaction vector. 
       if contained {
           trans_vec.push(Transaction{timestamp: trans_time, account_id: trans_acc_id, trans_type: trnas_type_converted, amount: trans_amount})
       }

    }
    //Final return, we should never end here, this will only be called if we reached end of stdin without encoutering the "DONE". 
    return (trans_vec, account_vec);
}
/*
    This function will create the jobs from the accounts and transactions vectors. 
    @param: accounts: Accounts vectors, stores all the accounts that were read from stdin. 
    @param: transaction: All transactions that were read from stdin. 
    @return Vector of Jobs. 

    This function will iterate over the accounts, and then inside that loop, iterate over the transations, 
    when account_id == trnsaction_account_id, push transaction into job[account].transatioins, the data is cloned, because, the 
    account and transactions vectors will be dropped from memory when this function finishes executing (ie they will be freed). 
*/
fn create_jobs(accounts: Vec<Account>, transactions: Vec<Transaction>) -> Vec<Job> {
    //Intialize new job vector 
    let mut job_vec : Vec<Job> = Vec::new();
    //Start iteration over the accounts vector
    for account in 0..accounts.len() {
        //clone the account, ie coppy the contents of account at accounts[account]. 
        let _account = accounts[account].clone();
        //iinitalize new transactions Vector this will contained the subset of transactions that belong to this account. 
        let mut _transactions: Vec<Transaction> = Vec::new();
        //Begin iterating over transactions vector 
        for transaction in 0..transactions.len() {
            //Case when to push the transaction into _transactions
            if transactions[transaction].account_id == _account.account_id {
                _transactions.push(transactions[transaction].clone());
            }
        }
        //After each iteration we can then push the new Job into the Jobs Vectors. 
        job_vec.push(Job{account: _account, transactions: _transactions.clone(), total_num: _transactions.len()});
    }
    //Final return of job vector. 
    return job_vec;
}
/*
    This is the function which will be ran on the threads. 
    @param mutable reference to Job, this is the job that will be worked on by the thread. 

    This function will iterate through the transctions on the job, and then make the needed changes on the
    account. In case that the transaction amount is above the account balance, the transaction will be declined and will not 
    be proccessed. At the end, we print a message containining the number of transactions that were proccessed, the account id, and the 
    new account balance. 
*/
fn process_transactions(mut job: &mut Job) {
    //Initiate to track proccessed tranasactions
    let mut processed_trans = 0;
    //Bigin iterations for transactions in the job
    for transaction in 0..job.transactions.len() {
        //Match the transaction type to handle diffrent types of transations
        match job.transactions[transaction].trans_type {
            //On deposit, trans_amount is added to acount_balance
            TransType::Deposit => {job.account.account_balance += job.transactions[transaction].amount; processed_trans += 1;}
            //On withdraw, which check for sufficient founds, if founds are available, we substract the amount from the account,
            //If founds are not available, the transaction is declined and message is printed with transaction information. 
            TransType::Withdraw => {
                if job.transactions[transaction].amount > job.account.account_balance {
                    println!("Trnsactions: {:?} Declined!, Transaction amount exeeds account balace", job.transactions[transaction]);
                }
                else {
                    job.account.account_balance -= job.transactions[transaction].amount;
                    processed_trans += 1;
                }
            }
            //On account balance check, we just proccess it by imcrementing the proccessed trans by 1
            TransType::CheckBalance => {processed_trans += 1}
            //Should never arrive at this state if the data was read successfully, application will stop and print the error message. 
            _ => panic!("Arrived at imposible state")
        }
    }
    //Final transaction tally report and account balance report. 
    println!("Proccessed {processed_trans} transactions out of {num_transactions}.\nAccount: {account_id},\nFinal balance {balance}",
            processed_trans=processed_trans,num_transactions = job.total_num, account_id=job.account.account_id, balance=job.account.account_balance);
}