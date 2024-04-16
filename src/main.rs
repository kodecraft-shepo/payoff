use payoff::payoff::{calculate_payoff, OptionData, OptionDataSet};

fn main(){
    let option1 = OptionData {
        strike: 55.0,
        spot: 50.0,
        option_kind: "Call".to_string(),
        side: "Buy".to_string(),
        ccy2: 3.0,
    };

    let option2 = OptionData {
        strike: 60.0,
        spot: 50.0,
        option_kind: "Put".to_string(),
        side: "Sell".to_string(),
        ccy2: 3.0,
    };

    // Creating OptionDataSet and populating it with options
    let mut option_data_set = OptionDataSet::default();
    option_data_set.options.push(option1);
    option_data_set.options.push(option2);

    let payoff = calculate_payoff(option_data_set, 50.0);

    println!("{:?}", payoff);
}