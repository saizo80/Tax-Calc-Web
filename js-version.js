// TODO: Write comments

function complexMath(input, rate) {
    input += input*rate;
    input = Number((input.toFixed(2)));
    return input;
}

function findTaxTarget(target, rate) {
    let input = target;
    while (complexMath(input, rate) !=target) {
        input -= 0.01;
        if (input < 0.00) {
            return -0.03;
        }
    }
    return input;
}

function jsInterface(input, rate) {
    let userInput = parseFloat(input);
    let userRate = parseFloat(rate)/100.0;
    if (!isNaN(userInput) && !isNaN(userRate)) {
        let value = findTaxTarget(userInput, userRate);
        if (value != -0.03) {
            return "$" + value.toFixed(2).toString()
        }
        else {
            return "Error, could not find target amount.";
        }
    }
    else {
        return "Error, please enter a valid number.";
    }
}
