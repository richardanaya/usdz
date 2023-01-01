use usdz::*;
use web::*;
mod viewer;
use viewer::*;

#[web::main]
async fn main() {
    let result = fetch(FetchOptions {
        url: "./basic.usdz",
        response_type: FetchResponseType::ArrayBuffer,
        ..Default::default()
    })
    .await;
    let buffer = if let FetchResponse::ArrayBuffer(_, ab) = result {
        ab
    } else {
        console_error("Failed to fetch");
        return;
    };
    let file = UsdzFile::parse(&buffer).unwrap();
    let usd_file_data = file.get_file_data("basic/basic.usd").unwrap();
    let usd = Usd::parse(&usd_file_data).unwrap();
    console_log(&format!("{:#?}", usd));
    view(usd).await;
}
