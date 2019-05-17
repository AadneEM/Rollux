var Discord = require('discord.io');
var logger = require('winston');
var auth = require('./auth.json');

logger.remove(logger.transports.Console);
logger.add(new logger.transports.Console, {
    colorize: true
});
logger.level = 'debug';

var bot = new Discord.Client({
    token: auth.token,
    autorun: true
});

bot.on('ready', function (evt) {
    logger.info('Connected');
    logger.info('Logged in as: ');
    logger.info(bot.username + ' - (' + bot.id + ')');
});

bot.on('message', function (user, userID, channelID, message, evt) {
    if (message.substring(0, 1) == '/') {
        try {
            var args = message.substring(1).split(' ');
            var cmd = args[0];

            console.log(message)

            switch (cmd) {
                case 'hi':
                    bot.sendMessage({
                        to: channelID,
                        message: 'Hello, World!'
                    })
                case 'roll':
                    // dice[0] = amount, dice[1] = what dice
                    var dice = args[1].toLowerCase().split(/[d/x+\-*]/g)
                    var message = ''

                    console.log(dice.toString())

                    if (dice.some(e => parseInt(e) === 0)) {
                        message = "No! bad " + user
                    }
                    else if (dice[0] == 1 && dice.length <= 2) {
                        message = args[1] + ': ' + roll(dice[1])
                    } else {
                        var results = []
                        message = args[1] + ': ['
                        for (i = 0; i < dice[0]; i++) {
                            results[i] = roll(dice[1])
                        }
                        var multiplier = ''
                        var sum = results.reduce(getSum)
                        if (args[1].substring().includes('+')) {
                            multiplier = ' + ' + dice[2]
                            sum += parseInt(dice[2])
                        } else if (args[1].substring().includes('-')) {
                            multiplier = ' - ' + dice[2]
                            sum -= parseInt(dice[2])
                        } else if (args[1].substring().includes('/')) {
                            multiplier = ' / ' + dice[2]
                            sum /= parseInt(dice[2])
                        } else if (args[1].substring().includes('x') || args[1].substring().includes('*')) {
                            multiplier = ' x ' + dice[2]
                            sum *= parseInt(dice[2])
                        }
                        message += results.toString().replace(/,/g, ', ') + ']' + multiplier + ' = ' + sum
                    }

                    bot.sendMessage({
                        to: channelID,
                        message: message
                    })
                    break;
            }
        } catch (error) {
            console.log(error)
        }
    }
});

function roll(dice) {
    return parseInt((Math.random() * dice) + 1)
}

function getSum(total, num) {
    return total + num
}