use serde::{Deserialize, Serialize};
use ndarray::Array1;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum Side {
    #[default]
    BUY,
    SELL,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum OptionKind {
    #[default]
    CALL,
    PUT,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OptionDataSet {
    pub options: Vec<OptionData>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OptionData {
    pub strike: f64,
    pub spot: f64,
    pub option_kind: OptionKind,
    pub side: Side,
    pub ccy2: f64,
}

/// Payoff
/// Struct for storing the vectors of x and y values
#[derive(Debug, Serialize, Deserialize, Clone, Default,PartialEq)]
pub struct Payoff{
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

fn format_currency(number: f64, scale: u8) -> String {
    format!("{:.*}", usize::from(scale), number)
}

fn calculate_axis_values(strike: f64, price: f64, premium: f64, side: Side, option_kind: OptionKind) -> f64 {
    match option_kind {
        OptionKind::CALL => {
            match side {
                Side::BUY => (price - strike).max(0.0) - premium,
                Side::SELL => premium - (price - strike).max(0.0),
            }
        },
        OptionKind::PUT => {
            match side {
                Side::BUY => (strike - price).max(0.0) - premium,
                Side::SELL => premium - (strike - price).max(0.0),
            }
        }
    }
}

fn calculate_total_payoff(option_data: &OptionData, price: f64) -> f64 {
    match option_data.side {
        Side::BUY => calculate_axis_values(option_data.strike, price, option_data.ccy2, Side::BUY, option_data.option_kind.clone()),
        Side::SELL => calculate_axis_values(option_data.strike, price, option_data.ccy2, Side::SELL, option_data.option_kind.clone()),
    }
}
/// Calculate the payoff for a given set of options
/// Arguments:
/// * `option_data_set` - A set of options
/// * `spot` - The spot price
/// * `dimension` - The number of points to calculate the payoff for
/// 
/// Returns:
/// * `Payoff` - The payoff for the given set of options
/// 
/// # Example
/// ```
/// let option1 = OptionData {
///     strike: 64584.0,
///     spot: 63153.43,
///     option_kind: OptionKind::CALL,
///     side: Side::BUY,
///     ccy2: 96.2,
///     };
///
///     let option2 = OptionData {
///     strike:63384.0,
///     spot: 63153.43,
///     option_kind: OptionKind::CALL,
///     side: Side::SELL,
///     ccy2: 439.16,
///     };
///     
///     let option3 = OptionData {
///     strike:61721.0,
///     spot: 63153.43,
///     option_kind: OptionKind::PUT,
///     side: Side::BUY,
///     ccy2: 63.69,
///     };
///
///     let option4 = OptionData {
///     strike:62921.0,
///     spot: 63153.43,
///     option_kind: OptionKind::PUT,
///     side: Side::SELL,
///     ccy2: 347.77,
///     };
///
// Creating OptionDataSet and populating it with options
///     let mut option_data_set = OptionDataSet::default();
///     option_data_set.options.push(option1);
///     option_data_set.options.push(option2);
///     option_data_set.options.push(option3);
///     option_data_set.options.push(option4);
///
///     let payoff = calculate_payoff(option_data_set, 63153.43, Option::None);
/// ```
pub fn calculate_payoff(option_data_set: OptionDataSet, spot: f64, dimension: Option<usize>) -> Payoff {
    let dim = match dimension {
        Some(d) => d,
        None => 400,
    };
    let mut payoff = Payoff::default();
    let std_dev = 0.1;
    let upper_bound = spot * (1.0 + std_dev);
    let lower_bound = spot * (1.0 - std_dev);
    let x = Array1::linspace(lower_bound, upper_bound, dim).to_vec();
    let y = x.iter().map(|&price| {
        let payoff: f64 = option_data_set.options.iter().map(|option_data| {
            calculate_total_payoff(option_data, price)
        }).sum();
        format_currency(payoff, 2).parse::<f64>().unwrap()
    }).collect();
    payoff.x = x;
    payoff.y = y;
    
    payoff
}