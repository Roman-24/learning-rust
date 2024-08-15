#[derive(Debug)]
enum CoinEuro {
    TenCent(f32),
    TwentyCent(f32),
    FiftyCent(f32),
    OneEuro(u32),
    TwoEuro(u32),
    PaperBanknote(String),
}

fn main() {
    let coin = CoinEuro::TenCent(0.10);

    println!("TenCent {:?}", coin);
}
