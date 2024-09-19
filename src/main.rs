use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Wallet{
    balances:HashMap<String,f64>,//存储每种货币的余额
}

impl  Wallet{
    fn new() -> Wallet{
        Wallet {
            balances: HashMap::new()
        }
    }

    fn deposit(&mut self, currency: &str, amount: f64) {
        let balance = self.balances.entry(currency.to_string()).or_insert(0.0);
        *balance += amount;
        println!("成功存入 {}: ${:.2}", currency, amount);
    }

    fn withdraw(&mut self, currency: &str, amount: f64) {
        let balance = self.balances.entry(currency.to_string()).or_insert(0.0);
        if amount <= *balance {
            *balance -= amount;
            println!("成功取出 {}: ${:.2}", currency, amount);
        } else {
            println!("{} 余额不足，无法取出: ${:.2}", currency, amount);
        }
    }

    fn get_balance(&self, currency: &str) -> f64 {
        *self.balances.get(currency).unwrap_or(&0.0)
    }
}
#[tokio::main]
fn main() {
    let mut wallet = Wallet::new();

    // 进行存款
    wallet.deposit("Bitcoin", 100.0);
    wallet.deposit("Litecoin", 50.0);
    wallet.deposit("LuckyCoin", 30.0);
    wallet.deposit("BellsCoin", 20.0);
    wallet.deposit("DogeCoin", 10.0);

    // 查看余额
    println!("当前余额：");
    for currency in ["Bitcoin", "Litecoin", "LuckyCoin", "BellsCoin", "DogeCoin"].iter() {
        println!("{}: ${:.2}", currency, wallet.get_balance(currency));
    }

    // 进行取款
    wallet.withdraw("Bitcoin", 30.0);
    println!("取款后余额：");
    println!("Bitcoin: ${:.2}", wallet.get_balance("Bitcoin"));

    // 调用各公链的 API 示例（具体实现需根据 API 文档）
    // await get_balance_from_chain("Bitcoin").await;
}

async fn get_balance_from_chain(currency: &str){
    match currency{
        "BitCoin"=>{
            // 调用BitcoinAPI
        }
        "LiteCoin"=>{
            // 调用LitecoinAPI
        }
        "LuckyCoin"=>{
            // 调用LuckycoinAPI
        }
        "BellsCoin"=>{
            // 调用LuckycoinAPI
        }
        "DogeCoin"=>{
            // 调用LuckycoinAPI
        }
    }
}
