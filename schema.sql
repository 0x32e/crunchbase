CREATE TABLE IF NOT EXISTS fundings (
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