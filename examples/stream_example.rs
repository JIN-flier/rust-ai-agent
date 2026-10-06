use rust_ai_agent::{
    constant::DEFAULT_MODEL,
    llm::{semaphore::get_semaphore, stream::chat_stream_with_retry},
};
use tokio::task::JoinSet;
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // let stream = chat_stream(
    //     DEFAULT_MODEL,
    //     Some("You are a helpful assistant."),
    //     "I wanna travel to Japan.",
    // );
    // futures::pin_mut!(stream);
    // let mut output = String::new();
    // while let Some(result) = stream.next().await {
    //     match result {
    //         Ok(chunk) => {
    //             output.push_str(&chunk);
    //             print!("{}", chunk);
    //         }
    //         Err(err) => {
    //             eprintln!("Error: {}", err);
    //             break;
    //         }
    //     }
    // }
    // println!("\n==========================");
    // println!("\nFinal output: {}", output);

    let prompts = vec![
        "I wanna travel to Japan.",
        "I wanna travel to Korea.",
        "I wanna travel to China.",
    ];

    let mut set = JoinSet::new();
    for prompt in prompts {
        let span = tracing::info_span!("chat", prompt = prompt);
        set.spawn(
            async move {
                tracing::info!("\n\n{prompt}");
                let permit = get_semaphore().acquire().await?;
                let output = chat_stream_with_retry(
                    DEFAULT_MODEL,
                    Some("You are a helpful assistant."),
                    prompt,
                )
                .await?;
                drop(permit);
                Ok::<_, anyhow::Error>((prompt, output))
            }
            .instrument(span),
        );
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok((prompt, output))) => {
                tracing::info!("\n\n{prompt}: {output}");
            }
            Ok(Err(err)) => {
                tracing::error!("Error: {}", err);
            }
            Err(err) => {
                tracing::error!("Task join error: {}", err);
            }
        }
    }

    Ok(())
}
