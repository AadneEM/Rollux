
module.exports = {
    roll: function (req) {
        if (typeof req !== "string") {
            return "<@138725786067664897>! You don goofed!";
        }

        // Splits request into segments. Example "3d6+3+5d2" => ["3d6", "+3", "+5d2"]
        var segments = req.replace(/\s/g, '').match(/([+\-]?\d{0,}d\d{1,})|([+\-*/x]\d{1,})/gi);

        var message = '';

        var i = -1;
        var results = [];
        var curResult = 0;
        var curMod = 1;
        segments.forEach(e => {
            if (e.match(/([+\-]?\d{0,}d\d{1,})/gi)) { // xdy
                // if not first add previous result and resetting curResult
                if (i >= 0) {
                    curResult *= curMod;
                    results[i] = curResult;

                    curResult = 0;
                }
                i++;

                // Set modifier for dice group
                curMod = (e.substring(0, 1) === '-') ? -1 : 1;

                // dice[0] = number of rolls, dice[1] = sides of dice rolled
                var dice = e.replace(/[+\-]?/, '').split(/d/i);
                var rolls = [];
                for (var l = 0; l < parseInt(dice[0] === '' ? 1 : dice[0]); l++) {
                    rolls[l] = rollDice(dice[1]);
                    curResult += rolls[l];
                }

                // add result to message
                var sign = e.match(/[+\-]/);
                message += ((sign === null) ? ' ' :  ' ' + sign) + '[' + rolls.toString() + ']';
            } else if (e.match(/([+\-*/x]\d{1,})/gi)) { // Modifier
                switch (e.substring(0, 1)) {
                    case '+':
                        curResult += parseInt(e.substring(1));
                        break;
                    case '-':
                        curResult -= parseInt(e.substring(1));
                        break;
                    case '*':
                    case 'x':
                        curResult *= parseInt(e.substring(1));
                        break;
                    case '/':
                        curResult /= parseInt(e.substring(1));
                        break;
                }
                message += e;
            }
        })
        curResult *= curMod;
        results[i] = curResult;

        
        // Adding the sum to the message
        var sum = 0;
        if (results.length > 1){
            sum = results.reduce(getSum);
        } else if (results.length === 1){
            sum = results[0];
        }
        message += ' = ' + sum;

        if (message.length > 2000)
            message = "Response was to long. Sum: " + sum;
        
        return message;
    }
}

/**
 * Roll one dice, returns result
 * @param {int} dice Amount of sides od dice rolled
 */
function rollDice(dice) {
    return parseInt((Math.random() * dice) + 1)
}

function getSum(total, num) {
    return total + num
}