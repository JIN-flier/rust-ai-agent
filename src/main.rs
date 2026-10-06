use anyhow::Ok;
use rust_ai_agent::{
    constant::DEFAULT_MODEL,
    llm::{complete::chat_completion, structured::chat_completion_structed},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let result = chat_completion(
        DEFAULT_MODEL,
        Some("You are a helpful assistant."),
        "I wanna travel to Japan.",
    )
    .await?;
    println!("Result: {}", result);

    let plan: rust_ai_agent::models::action_plan::ActionPlan = chat_completion_structed(
        DEFAULT_MODEL,
        Some("You are a helpful assistant."),
        "I wanna travel to Japan.",
    )
    .await?;
    println!("Result: {plan:#?}");

    Ok(())
}
