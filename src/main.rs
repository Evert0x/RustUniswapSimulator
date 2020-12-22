use uint::construct_uint;

construct_uint! {
	pub struct U256(4);
}

struct Token {
    name: String,
    symbol: String,
    decimals: u8,
}

struct UniswapPool {
    token1: Token,
    token2: Token,
    token1Amount: U256,
    token2Amount: U256,
    k: U256,
}

impl UniswapPool {

}


fn main() {
    // Statements here are executed when the compiled binary is called
    let token1 = Token{name: String::from("FLUX"), symbol: String::from("FLX"), decimals: 18};
    let token1Amount = U256([1, 1, 1, 1]);
    let token2 = Token{name: String::from("WRAPPED ETH"), symbol: String::from("WETH"), decimals: 18};
    // Print text to the console
    println!("TOKEN 1
    \tname:{}
    \tsymbol:{}
    \tdecimals:{}", token1.name, token1.symbol, token1.decimals);

    println!("TOKEN 2
    \tname:{}
    \tsymbol:{}
    \tdecimals:{}", token2.name, token2.symbol, token2.decimals);

    // let mut line = String::new();
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
}
