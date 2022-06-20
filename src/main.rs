#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Dog to Human years convertor!"
}

#[get("/dog/<dog_years>")]
fn dog_years_to_human_years(dog_years: &str) -> String {

    match dog_years.parse::<f64>() {
        Ok(years) => {
            if years >= 0.0 {
                format!("{} in dogs year(s) is equivalent to {} in human year(s)",
                        years, 
                        years*7.0)
            } else {
                format!("Please provide a positive numeric value for the number of human year(s)")
            }
        },
        _ => format!("Please provide a positive numeric value for the number of human year(s)")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, dog_years_to_human_years])
}