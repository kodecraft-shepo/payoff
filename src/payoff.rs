use serde::{Deserialize, Serialize};
use ndarray::Array1;

#[derive(Debug, Serialize, Deserialize, Clone, Default,PartialEq)]
pub struct OptionDataSet {
    pub options: Vec<OptionData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct OptionData {
    pub strike: f64,
    pub spot: f64,
    pub option_kind: String,
    pub side: String,
    pub ccy2: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default,PartialEq)]
pub struct Payoff{
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

fn format_currency(number: f64, scale: u8) -> String {
    format!("{:.*}", usize::from(scale), number)
}

// Calculate call option payoff
fn call_option_formula(strike: f64, price: f64, premium: f64, direction: String) -> f64 {
    let call_option = if direction.to_uppercase() == "BUY" {
        (price - strike).max(0.0) - premium
    } else {
        premium - (price - strike).max(0.0)
    };
    call_option
}

//Calculate put option payoff
fn put_option_formula(strike: f64, price: f64, premium: f64, direction: String) -> f64 {
    let put_option = if direction.to_uppercase() == "BUY" {
        (strike - price).max(0.0) - premium
    } else {
        premium - (strike - price).max(0.0)
    };
    put_option
}

fn calculate_total_payoff(option_data: &OptionData, price: f64) -> f64 {
    match option_data.option_kind.as_str() {
        "Call" => {
            if option_data.side == "Buy" {
                call_option_formula(option_data.strike, price, option_data.ccy2, "BUY".to_string())
            } else {
                call_option_formula(option_data.strike, price, option_data.ccy2, "SELL".to_string())
            }
        }
        "Put" => {
            if option_data.side == "Buy" {
                put_option_formula(option_data.strike, price, option_data.ccy2, "BUY".to_string())
            } else {
                put_option_formula(option_data.strike, price, option_data.ccy2, "SELL".to_string())
            }
        }
        _ => 0.0,
    }
}

pub fn calculate_payoff(option_data_set: OptionDataSet, spots: f64) -> Payoff {
    let mut payoff = Payoff::default();
    let x = Array1::range(0.0, spots * 2.0, 100.0).to_vec();
    let y = x.iter().map(|&price| {
        let payoff: f64 = option_data_set.options.iter().map(|option_data| {
            format_currency(calculate_total_payoff(option_data, price), 2).parse::<f64>().unwrap()
        }).sum();
        payoff
    }).collect();

    payoff.x = x;
    payoff.y = y;
    
    payoff
}