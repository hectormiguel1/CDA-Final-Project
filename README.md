# CDA-Final-Project
This program was made on rust, binaries included are on windows and linux. 
included binaris are stored in each target directory for each of the programs.

# Programs:
<ins>account-gen</ins>: This is a simple program that n unique accounts and their starting balance, they accounts generated are printed to stdout.

<ins>transactions-gen</ins>: This applications reads accounts generated by account-gen and makes transactions from them, it creates a transcations at time 0 of transaction type DEPOSI, 
  to indicate that this is a new account. It will generate between 5 and 20 transactions per account. 

<ins>tranasation-proccessor</ins>: takes transactions in the format generated by transaction-gen to batch proccess them in a multithreaded/mulicore system. 

## Account-gen:
Program Usage:

    ./account_generator [options] 

    [OPTIONS]: 

    -n [NUMBER]: number of accounts to generate.
 
 This is the only application that requires command line arguments. 
 
 To compile this application, with cargo and rustc installed, go inside account-gen directory and run cargo build --release, for a release version of the tool, 
 this linux and windows version can be found inside the target/release subfolders. 
 
 Accounts are printed in the format: {account_id} {accout_balance}
 
 ## Transactions-gen:
 To compile this application, with cargo and rustc installed, go inside account-gen directory and run cargo build --release, for a release version of the tool, 
 this linux and windows version can be found inside the target/release subfolders. 

 This application also employes multithreads/multicore design to generate transactions for up to 100 accounts at a time. 

 Application reads Account in format: {account_id} {account_balance}

 Application prints transaction in format: {transaction_time_nanoseconds} {transaction_account_id} {transaction_type} {transaction_amount}
 
 ## Tranasation-proccessor:
 To compile this application, with cargo and rustc installed, go inside account-gen directory and run cargo build --release, for a release version of the tool, 
 this linux and windows version can be found inside the target/release subfolders. 

 Applications uses multithreading/multicore desgn to proccess batches of up 100 Jobs {account and pending tranasactions} at a time. 

 Application reads transactions in format: {transaction_time_nanoseconds} {transaction_account_id} {transaction_type} {transaction_amount}


## Running them togher:
To take advantage of 3 applications you can use a pipe to pass the outout of one application to become the input for another appliction, on windows this can be achived
using powershell. Linux and MacOS have pipe builtin to their terminal application. 

Example: 

 <ins>Linux/MacOs</ins>: 

 './account-gen -n 20 | transaction-gen | transaction-proccessor' <- This will run the 3 applications at the same time, which each wait for outfrom the other. 

 <ins>Windows: IN-POWERSHELL!!!</ins>
 
  '/account-gen.exe -n 20 | transaction-gen.exe | transaction-proccessor.exe'   
 
 
 ## Current Tests: 
 I have tested the application with up to 100K accounts generated, this inturn generates from 1.2M to 6M transactions, run time for this operation is around 2 mintes.

 This is when tested on a Ryzen 7 3800x with 8 cores and 16 total threads at a time. because there were 100 threads spawn, these had to time splice on each thread/core on the cpu. 
 