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
    !run [Progam]  
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

    msg.react(ctx, '🔃').await?;
    
    let result = execute(ctx, msg, &msg.content).await;

    msg.delete_reactions(ctx).await?;

    match result {
        Ok(res) => {msg.reply(ctx, res).await?; msg.react(ctx, '✅').await?;},
        Err(err) => {msg.reply(ctx, err).await?; msg.react(ctx, '❎').await?;},
    };

    Ok(())
}

//Brainfuck part ------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug,PartialEq)]
enum Operators {
    Inc,
    Dec,
    MovL,
    MovR,
    In,
    Out,
    LoopO,
    LoopC
}

#[derive(Debug,PartialEq)]
struct Runtime {
    prg: Vec<Operators>, 
    prg_pos: usize, // Index of where we are in execution of the program
    mem: [u8; 3000],
    mem_pos: usize, // Pointer to cell

    std_out: String, // Intermediat storage of output, will be flushed when sing In command, will move to result at the end of execution 
    result: String,
    error: Option<&'static str>
}

impl Runtime {
    fn new<'a>(raw_prg: &String) -> Runtime {
        Runtime { prg: parse(raw_prg), prg_pos: 0, mem: [0; 3000], mem_pos: 0, std_out: String::from(""), result: String::from("result: "), error: None }
    }
}

fn parse(prg: &String) -> Vec<Operators> {
    let mut result:Vec<Operators> = vec![];

    for char in prg.chars() {
        let op = match char {
            '+' => Some(Operators::Inc),
            '-' => Some(Operators::Dec),
            '<' => Some(Operators::MovL),
            '>' => Some(Operators::MovR),
            ',' => Some(Operators::In),
            '.' => Some(Operators::Out),
            '[' => Some(Operators::LoopO),
            ']' => Some(Operators::LoopC),
            _ => None,
        };

        if let Some(x) = op {
            result.push(x);
        }
    }
    result
}

async fn execute(ctx: &Context, msg: &Message, program: &String) -> Result<String, &'static str> {
    let mut runtime = Runtime::new(program);

    while let None = runtime.error {
        if runtime.prg_pos >= runtime.prg.len() {
            break;
        }

        let mem_value = runtime.mem[runtime.mem_pos];
        let current_operator = &runtime.prg[runtime.prg_pos];

        match current_operator {
            Operators::Inc => {runtime.mem[runtime.mem_pos] = mem_value.wrapping_add(1);},
            Operators::Dec => {runtime.mem[runtime.mem_pos] = mem_value.wrapping_sub(1);},
            Operators::MovL => {
                let x = runtime.mem_pos.checked_sub(1);
                if let Some(y) = x {
                    runtime.mem_pos = y;
                } else {
                    runtime.error = Some("ERROR: Head moved off tape on left side!");
                }
            },
            Operators::MovR => {
                let x = runtime.mem_pos + 1;
                if x > runtime.mem.len(){
                    runtime.mem_pos = x;
                } else {
                    runtime.error = Some("ERROR: Head moved off tape on the right side!\nHELP: Max memory size is 3000");
                }
            },
            Operators::In => todo!(),
            // {
            //     let _ = msg.reply(ctx, format!("{}\n\nenter input", runtime.std_out)).await;

            //     runtime.std_out = String::from("");
            // },
            Operators::Out => {runtime.std_out.push(mem_value as char);},
            Operators::LoopO => todo!(),
            Operators::LoopC => todo!()
        }

        runtime.prg_pos = runtime.prg_pos + 1;
    }

    if let Some(err) = runtime.error {
        return Err(err)
    }

    runtime.result.push_str(runtime.std_out.as_str());

    Ok(runtime.result)
}