use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, 
    ChatCompletionRequestUserMessageArgs, 
    CreateChatCompletionRequestArgs, 
    ResponseFormat, 
    ResponseFormatJsonSchema
};
use crate::models::action_plan::ActionPlan;
use serde_json;

pub async fn chat_completion_structed(model: &str, system: Option<&str>, prompt: &str) -> anyhow::Result<ActionPlan> {
    let client = async_openai::Client::new();
    let mut messages = vec![];

    if let Some(system) = system {
        messages.push(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system.to_string())
                .build()?
                .into()
        );
    }
    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
        .content(prompt.to_string())
        .build()?
        .into()
    );

    let schema = schemars::schema_for!(ActionPlan);
    let schema_json = schema.as_value().clone();
    let format_setting = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: Some("A step-by-step action plan with difficulty levels and estimated time.".into()),
            name: "action_plan".into(),
            schema: schema_json,
            strict: Some(true),
        }
    };

    
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .response_format(format_setting)
        // .max_tokens(4096u32)
        .build()?;

    let response = client.chat().create(request).await?;

    let plan: ActionPlan = response
        .choices
        .into_iter()
        .next()
        .and_then(|c|c.message.content)
        .ok_or_else(||anyhow::anyhow!("No completion found"))
        .and_then(|s|serde_json::from_str(&s).map_err(|e|anyhow::anyhow!("Failed to parse JSON: {}", e)))?;

    Ok(plan)
}