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

// struct Company {
//     id: u64,
//     name: String,
//     description: String,
//     url: Option<String>,
//     location: Option<String>,
//     website: Option<String>,
//     industries: Option<String>, // TODO: Array of Industry struct?
// }

/*
CREATE TABLE fundings (
    id SERIAL PRIMARY KEY,
    transaction_name character varying(256) NOT NULL DEFAULT ''::character varying,
    transaction_url character varying(256) NOT NULL DEFAULT ''::character varying,
    organization_name character varying(256) NOT NULL DEFAULT ''::character varying,
    organization_url character varying(256) NOT NULL DEFAULT ''::character varying,
    funding_type character varying(256) NOT NULL DEFAULT ''::character varying,
    money_raised bigint DEFAULT 0,
    money_raised_currency character varying(256) DEFAULT ''::character varying,
    money_raised_in_usd bigint DEFAULT 0,
    announced_date character varying(256) NOT NULL DEFAULT 0,
    number_of_investors integer,
    number_of_funding_rounds integer,
    organization_description character varying(256),
    organization_industries character varying(256),
    organization_location character varying(256),
    organization_website character varying(256),
    created timestamp without time zone DEFAULT now()
);
 */