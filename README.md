# Structural Payoff computation library

This is a library for computation plots in charts
This accepts data from an option and returns Payoff with vectors of x and y axis for plotting

## Computing your payoff diagram data

```bash
let option1 = OptionData {
        strike: 64584.0,
        spot: 63153.43,
        option_kind: OptionKind::CALL,
        side: Side::BUY,
        ccy2: 96.2,
    };

    let option2 = OptionData {
        strike:63384.0,
        spot: 63153.43,
        option_kind: OptionKind::CALL,
        side: Side::SELL,
        ccy2: 439.16,
    };

    let option3 = OptionData {
        strike:61721.0,
        spot: 63153.43,
        option_kind: OptionKind::PUT,
        side: Side::BUY,
        ccy2: 63.69,
    };

    let option4 = OptionData {
        strike:62921.0,
        spot: 63153.43,
        option_kind: OptionKind::PUT,
        side: Side::SELL,
        ccy2: 347.77,
    };

    // Creating OptionDataSet and populating it with options
    let mut option_data_set = OptionDataSet::default();
    option_data_set.options.push(option1);
    option_data_set.options.push(option2);
    option_data_set.options.push(option3);
    option_data_set.options.push(option4);

    let payoff = calculate_payoff(option_data_set, 63153.43, Option::None);
```
