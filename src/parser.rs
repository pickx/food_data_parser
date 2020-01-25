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

