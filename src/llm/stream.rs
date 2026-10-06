use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_stream::stream;
use backon::{ExponentialBuilder, Retryable};
use futures::{Stream, StreamExt};

pub fn chat_stream(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
        let client = async_openai::Client::new();
        let mut messages = vec![];

        if let Some(system) = system {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system.to_string())
                    .build()?
                    .into(),
            );
        }
        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt.to_string())
                .build()?
                .into(),
        );
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            // .max_tokens(4096u32)
            .build()?;

        let mut stream = client.chat().create_stream(request).await?;

        while let Some(respons_result) = stream.next().await {
            match respons_result {
                Ok(response_chunk) => {
                    if let Some(choice) = response_chunk.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            yield Ok(content.clone());
                        }
                    }
                }
                Err(err) => {
                    yield Err(anyhow::anyhow!(err));
                }
            }
        }
    }
}

pub async fn chat_stream_with_retry(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> anyhow::Result<String> {
    let op = || async {
        let s = chat_stream(model, system, prompt);

        futures::pin_mut!(s);
        let mut output = String::new();
        while let Some(result) = s.next().await {
            match result {
                Ok(chunk) => {
                    output.push_str(&chunk);
                    print!("{}", chunk);
                }
                Err(err) => {
                    tracing::error!("Error: {}", err);
                    return Err(err);
                }
            }
        }
        Ok(output)
    };

    op.retry(ExponentialBuilder::default().with_max_times(3))
        .await
}
