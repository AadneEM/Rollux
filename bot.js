var Discord = require('discord.io');
var logger = require('winston');
var auth = require('./auth.json');
var diceRoller = require('./diceRolling.js')

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
                    sendMessage(channelID, 'Hello, World!')
                case 'roll':
                    if (/([+\-]?\d{0,}d\d{1,})([+\-*x/]\d{1,}){0,}/gi.test(message.substring(6).replace(/\s/g, ''))) {
                        sendMessage(
                            channelID,
                            message.substring(6) + ': ' + diceRoller.roll(message.substring(6))
                        )
                    } else {
                        sendMessage(channelID, "I don't recognize this: \""
                            + message
                                .substring(6)
                                .replace(/([+\-]?\d{0,}d\d{1,})([+\-*x/]\d{1,}){0,}/gi, "")
                                .toString()
                            + "\"");
                    }
                    break;
            }
        } catch (error) {
            console.log(error)
        }
    }
});

function sendMessage(client, message) {
    if (message.length > 2500) {
        message = 'Response was to long. Sorry'
    }
    bot.sendMessage({
        to: client,
        message: message
    })
}
