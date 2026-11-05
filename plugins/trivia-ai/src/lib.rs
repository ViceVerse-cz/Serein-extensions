/// VenSerein (TriviaAI): Generate trivia questions and quizzes for chat.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "new-trivia" {
        return Output::default();
    }
    Output {
        replacement: Some("❓ **Trivia Time**: What was the original name of Discord? (Type your answer!)".into()),
        ..Default::default()
    }
}

export!(handle);
