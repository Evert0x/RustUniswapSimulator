struct Token {
    name: String,
    symbol: String,
    decimals: u8,
}

struct UniswapPool {
    token1: Token,
    token2: Token,
    token1_amount: u128,
    token2_amount: u128,
    k: u128,
}

impl UniswapPool {

    fn new(token1: Token, token2: Token, token1_amount: u32, token2_amount: u32) -> Self {
        UniswapPool {
            token1,
            token2,
            token1_amount: u128::from(token1_amount),
            token2_amount: u128::from(token2_amount),
            k: u128::from(token1_amount) * u128::from(token2_amount)

        }
    }
    fn exactInputToken1ToOutput(&mut self, Input: u128) -> u128 {
        self.token1_amount += Input;

        let original = self.token2_amount;
        self.token2_amount = self.k / self.token1_amount;
        return original - self.token2_amount;
    }
    fn exactInputToken2ToOutput(&mut self, Input: u128) -> u128 {
        self.token2_amount += Input;

        let original = self.token1_amount;
        self.token1_amount = self.k / self.token2_amount;
        return original - self.token1_amount;
    }
}

fn getLine() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}

fn main() {
    // Statements here are executed when the compiled binary is called
    let token1 = Token{name: String::from("FLUX"), symbol: String::from("FLX"), decimals: 0};
    let token1_amount = 100000;
    let token2 = Token{name: String::from("WRAPPED ETH"), symbol: String::from("WETH"), decimals: 0};
    let token2_amount = 100000;
     // Print text to the console
     println!("TOKEN 1
     \tname:{}
     \tsymbol:{}
     \tdecimals:{}", token1.name, token1.symbol, token1.decimals);

     println!("TOKEN 2
     \tname:{}
     \tsymbol:{}
     \tdecimals:{}\n", token2.name, token2.symbol, token2.decimals);

    let mut pool = UniswapPool::new(token1, token2, token1_amount, token2_amount);


    loop {
        println!("POOL STATE
\t{}:{}
\t{}:{}\n", pool.token1.symbol, pool.token1_amount, pool.token2.symbol, pool.token2_amount);
        loop {
            println!("swap 10000 {} for {}? (y/n)", pool.token1.symbol, pool.token2.symbol);
            let line = getLine();
            if line == "y" {
                let tokens = pool.exactInputToken1ToOutput(10000);
                println!("Received {} {}", tokens, pool.token2.symbol)
            }
            if line == "y" || line == "n" { break; }
        }
        loop {
            println!("swap 10000 {} for {}? (y/n)", pool.token2.symbol, pool.token1.symbol);
            let line = getLine();
            if line == "y" {
                let tokens = pool.exactInputToken2ToOutput(10000);
                println!("Received {} {}", tokens, pool.token1.symbol)
            }
            if line == "y" || line == "n" { break; }
        }
    }
    // let mut line = String::new();
    // let b1 =std::io::stdin().read_line(&mut line).unwrap();
}
