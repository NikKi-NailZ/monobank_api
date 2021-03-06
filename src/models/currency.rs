use serde::{Deserialize};
use reqwest::{
    blocking::get,
    Error
};

use crate::models::date::Date;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    currency_code_a: i32,
    currency_code_b: i32,
    date: Date,
    rate_buy: Option<f32>,
    rate_sell: Option<f32>,
    rate_cross: Option<f32>
}

impl Currency {
    /// get_list static method which return vector of currencies from mono API.
    /// # Examples
    /// ```
    ///     use monobank_api::models::currency::Currency;
    /// 
    ///     let list = Currency::get_list();
    /// ```
    pub fn get_list() -> Result<Vec<Currency>, Error> {
        let url = crate::URLS.get("currency").unwrap();

        get(*url)?.json::<Vec<Currency>>()
    }
}
