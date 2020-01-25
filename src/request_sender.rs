struct RequestSender {
    api_key: String,
    client: reqwest::Client,

}

impl RequestSender {
    fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        RequestSender { api_key, client }
    }

    async fn search(food_name: String) -> Vec<Food> {

    }


}


//async fn get_food_details(foods: Vec<Food>) -> Result<Vec<Value>, reqwest::Error> {
//    let client = reqwest::Client::new();
//    let mut food_datas = Vec::new();
//    for food in foods {
//        let url = format!("https://api.nal.usda.gov/fdc/v1/{}?api_key={}", food.id, *API_KEY);
//        let food_data_body = client
//            .get(&url)
//            .send()
//            .await?
//            .text()
//            .await?;
//
//
//        food_datas.push(serde_json::from_str(&food_data_body).unwrap());
//    }
//
//    Ok(food_datas)
//}