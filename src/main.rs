//imports -------------------------------------------------------------------------------------------------------------------------------------------------
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    framework::standard::{
        macros:: {command,group},
        StandardFramework,
        CommandResult
    },
    prelude::*,
};


//Const variables -----------------------------------------------------------------------------------------------------------------------------------------
const HELP_MSG: &str = "
    run [-options] [\"brainfuck program\"]    
";


//Discord bot part ----------------------------------------------------------------------------------------------------------------------------------------
#[group]
#[commands(help, run)]
struct General;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);


    let token = env::var("TOKEN").expect("token");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

//commands ------------------------------------------------------------------------------------------------------------------------------------------------
#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, HELP_MSG).await?;

    Ok(())
}

#[command]
async fn run(ctx: &Context, msg: &Message) -> CommandResult {
    let input: Vec<&str> = msg.content.split(" ").collect();

    let _options = input.clone().into_iter().find(|x| x.chars().nth(0).unwrap() == '-');

    let program = 
        String::from(input.into_iter().clone().find(|x| x.chars().nth(0).unwrap() == '\"' && x.chars().last().unwrap()  == '\"').unwrap_or("nothing"));


    msg.react(ctx, '🔃').await?;
    
    let result = run_brainfuck(ctx, msg, &program);

    msg.reply(ctx, result.unwrap()).await?;

    msg.delete_reactions(ctx).await?;

    Ok(())
}

//Brainfuck part ------------------------------------------------------------------------------------------------------------------------------------------

struct Runtime {
    prg: String,
    prg_pos: u32,
    mem: [u8; 3000],
    mem_pos: u32,

    std_out: String,
    result: String
}

impl Runtime {
    fn new<'a>(raw_prg: &String) -> Runtime {
        Runtime { prg: String::from(raw_prg.as_str()), prg_pos: 0, mem: [0; 3000], mem_pos: 0, std_out: String::from(""), result: String::from("") }
    }
}

fn run_brainfuck(_: &Context, _: &Message, prg: &String) -> Result<String, &'static str> {
    let mut runtime = Runtime::new(prg);


    runtime.result.push_str("test");
    Ok(runtime.result)
}