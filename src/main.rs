use std::{ 
    fs,
    process::Command, 
};

static FILENAME: &str = "lTKsjZN-Aec.csv";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match text_llm_improve(FILENAME) {
        Ok(_) => println!("Text LLM enhanced"),
        Err(e) => {
            eprintln!("Enhancing text failed");
            return Err(e)?;
        }
    }

    Ok(())
}


fn text_llm_improve(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut citation = String::from("");
    {
        citation = fs::read_to_string(format!("{}", &filename)).expect("Failed to read csv file {filename}.csv");
    }
    
    let mut prompt = format!("Format the following text with proper punctuation, paragraph breaks, and grammar. Do not change words order. Return the final corrected version without asking for feedback or follow-up questions. Here is the text: {citation}.");
    prompt = escape_basic(&prompt);
    println!("PROMPT LENGTH: {}", &prompt.len());
    println!("PROMPT: {}", &prompt);

    let output = Command::new("wasmedge")
        .args([
            "--dir", ".:.",
            "--nn-preload", "default:GGML:AUTO:models/llama-2-7b-chat.Q5_K_M.gguf",
            "llama-simple.wasm",
            "--prompt", &prompt,
            "--ctx-size", "4096",
            "--log-enable"
        ])
        .output()
        .expect("Failed to execute command");

    match output.status.success() {
        true => {
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        },
        false => {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
    }

    Ok(())
}


// Escape special symbols for prompt
fn escape_basic(text: &str) -> String {
    text
        .replace("[Music]", "")
        .replace("[Applause]", "")
        .replace('"', "")
        .replace('\n', "")
        .trim()
        .to_string()
}