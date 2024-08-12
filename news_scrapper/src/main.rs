fn main() -> Result<(), Box<dyn std::error::Error>> {

    // connect to the target page

    let response = reqwest::blocking::get("https://www.animenewsnetwork.com/news/")?;

    // extract the raw html and print it

    let html = response.text()?;

    println!("{html}");

    Ok(())

}