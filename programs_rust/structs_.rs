struct Bitcoin {
    blocks_count: u32,
    difficulty_target: u32,
    network_info: String,
    verified: bool,
    block_id: u32,
}
impl Bitcoin {
    fn verify_block(&self) -> bool {
        if self.block_id <= self.difficulty_target {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let testnet = Bitcoin {
        blocks_count: 500,
        difficulty_target: 45,
        network_info: String::from("testnet"),
        verified: true,
        block_id: 1200,
    };

    let signet = Bitcoin {
        blocks_count: 500,
        difficulty_target: 45,
        network_info: String::from("signet"),
        verified: true,
        block_id: 50000,
    };
    println!("---- this is testnet ----");
    println!("{}", testnet.blocks_count);
    println!("{}", testnet.difficulty_target);
    println!("{}", testnet.network_info);
    println!("{}", testnet.verified);
    println!("block verification status : {}", testnet.verify_block());
    println!("---- this is signet ----");
    println!("{}", signet.blocks_count);
    println!("{}", signet.difficulty_target);
    println!("{}", signet.network_info);
    println!("{}", signet.verified);
    println!("block verification status : {}", signet.verify_block());
}
