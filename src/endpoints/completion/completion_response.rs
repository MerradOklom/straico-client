use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents a collection of completion data with associated pricing and word count statistics.
///
/// This struct aggregates completion results along with total price and word count information.
///
/// # Fields
/// * `completions` - A mapping of completion identifiers to their associated model data
/// * `overall_price` - The total price breakdown for all completions
/// * `overall_words` - The total word count statistics for all completions
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionData {
    /// A map of completion identifiers to their associated model data containing
    /// completion responses, pricing and word count information
    completions: HashMap<Box<str>, Model>,
    /// Price breakdown showing input, output and total costs across all completions
    overall_price: Price,
    /// Word count statistics showing input, output and total counts across all completions
    overall_words: Words,
}

/// Represents the pricing breakdown for model usage.
///
/// This struct tracks the costs associated with both input and output tokens,
/// as well as the total combined price.
///
/// # Fields
/// * `input` - The cost for input/prompt tokens
/// * `output` - The cost for output/completion tokens
/// * `total` - The total combined cost of input and output
#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    /// Cost for input/prompt tokens
    input: f32,
    /// Cost for output/completion tokens
    output: f32,
    /// Total combined cost of input and output
    total: f32,
}

/// Represents word count statistics for text processing.
///
/// This struct tracks the number of words in input and output text,
/// as well as maintaining a total word count.
///
/// # Fields
/// * `input` - The number of words in the input/prompt text
/// * `output` - The number of words in the output/completion text
/// * `total` - The total combined word count of input and output
#[derive(Serialize, Deserialize, Debug)]
pub struct Words {
    /// Number of words in the input/prompt text
    input: u32,
    /// Number of words in the output/completion text
    output: u32,
    /// Total combined word count from input and output
    total: u32,
}

/// Represents a model's completion data along with associated pricing and word count metrics.
///
/// This struct combines the completion response with pricing and word count statistics
/// for a specific model interaction.
///
/// # Fields
/// * `completion` - The completion response containing choices, usage stats and metadata
/// * `price` - The price breakdown for this model completion
/// * `words` - Word count statistics for the input/output text
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    /// The completion response containing choices, usage stats and metadata
    completion: Completion,
    /// Price breakdown showing input, output and total costs for this model completion
    price: Price,
    /// Word count statistics showing input, output and total counts for this model completion
    words: Words,
}

/// Represents a completion response from a language model.
///
/// This struct contains the generated outputs and metadata for a completion request,
/// including multiple choices/responses, model information, and usage statistics.
///
/// # Fields
/// * `choices` - A vector of generated responses/completions
/// * `object` - The type of object (e.g. "chat.completion")
/// * `id` - Unique identifier for this completion
/// * `model` - Name/identifier of the model used
/// * `created` - Unix timestamp of when this completion was created
/// * `usage` - Token usage statistics for this completion
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Completion {
    /// Vector of generated response choices from the model
    pub choices: Vec<Choice>,
    /// The type/category of response object (e.g. "chat.completion")
    pub object: Box<str>,
    /// Unique identifier for this completion
    pub id: Box<str>,
    /// Name/identifier of the model used for generation
    pub model: Box<str>,
    /// Unix timestamp of when this completion was created
    pub created: u64,
    /// Token usage statistics for this completion
    pub usage: Usage,
}

/// Represents token usage statistics for a language model completion.
///
/// This struct tracks the number of tokens used in the prompt, completion, and the total
/// tokens consumed during the model interaction.
///
/// # Fields
/// * `prompt_tokens` - Number of tokens in the input/prompt text
/// * `completion_tokens` - Number of tokens in the generated completion/output
/// * `total_tokens` - Total combined token count (prompt + completion)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    /// Number of tokens in the input/prompt text
    prompt_tokens: u32,
    /// Number of tokens in the generated completion/output
    completion_tokens: u32,
    /// Total combined token count (prompt + completion)
    total_tokens: u32,
}

/// Represents a single generated choice/response from a language model completion.
///
/// This struct contains details about a specific completion response, including the
/// message content, its position in the list of choices, and why the completion stopped.
///
/// # Fields
/// * `message` - The actual response content and metadata
/// * `index` - Zero-based position of this choice in the list of responses
/// * `finish_reason` - Why the model stopped generating (e.g. "stop", "length", "tool_calls")
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    /// The message content and metadata for this choice
    pub message: Message,
    /// Zero-based position of this choice in the list of responses
    pub index: u8,
    /// Reason why the model stopped generating (e.g. "stop", "length", "tool_calls")
    pub finish_reason: Box<str>,
}

