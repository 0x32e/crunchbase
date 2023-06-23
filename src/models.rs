use serde::Deserialize;

#[derive(Debug, Eq, Deserialize, PartialEq, Clone)]
pub struct Funding {
    #[serde(rename(deserialize = "Transaction Name"))]
    pub transaction_name: Option<String>,
    #[serde(rename(deserialize = "Transaction Name URL"))]
    pub transaction_url: Option<String>,
    #[serde(rename(deserialize = "Organization Name"))]
    pub organization_name: Option<String>,
    #[serde(rename(deserialize = "Organization Description"))]
    pub organization_description: Option<String>,
    #[serde(rename(deserialize = "Funding Type"))]
    pub funding_type: Option<String>,
    #[serde(rename(deserialize = "Money Raised"))]
    pub money_raised: Option<i64>,
    #[serde(rename(deserialize = "Money Raised Currency"))]
    pub money_raised_currency: Option<String>,
    #[serde(rename(deserialize = "Money Raised Currency (in USD)"))]
    pub money_raised_in_usd: Option<i64>,
    #[serde(rename(deserialize = "Announced Date"))]
    pub announced_date: Option<String>,
    #[serde(rename(deserialize = "Number of Investors"))]
    pub number_of_investors: Option<i32>,
    #[serde(rename(deserialize = "Number of Funding Rounds"))]
    pub number_of_funding_rounds: Option<i32>,
    #[serde(rename(deserialize = "Organization Industries"))]
    pub organization_industries: Option<String>,
    #[serde(rename(deserialize = "Organization Location"))]
    pub organization_location: Option<String>,
    #[serde(rename(deserialize = "Organization Website"))]
    pub organization_website: Option<String>,
}

impl Funding {
    pub fn columns() -> Vec<String> {
        vec![
            "Transaction Name".to_string(),
            "Money Raised ($)".to_string(),
            "Organization Name".to_string(),
            "Website".to_string(),
            "Announced Date".to_string(),
            "Location".to_string(),
            "Industries".to_string(),
            "Company Description".to_string(),
        ]
    }
}

pub struct Fundings(pub Vec<Funding>);

impl Fundings {
    pub fn to_csv_string(&self) -> String {
        let mut buffer = Vec::new();
        {
            let mut wtr = csv::Writer::from_writer(&mut buffer);
            let _ = wtr.write_record(&Funding::columns());
            for funding in &self.0 {
                let _ = wtr.write_record(&[
                    funding.transaction_name.as_ref().unwrap(),
                    &funding.money_raised_in_usd.as_ref().unwrap().to_string(),
                    funding.organization_name.as_ref().unwrap_or(&String::from("N/A")),
                    funding.organization_website.as_ref().unwrap_or(&String::from("N/A")),
                    funding.announced_date.as_ref().unwrap_or(&String::from("N/A")),
                    funding.organization_location.as_ref().unwrap_or(&String::from("N/A")),
                    funding.organization_industries.as_ref().unwrap_or(&String::from("N/A")),
                    funding.organization_description.as_ref().unwrap_or(&String::from("N/A")),
                ]);
            }
            let _ = wtr.flush();
        }
        String::from_utf8(buffer).ok().unwrap()
    }
}
