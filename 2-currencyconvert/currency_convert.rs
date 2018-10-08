use std::io;

// Develop a programme to convert currency X to currency Y and vice versa.

const _USD_TO_EUR: f64 = 0.8679;
const _USD_TO_CAD: f64 = 1.2936;
const _USD_TO_GBP: f64 = 0.7624;
const _USD_TO_YEN: f64 = 113.7264;
const _USD_TO_CHF: f64 = 0.9973;

fn main() {
    println!("Welcome to the CurrencyEXchange. We offer currency swapping between six currencies, as follows:\n");

    println!("US Dollar: USD");
    println!("EU Euro: EUR");
    println!("Canadian Dollar: CAD");
    println!("British Pound: GBP");
    println!("Japanese Yen: YEN");
    println!("Swiss Franc: CHF");

    println!("Enter the three digit code of the currency you want to convert from:");
    let mut from = String::new();
    io::stdin().read_line(&mut from).expect("Failed to read line.");
    
    println!("Enter the three digit code of the currency you want to convert to:");
    let mut to = String::new();
    io::stdin().read_line(&mut to).expect("Failed to read line.");
    
    from = from.to_lowercase();
    to = to.to_lowercase();

    
    println!("Enter the amount of currency you want to exchange.");
    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Failed to read line.");
    let amt: f64 =  text.trim().parse()
        .expect("N must be a number");

    println!("addr from: {} addr to: {}", &from, &to);
    println!("~~~~~ Working Doot Doot ~~~~~");
    let conv_amt = convert(amt, &from, &to);

    println!("You have converted {} {} into {} {}", amt, from, conv_amt, to);

}

fn convert(amount: f64, first_curr: &str, sec_curr: &str) -> f64 {
    if first_curr == "usd" {
        match sec_curr {
            "eur" => amount * _USD_TO_EUR,
            "cad" => amount * _USD_TO_CAD,
            "gbp" => amount * _USD_TO_GBP,
            "yen" => amount * _USD_TO_YEN,
            "chf" => amount * _USD_TO_CHF,
            _ => amount
        };
            println!("we do match usd");
    }
    if first_curr == "eur" {
        match sec_curr {
            "usd" => amount * _USD_TO_EUR.recip(),
            "cad" => amount * _USD_TO_EUR.recip() * _USD_TO_CAD,
            "gbp" => amount * _USD_TO_EUR.recip() * _USD_TO_GBP,
            "yen" => amount * _USD_TO_EUR.recip() * _USD_TO_YEN,
            "chf" => amount * _USD_TO_EUR.recip() * _USD_TO_CHF,
            _ => amount
        };
    }
    if first_curr == "cad" {
        match sec_curr {
            "usd" => amount * _USD_TO_CAD.recip(),
            "eur" => amount * _USD_TO_CAD.recip() * _USD_TO_EUR,
            "gbp" => amount * _USD_TO_CAD.recip() * _USD_TO_GBP,
            "yen" => amount * _USD_TO_CAD.recip() * _USD_TO_YEN,
            "chf" => amount * _USD_TO_CAD.recip() * _USD_TO_CHF,
            _ => amount
        };
    }
    if first_curr == "gbp" {
        match sec_curr {
            "usd" => amount * _USD_TO_GBP.recip(),
            "eur" => amount * _USD_TO_GBP.recip() * _USD_TO_EUR,
            "cad" => amount * _USD_TO_GBP.recip() * _USD_TO_CAD,
            "yen" => amount * _USD_TO_GBP.recip() * _USD_TO_YEN,
            "chf" => amount * _USD_TO_GBP.recip() * _USD_TO_CHF,
            _ => amount
        };
    }
    if first_curr == "yen" {
        match sec_curr {
            "usd" => amount * _USD_TO_YEN.recip(),
            "eur" => amount * _USD_TO_YEN.recip() * _USD_TO_EUR,
            "cad" => amount * _USD_TO_YEN.recip() * _USD_TO_CAD,
            "gbp" => amount * _USD_TO_YEN.recip() * _USD_TO_GBP,
            "chf" => amount * _USD_TO_YEN.recip() * _USD_TO_CHF,
            _ => amount
        };
    }
    if first_curr == "chf" {
        match sec_curr {
            "usd" => amount * _USD_TO_CHF.recip(),
            "eur" => amount * _USD_TO_CHF.recip() * _USD_TO_EUR,
            "cad" => amount * _USD_TO_CHF.recip() * _USD_TO_CAD,
            "gbp" => amount * _USD_TO_CHF.recip() * _USD_TO_GBP,
            "yen" => amount * _USD_TO_CHF.recip() * _USD_TO_YEN,
            _ => amount
        }
    } else {
        return amount;
    }
}
    
