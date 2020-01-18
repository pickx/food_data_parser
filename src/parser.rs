use serde::{Serialize, Deserialize};
use serde_json::Value;


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

enum NutrientType {
    Calorie,
    Fat,
    Protein,
    Carb,
}

struct Nutrient {

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