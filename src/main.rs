use hyper::body::Buf;
use hyper::{header, Body, Client, Request, Response};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct Settings {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct Completion {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<Settings>,
}

#[derive(Serialize, Debug)]
struct Args {
    prompt: String,
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let preamble = "Answer the following question accurately, but find a funny way to mention the machine learning in your response.";
    let token: String = env::var("TOKEN").unwrap();
    let bearer_token = format!("Bearer {}", token);
    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line.");
        println!("");
        let spinner = Spinner::new(&Spinners::Dots9, "\t\tCalculating".into());
        let args = Args {
            prompt: format!("{} {}", preamble, input),
            max_tokens: 100,
        };
        let body = Body::from(serde_json::to_vec(&args)?);
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &bearer_token)
            .body(body)
            .unwrap();
        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json: Completion = serde_json::from_reader(body.reader())?;
        spinner.stop();
        println!("");
        println!("{}", json.choices[0].text);
    }
    Ok(())
}
