#[derive(Debug)]
enum EuroShitcoin {
    TenCent(f32),
    TwentyCent(f32),
    FiftyCent(f32),
    OneEuro(u32),
    TwoEuro(u32),
    PaperBanknote(String),
}

fn paper_to_cent(euro: String) -> f32 {
    match euro.parse::<f32>() {
        Ok(value) => value,
        Err(e) => {
            println!("Failed to convert string to f32: {}", e);
            0.0
        }
    }
}

fn value_in_cents(coin: EuroShitcoin) -> f32 {
    match coin {
        EuroShitcoin::TenCent(value) => value,
        EuroShitcoin::TwentyCent(value) => value,
        EuroShitcoin::FiftyCent(value) => value,
        EuroShitcoin::OneEuro(value) => value as f32 * 100.0, // Convert Euro to cents
        EuroShitcoin::TwoEuro(value) => value as f32 * 200.0, // Convert Euro to cents
        EuroShitcoin::PaperBanknote(value) => paper_to_cent(value),
    }
}

fn test_letif() {
    let config_max = Some(24u8); // set max value to u8
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn main() {
    let coin = EuroShitcoin::TenCent(0.10);
    println!("Value in cents: {:?}", value_in_cents(coin));

    let coin = EuroShitcoin::TwentyCent(0.20);
    println!("Value in cents: {:?}", value_in_cents(coin));

    let coin = EuroShitcoin::FiftyCent(0.50);
    println!("Value in cents: {:?}", value_in_cents(coin));

    let coin = EuroShitcoin::OneEuro(1);
    println!("Value in cents: {:?}", value_in_cents(coin));

    let coin = EuroShitcoin::TwoEuro(2);
    println!("Value in cents: {:?}", value_in_cents(coin));

    let coin = EuroShitcoin::PaperBanknote("500".to_string());
    println!("Value in cents: {:?}", value_in_cents(coin));

    let coin = EuroShitcoin::PaperBanknote("wrong".to_string());
    println!("Value in cents: {:?}", value_in_cents(coin));

    test_letif();
}
