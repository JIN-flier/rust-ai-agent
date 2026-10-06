use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};

pub async fn chat_completion(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> anyhow::Result<String> {
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

    let response = client.chat().create(request).await?;

    // tracing::info!("Chat completion response: {:?}", response);

    let result = response
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| anyhow::anyhow!("No completion found"))?;

    Ok(result)
}
