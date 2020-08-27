mod lib;
use rand::thread_rng;
use serenity::{
    client::{Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::{prelude::Ready, channel::Message},
    Client,
};

#[group]
#[commands(hi, roll)]
struct General;

#[command]
fn hi(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello world!")?;

    Ok(())
}

#[command]
fn roll(ctx: &mut Context, msg: &Message) -> CommandResult {
    let content = msg.content_safe(&ctx.cache);
    let mut rng = thread_rng();

    let roll_result = lib::roll_dice(&content, &mut rng)?;

    let result : String = roll_result.rolls.iter().map(|i| {
        let operator = &i.0.operator;
        let rolls = &i.0.results;
        let modifiers : String = i.1.iter().map(|seg| {
            match seg {
                lib::Segment::Modifier {op, amount} => format!("{}{}", op, amount),
                _ => unreachable!()
            }
        })
        .collect::<Vec<_>>().join("");
        
        format!("{}{:?} {}", operator, rolls, modifiers)
        
    }).collect::<Vec<_>>().join(" ");

    let total = roll_result.total;

    let result = format!("{}= {}", result, total);
    
    if let Err(e)  = msg.reply(ctx, result) {
        eprintln!("Failed to respond to command: {:?}", e)
    }
    
    Ok(())
}

struct Handler;

impl EventHandler for Handler {

    fn ready(&self, ctx: Context, _: Ready) {
        use serenity::model::{gateway::Activity, user::OnlineStatus};

	ctx.set_presence(Some(Activity::playing("with dice | /roll")), OnlineStatus::Online);
    }
    
}

fn main() {
    let discord_token = std::env::var("DISCORD_TOKEN").expect("failed to read DISCORD_TOKEN environment variable");  

    let mut client = Client::new(&discord_token, Handler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("/"))
            .group(&GENERAL_GROUP),
    );

    if let Err(e) = client.start() {
        eprintln!("Error occured starting discord client: {:?}", e);
    }
}
