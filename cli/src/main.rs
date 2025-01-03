use zwl_parser::{Wallet, zwl::ZecWalletLite, ywallet::YWallet, WalletWriter};

fn main() {
    match Wallet::parse::<ZecWalletLite>("zecwallet-light-wallet.dat") {
        Ok(w) => {
            println!("{:#?}", w)
            // let _res = w.write::<YWallet>("generated_from_zwl.db");
        },
        Err(e) => println!("{}", e.to_string())
    }

    println!("\n=====\n");

    match Wallet::parse::<YWallet>("zec.db") {
        Ok(w) => {
            println!("{:#?}", w);            
        },
        Err(e) => println!("{}", e.to_string())
    }
    
}