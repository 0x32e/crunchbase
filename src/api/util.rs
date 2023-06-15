use tabled::{builder::Builder, settings::Style};
use tabled::settings::{Color, style::BorderColor};
use crate::funding::Funding;
use num_format::{Locale, ToFormattedString};

pub fn display_table(fundings: &Vec<Funding>) {
    let mut builder = Builder::default();

    for funding in fundings {
        builder.push_record([
            funding.transaction_name.as_ref().unwrap().to_string(), 
            funding.money_raised_in_usd.map_or("N/A".to_string(), |v| v.to_formatted_string(&Locale::en)),
            funding.organization_name.as_ref().unwrap().to_string(),
            funding.organization_website.clone().map_or("N/A".to_string(), |v| v),
            funding.announced_date.clone().map_or("N/A".to_string(), |v| v),
            funding.organization_location.clone().map_or("N/A".to_string(), |i| i),
        ]);
    }

    let columns = [
        "Transaction Name",
        "Money Raised ($)",
        "Organization Name",
        "Website",
        "Announced Date",
        "Location",
    ];

    let columns = (0..builder.count_columns()).map(|i| columns[i]);
    builder.set_header(columns);

    let mut table = builder.build();
    table.with(Style::ascii_rounded());
    table.with(BorderColor::default().top(Color::FG_GREEN));
    table.with(BorderColor::default().bottom(Color::FG_GREEN));
    table.with(BorderColor::default().left(Color::FG_GREEN));
    table.with(BorderColor::default().right(Color::FG_GREEN));

    println!("{} funding(s) found:", fundings.len());
    println!("{}", table);
}

