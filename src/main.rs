#[macro_use]
extern crate lazy_static;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use dotenv;


#[derive(Deserialize, Debug)]
struct FoodSearchResponse {
    foods: Vec<Food>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Food {

    #[serde(rename = "fdcId")]
    id: u32,

    description: String,

}

//set up api_key lazily from env
lazy_static! {
    static ref API_KEY: String = {
        dotenv::dotenv().ok();
        let api_key = dotenv::var("API_KEY").unwrap();

        api_key
    };
}

async fn get_food_details(foods: Vec<Food>) -> Result<Vec<Value>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut food_datas = Vec::new();
    for food in foods {
        let url = format!("https://api.nal.usda.gov/fdc/v1/{}?api_key={}", food.id, *API_KEY);
        let food_data_body = client
            .get(&url)
            .send()
            .await?
            .text()
            .await?;


        food_datas.push(serde_json::from_str(&food_data_body).unwrap());
    }

    Ok(food_datas)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {


    let url = format!("https://api.nal.usda.gov/fdc/v1/search?api_key={}", *API_KEY);

    let req_json = serde_json::json!({
        "generalSearchInput": "avocado",
        "includeDataTypeList": ["Survey (FNDDS)"],
    });


    let foods: Vec<Food> = reqwest::Client::new()
        .post(url.as_str())
        .json(&req_json)
        .send()
        .await?
        .json::<FoodSearchResponse>()
        .await?
        .foods;

    let food_details = get_food_details(foods).await?;

    dbg!(food_details);



    Ok(())
}