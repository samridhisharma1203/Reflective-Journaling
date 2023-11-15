use std::io;
use std::process::Command;
use chrono::Local;

fn main() {
    // Welcome message
    println!("Welcome to your journal and reflection app!");

    // Display the options for selecting the emotional state
    println!("How was your day today? Choose an option:");
    println!("1. Happy Day");
    println!("2. Sad Day");
    println!("3. Anxious Day");
    println!("4. Peaceful Day");
    println!("5. Productive Day");
    println!("6. Hectic Day");
    println!("7. Exciting Day");
    println!("8. Boring Day");
    println!("9. Challenging Day");
    println!("10. Reflective Day");
    println!("11. Grateful Day");
    println!("12. Routine Day");
    println!("13. Surprise Day");
    println!("14. Achieving Day");
    println!("15. Relaxed Day");

    // Read user's choice for emotional state
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Invalid input");

    // Match the choice to an emotional state with a corresponding quote
    let emotion = match choice {
        1 => "Happy Day: \"Celebrate the small joys that make your day a happy one. Happiness is in the little moments.\"",
        2 => "Sad Day: \"In the midst of sadness, remember that every sunset is followed by a sunrise. Better days will come.\"",
        3 => "Anxious Day: \"Anxiety may visit, but it doesn't have to stay. Keep calm and carry on; this too shall pass.\"",
        4 => "Peaceful Day: \"Find solace in the tranquility of today. Peaceful days offer a respite from life's storms.\"",
        5 => "Productive Day: \"Today you proved that you're capable of great things. Keep up the momentum, and there's no limit to what you can achieve.\"",
        6 => "Hectic Day: \"In the chaos of life, take a moment to breathe. You're stronger than you think, even on the most hectic days.\"",
        7 => "Exciting Day: \"Embrace the thrill of today, for excitement fuels your spirit and adds color to life's canvas.\"",
        8 => "Boring Day: \"Even in the most uneventful days, find beauty in simplicity. Boredom can be the canvas for creativity.\"",
        9 => "Challenging Day: \"Challenges are the stepping stones to growth. Today's difficulties will be tomorrow's strengths.\"",
        10 => "Reflective Day: \"In moments of reflection, you discover the wisdom within yourself. Your thoughts are the keys to personal growth.\"",
        11 => "Grateful Day: \"Gratitude turns what we have into enough. Count your blessings, and even an ordinary day can be extraordinary.\"",
        12 => "Routine Day: \"In the routines of life, find comfort and stability. Consistency paves the way for success.\"",
        13 => "Surprise Day: \"Life's surprises are like hidden treasures waiting to be discovered. Embrace the unexpected with an open heart.\"",
        14 => "Achieving Day: \"Celebrate your achievements and let them inspire your future endeavors. Success is a journey, not a destination.\"",
        15 => "Relaxed Day: \"Take a deep breath and let go of stress. Relaxation rejuvenates the soul and prepares you for what's to come.\"",
        _ => {
            println!("Invalid choice. Exiting.");
            return;
        }
    };

    // Initialize the journal entry
    let mut journal_entry = String::new();

    // Define reflection questions
    let questions = [
        "What were the highlights of your day?",
        "What challenges did you face today?",
        "What did you learn today?",
        "What are you grateful for today?",
        "What could you do better tomorrow?",
        "Any other thoughts or reflections?",
    ];

    // Loop to gather answers to reflection questions
    for question in &questions {
        println!("{}", question);
        io::stdin().read_line(&mut journal_entry).expect("Failed to read line");
    }

    // Clear the screen (works on Unix-like systems)
    if cfg!(unix) {
        Command::new("clear").status().expect("Failed to clear the screen");
    }

    // Get the current date and time
    let date = Local::now();

    // Print the selected emotional state and date
    println!("{}", emotion);
    println!("Today's date: {} - {}.", date.format("%Y-%m-%d"), emotion);
    println!("Your journal entry:");

    // Print the journal entry in a box
    print_journal_entry(&journal_entry);
}

// Function to print the journal entry in a box
fn print_journal_entry(entry: &str) {
    let len = entry.lines().map(|line| line.len()).max().unwrap_or(0);

    // Print the top border of the box
    println!("┏{}┓", "━".repeat(len + 4));

    // Print each line of the journal entry with borders
    for line in entry.lines() {
        println!("┃ {} ┃", line);

    // Print the bottom border of the box
    println!("┗{}┛", "━".repeat(len + 4));
}