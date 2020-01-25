mod parser;
mod request_sender;

use anyhow::Result;

#[macro_use]
extern crate lazy_static;

use dotenv;



//set up api_key lazily from env
lazy_static! {
    static ref API_KEY: String = {

        dotenv::dotenv().ok();
        let api_key = dotenv::var("API_KEY").unwrap();

        api_key
    };
}



#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

//    let



//    let url = format!("https://api.nal.usda.gov/fdc/v1/search?api_key={}", *API_KEY);
//
//    let req_json = serde_json::json!({
//        "generalSearchInput": "avocado",
//        "includeDataTypeList": ["Survey (FNDDS)"],
//    });
//
//
//    let foods: Vec<Food> = reqwest::Client::new()
//        .post(url.as_str())
//        .json(&req_json)
//        .send()
//        .await?
//        .json::<FoodSearchResponse>()
//        .await?
//        .foods;
//
//    let food_details = get_food_details(foods).await?;
//
//    dbg!(food_details);





    Ok(())
}