/// Represents different types of messages in a conversation.
///
/// This enum is used to differentiate between messages from different roles in a chat or
/// conversation context. It supports serialization/deserialization with serde and uses
/// the "role" field as a tag with lowercase values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// A message from a user, containing text content
    User { content: Box<str> },
    /// A message from the AI assistant, which may contain text content and/or tool calls
    Assistant {
        content: Option<Box<str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    /// A system message providing context or instructions
    System { content: Box<str> },
    /// A message from a tool containing output or results
    Tool { content: Box<str> },
}

/// Represents a call to a function-based tool in the conversation.
///
/// This enum is used to specify function calls that can be made by the assistant. It uses
/// serde serialization with a "type" tag that is lowercase. Currently only supports function
/// calls with an ID and associated function data.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCall {
    /// A function call with a unique identifier and function parameters
    Function { id: String, function: FunctionData },
}

/// Represents the data required to make a function call.
///
/// This struct contains the function name and any arguments needed to execute the function.
/// It is used within `ToolCall` to specify function call details.
///
/// # Fields
/// * `name` - The name of the function to be called
/// * `arguments` - The function arguments as a dynamic JSON Value
#[derive(Deserialize, Clone, Debug)]
pub struct FunctionData {
    /// The name of the function to call
    name: String,
    /// The arguments to pass to the function as a JSON Value
    arguments: Value,
}

// Custom serializer to convert Value to String
impl Serialize for FunctionData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FunctionData", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("arguments", &self.arguments.to_string())?;
        state.end()
    }
}

impl CompletionData {
    /// Extracts and returns the first completion from the `completions` HashMap.
    ///
    /// # Returns
    /// The `Completion` object from the first entry in the completions map.
    pub fn get_completion(self) -> Completion {
        let values = self.completions.into_values();
        values.map(|x| x.completion).next().unwrap()
    }
}

impl Completion {
    /// Parses and processes the completion data, updating finish reasons and tool calls.
    ///
    /// This function performs two main operations on the completion data:
    /// 1. Processes any tool calls in the messages using `into_tool_calls_response()`
    /// 2. Updates finish reasons based on content and existing finish reason values:
    ///    - Sets to "tool_calls" if content is None
    ///    - Changes "end_turn" to "stop"
    ///
    /// # Returns
    /// Returns the processed completion wrapped in a Result
    pub fn parse(mut self) -> Result<Completion> {
        for x in self.choices.iter_mut() {
            x.message.into_tool_calls_response()?;
            if let Message::Assistant { content, .. } = &x.message {
                if content.is_none() {
                    x.finish_reason = "tool_calls".into();
                } else if x.finish_reason == "end_turn".into() {
                    x.finish_reason = "stop".into();
                }
            }
        }
        Ok(self)
    }
}

impl Message {
    /// Converts tool call markup in message content into structured tool calls.
    ///
    /// This function processes the content of an Assistant message to extract tool calls
    /// that are marked up with XML-style tags (<tool_call>...</tool_call>). When found,
    /// it:
    /// - Extracts the JSON content from within the tool call tags
    /// - Parses it into FunctionData structs
    /// - Creates ToolCall::Function instances from the parsed data
    /// - Stores the tool calls in the message's tool_calls field
    /// - Removes the original content containing the markup
    ///
    /// # Returns
    /// - `Ok(())` if processing succeeds or if no tool calls are found
    /// - `Err` if JSON parsing fails
    fn into_tool_calls_response(&mut self) -> Result<()> {
        if let Message::Assistant {
            content,
            tool_calls,
        } = self
        {
            if let Some(optional_content) = content {
                if optional_content.find("<tool_call>").is_some()
                    || optional_content.find("</tool_call>").is_some()
                {
                    let re = regex::Regex::new(r"<tool_call>(.*?)</tool_call>").unwrap();
                    let items = re
                        .captures_iter(&optional_content.replace("\n", ""))
                        .map(|cap| cap.get(1).unwrap().as_str().trim())
                        .map(|s| {
                            serde_json::from_str::<FunctionData>(s).map(|function_data| {
                                ToolCall::Function {
                                    id: String::from("func"),
                                    function: function_data,
                                }
                            })
                        })
                        .collect::<Result<Vec<ToolCall>, _>>()?;

                    let _ = tool_calls.insert(items);
                    content.take();
                }
            }
        }
        Ok(())
    }
    pub fn new_assistant_content(content: String) -> Self {
        Message::Assistant {
            content: Some(content.into()),
            tool_calls: None,
        }
    }

    pub fn new_assistant_tool_calls(tool_calls: Vec<ToolCall>) -> Self {
        Message::Assistant {
            content: None,
            tool_calls: Some(tool_calls),
        }
    }
}
