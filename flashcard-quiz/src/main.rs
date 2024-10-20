use std::{
    fs::File,
    io::{stdin, stdout, Write as _},
    path::Path,
};

use clap::Parser as _;
use rand::seq::SliceRandom as _;

/// Flashcard CLI - Create and manage flashcards for studying.
#[derive(Debug, clap::Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    /// Add a new flashcard
    Add { question: String, answer: String },
    /// Edit an existing flashcard
    Edit {
        index: usize,
        question: Option<String>,
        answer: Option<String>,
    },
    /// Remove an existing flashcard
    Remove { index: usize },
    /// Show all flashcards
    Show,
    /// Start the quiz
    Quiz {
        #[clap(short, long, default_value = "5")]
        num_questions: usize,
    },
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Quiz {
    cards: Vec<Card>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Card {
    question: String,
    answer: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut quiz = load_or_create_quiz()?;

    match args.subcommand {
        Subcommand::Add { question, answer } => add_card(&mut quiz, &question, &answer),
        Subcommand::Edit {
            index,
            question,
            answer,
        } => edit_card(&mut quiz, index - 1, question.as_deref(), answer.as_deref())?,
        Subcommand::Remove { index } => remove_card(&mut quiz, index - 1)?,
        Subcommand::Show => show_cards(&quiz),
        Subcommand::Quiz { num_questions } => start_quiz(&quiz, num_questions)?,
    }

    save_quiz(&quiz)?;

    Ok(())
}

fn load_or_create_quiz() -> anyhow::Result<Quiz> {
    let path = Path::new("quiz.json");
    if path.exists() {
        let file = File::open(path)?;
        let quiz = serde_json::from_reader(file)?;
        Ok(quiz)
    } else {
        let quiz = Quiz::default();
        Ok(quiz)
    }
}

fn save_quiz(quiz: &Quiz) -> anyhow::Result<()> {
    let file = File::create("quiz.json")?;
    serde_json::to_writer(file, quiz)?;
    Ok(())
}

fn add_card(quiz: &mut Quiz, question: &str, answer: &str) {
    quiz.cards.push(Card {
        question: question.to_string(),
        answer: answer.to_string(),
    });

    println!(
        "Added flashcard #{} with question: '{}' and answer: '{}'",
        quiz.cards.len(),
        question,
        answer
    );
}

fn edit_card(
    quiz: &mut Quiz,
    index: usize,
    question: Option<&str>,
    answer: Option<&str>,
) -> anyhow::Result<()> {
    let card = quiz
        .cards
        .get_mut(index)
        .ok_or_else(|| anyhow::anyhow!("invalid index: {index}"))?;

    if let Some(question) = question {
        card.question = question.to_string();
    }

    if let Some(answer) = answer {
        card.answer = answer.to_string();
    }

    println!(
        "Edited flashcard #{} with question: '{}' and answer: '{}'",
        index + 1,
        card.question,
        card.answer
    );

    Ok(())
}

fn remove_card(quiz: &mut Quiz, index: usize) -> anyhow::Result<()> {
    if index >= quiz.cards.len() {
        return Err(anyhow::anyhow!("invalid index: {index}"));
    }

    let card = quiz.cards.remove(index);

    println!(
        "Removed flashcard #{} with question: '{}' and answer: '{}'",
        index + 1,
        card.question,
        card.answer
    );

    Ok(())
}

fn show_cards(quiz: &Quiz) {
    for (index, card) in quiz.cards.iter().enumerate() {
        println!(
            "Flashcard #{}: Question: '{}', Answer: '{}'",
            index + 1,
            card.question,
            card.answer
        );
    }
}

fn start_quiz(quiz: &Quiz, num_questions: usize) -> anyhow::Result<()> {
    println!("Starting quiz with {num_questions} questions");
    println!();

    let cards = quiz
        .cards
        .choose_multiple(&mut rand::thread_rng(), num_questions);

    let mut score = 0;

    for (index, card) in cards.enumerate() {
        print_question(&format!("Question #{}:\n{}", index + 1, card.question));

        let input = prompt_input("Your answer: ")?;

        if input == card.answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect! The answer is: '{}'", card.answer);
        }

        println!();
    }

    println!("Quiz complete! You scored {score}/{num_questions}");

    Ok(())
}

fn prompt_input(prompt: &str) -> anyhow::Result<String> {
    print!("{prompt}");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn print_question(question: &str) {
    let max_width = 50;
    let lines = textwrap::wrap(question, max_width);

    for line in lines {
        println!("{line:^max_width$}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_card() {
        let mut quiz = Quiz::default();
        add_card(&mut quiz, "What is the capital of France?", "Paris");
        assert_eq!(quiz.cards.len(), 1);
    }

    #[test]
    fn test_edit_card() {
        let mut quiz = Quiz {
            cards: vec![Card {
                question: "What is the capital of France?".to_string(),
                answer: "Paris".to_string(),
            }],
        };

        edit_card(&mut quiz, 0, Some("What is the capital of Italy?"), None).unwrap();
        assert_eq!(quiz.cards[0].question, "What is the capital of Italy?");
    }

    #[test]
    fn test_remove_card() {
        let mut quiz = Quiz {
            cards: vec![Card {
                question: "What is the capital of France?".to_string(),
                answer: "Paris".to_string(),
            }],
        };

        remove_card(&mut quiz, 0).unwrap();
        assert_eq!(quiz.cards.len(), 0);
    }
}
