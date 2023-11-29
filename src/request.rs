pub fn create_request_url(day: u16, year: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

pub fn make_request(url: &str, session_id: &str) -> anyhow::Result<String, anyhow::Error> {
    let cookie = format!("session={}", session_id);
    // let body = reqwest::blocking::get(url)?.text()?;
    // let url = url.parse::<reqwest::Url>()?;
    // let mut request = reqwest::Request::new(reqwest::Method::GET, url);
    // request.headers.insert("Cookie", cookie);

    let client = reqwest::blocking::Client::new();

    let res = client.get(url).header("Cookie", cookie).send()?;

    let bytes = res.bytes()?;
    let body = String::from_utf8(bytes.into_iter().collect())?;

    Ok(body)
}
