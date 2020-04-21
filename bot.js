var Discord = require('discord.js');
var winston = require('winston');
var fs = require('fs');
var diceRoller = require('./diceRolling.js');

const client = new Discord.Client();

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

client.on('ready', () => {
    logger.info('Connected');
    logger.info(`Logged in as: ${client.user.tag}`);
    client.user.setPresence({ activity: { name: 'with dice | /roll' }})
        .then(logger.info)
        .catch(er => logger.error(`Encountered error: ${JSON.stringify(er)}`));
});

client.on('message', msg => {
    try {
        if (msg.content.substring(0, 1) !== '/') return;

        var args = msg.content.substring(1).split(' ');
        var cmd = args[0];

        switch (cmd) {
            case 'hi':
                sendMessage(channelID, 'Hello, World!');
                break;
            case 'roll':
                if (msg.content.length < 7) {
                    sendMessage(
                        channelID,
                        'Can\'t roll nothing'
                    )
                    break;
                }
                if (/([+\-]?\d{0,}d\d{1,})([+\-*x/]\d{1,}){0,}/gi.test(msg.content.substring(6).replace(/\s/g, ''))) {
                    var res = (msg.content.substring(6) + ': ' + diceRoller.roll(msg.content.substring(6))).replace('*', '\\*')
                    msg.reply(res);
                } else {
                    var res =  "I don't recognize this: \""
                        + msg.content
                            .substring(6)
                            .replace(/([+\-]?\d{0,}d\d{1,})([+\-*x/]\d{1,}){0,}/gi, "")
                            .toString()
                        + "\"";
                    msg.reply(res);
                }
                break;
        }
    } catch (er) {
        msg.reply('Sorry, something went wrong.')
        logger.error(`Encountered error: ${JSON.stringify(er)}`);
    }
});

client.on('warn', warn => {
    logger.warn(`Warning: ${JSON.stringify(er)}`);
});

client.on('error', er => {
    logger.error(`Error event triggered: ${JSON.stringify(er)}`);
});

var auth = get_auth();
client.login(auth.token);
