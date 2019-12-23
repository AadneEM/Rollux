var Discord = require('discord.io');
var winston = require('winston');
var fs = require('fs');
var diceRoller = require('./diceRolling.js');

const logger = winston.createLogger({
    level: 'debug',
    format: winston.format.json(),
    defaultMeta: { service: 'user-service' },
    transports: [
        new winston.transports.Console(),
        new winston.transports.File({ filename: 'error.log', level: 'error' }),
        new winston.transports.File({ filename: 'verbose.log', level: 'verbose' }),
    ]
});

const get_auth = () => {
    if (process.env.DISCORD_TOKEN != undefined) {
	logger.info('using auth from environment variable');
	return { token: process.env.DISCORD_TOKEN };
    } else {
	logger.info('using auth from auth.json file');
	return JSON.parse(fs.readFileSync('./auth.json', 'utf8'));
    }
};

var auth = get_auth();
var bot = new Discord.Client({
    token: auth.token,
    autorun: true
});

bot.on('ready', function (evt) {
    logger.info('Connected');
    logger.info('Logged in as: ' + bot.username + ' - (' + bot.id + ')');
});

bot.on('message', function (user, userID, channelID, message, evt) {
    if (message.substring(0, 1) == '/') {
        try {
            var args = message.substring(1).split(' ');
            var cmd = args[0];

            console.log(message);

            switch (cmd) {
                case 'hi':
                    sendMessage(channelID, 'Hello, World!');
                    break;
                case 'roll':
                    if (message.length < 7) {
                        sendMessage(
                            channelID,
                            'Can\'t roll nothing'
                        )
                        break;
                    }
                    if (/([+\-]?\d{0,}d\d{1,})([+\-*x/]\d{1,}){0,}/gi.test(message.substring(6).replace(/\s/g, ''))) {
                        var res = (message.substring(6) + ': ' + diceRoller.roll(message.substring(6))).replace('*', '\\*')
                        sendMessage(
                            channelID,
                            res
                        );
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
        } catch (e) {
            error(e, channelID);
        }
    }
});

function error(error, channelID){
    var res = "Sorry, something went wrong.";

    sendMessage(channelID, res);
    logger.error(error)
}

function sendMessage(client, message) {
    if (message.length > 2000) {
        message = 'Response was to long. Sorry';
    }
    bot.sendMessage({
        to: client,
        message: message
    });
}
