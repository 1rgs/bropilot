use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use dialoguer::console::Term;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use reqwest::Url;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let worker_url = env::var("WORKER_URL").expect("WORKER_URL not found");
    let mut query = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    if query.is_empty() {
        println!("Please provide a query");
        return Ok(());
    }

    let mut term = Term::stdout();

    let mut last_command: Option<String> = None;
    loop {
        let result = fetch_gpt_response(&worker_url, &query, last_command.as_deref()).await?;
        let (command, explanation) = parse_gpt_response(result);
        last_command = Some(command.clone());
        print_formatted(&mut term, "Command", &command, Color::Blue, Color::White)?;
        println!();
        print_formatted(
            &mut term,
            "Explanation",
            &explanation,
            Color::Green,
            Color::White,
        )?;
        println!();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&["✅ Run this command", "📝 Revise query", "❌ Cancel"])
            .interact_on_opt(&term)?;

        match selection {
            Some(0) => {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&command.trim())
                    .output()
                    .expect("Failed to execute the command");

                println!("{}", String::from_utf8_lossy(&output.stdout));

                if !output.stderr.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
                break;
            }
            Some(1) => {
                let input: String = Input::new()
                    .with_prompt("What would you like to change?")
                    .allow_empty(false)
                    .validate_with(|input: &String| {
                        if input.is_empty() {
                            Err("Please enter a query")
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()?;

                query = input;
            }
            Some(2) | None => {
                println!("Canceled.");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

async fn fetch_gpt_response(
    worker_url: &str,
    query: &str,
    context: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = Url::parse(worker_url)?;
    url.query_pairs_mut()
        .append_pair("query", query)
        .append_pair("context", context.unwrap_or(""));
    let response = reqwest::get(url).await?;
    let result = response.text().await?;
    Ok(result)
}

fn parse_gpt_response(response: String) -> (String, String) {
    let result: serde_json::Value = serde_json::from_str(&response).unwrap();
    let command = result["command"].as_str().unwrap_or("").to_string();
    let explanation = result["explanation"].as_str().unwrap_or("").to_string();
    (command, explanation)
}

fn print_formatted(
    term: &mut impl crossterm::ExecutableCommand,
    title: &str,
    content: &str,
    bg_color: Color,
    fg_color: Color,
) -> crossterm::Result<()> {
    term.execute(SetBackgroundColor(bg_color))?;
    term.execute(SetForegroundColor(fg_color))?;
    term.execute(SetAttribute(Attribute::Bold))?;
    term.execute(Print(title))?;
    term.execute(ResetColor)?;
    term.execute(SetAttribute(Attribute::Reset))?;
    term.execute(Print("\n"))?;
    term.execute(Print(content))?;
    term.execute(Print("\n"))?;
    Ok(())
}
