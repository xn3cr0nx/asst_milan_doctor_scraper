#![macro_use]
extern crate anyhow;
extern crate csv;

use scraper::{Html, Selector};
use reqwest::{get, header, Client};
use anyhow::{Result};
use serde::{Serialize, Deserialize};
use std::fs::{File};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Doctor {
    name: String,
    regional_code: u16,
    dob: String,
    typology: String,
    degree: String,
    enabling: String,
}

mod errors;


#[tokio::main]
async fn main() -> Result<()> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::COOKIE, header::HeaderValue::from_static("_ga=GA1.2.1754767651.1600680410; _gid=GA1.2.1603937012.1600680410; __utma=182983219.1754767651.1600680410.1600727171.1600762033.6; __utmz=182983219.1600680411.1.1.utmcsr=ats-milano.it|utmccn=(referral)|utmcmd=referral|utmcct=/portale/Cerca-medico-o-pediatra; DHWATSKEMP=2758958672.1.793815400.3350193152; __utmc=182983219; __utmb=182983219.2.10.1600762033; __utmt=1"));
    
    let client = Client::builder()
    .default_headers(headers)
    .build()?;
    
    let base_url = "https://dwh.ats-milano.it/sdg/help/".to_string();
    let params = [
        ("cmbAmbito", "308.Z09_015146"), 
        ("txtMedico", ""),
        ("__VIEWSTATE", ""),
        ("__LASTFOCUS", ""),
        ("__EVENTARGUMENT", ""),
        ("__EVENTTARGET", "cmbAmbito"),
        ("__EVENTVALIDATION", "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"),
        ("__VSTATE", "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")];
    let url = base_url.clone() + "viewmedico.aspx";
    let resp = client.post(&url)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    
    let list_document = Html::parse_document(&resp);

    let mut doctors: Vec<Doctor> = Vec::new();

    let regional_code_selector = Selector::parse(r#"span[id="rpMedico_ctl02_lblDescrizione"]"#).map_err(|_err| { errors::ScraperError::SelectorNotFound })?;
    let dob_selector = Selector::parse(r#"span[id="rpMedico_ctl03_lblDescrizione"]"#).map_err(|_err| { errors::ScraperError::SelectorNotFound })?;
    let typology_selector = Selector::parse(r#"span[id="rpMedico_ctl04_lblDescrizione"]"#).map_err(|_err| { errors::ScraperError::SelectorNotFound })?;
    let degree_selector = Selector::parse(r#"span[id="rpMedico_ctl07_lblDescrizione"]"#).map_err(|_err| { errors::ScraperError::SelectorNotFound })?;
    let enabling_selector = Selector::parse(r#"span[id="rpMedico_ctl08_lblDescrizione"]"#).map_err(|_err| { errors::ScraperError::SelectorNotFound })?;

    let selector = Selector::parse("a.v2").map_err(|_err| { errors::ScraperError::SelectorNotFound })?;
    for element in list_document.select(&selector) {
        let name = element.inner_html();
        match element.value().attr("href") {
            Some(path) => {
                if path != "#".to_string() {
                    let url = base_url.clone() + path;
                    let resp = get(&url).await?.text().await?;
                    let doctor_document = Html::parse_document(&resp);                        

                    let input = doctor_document.select(&regional_code_selector).next().ok_or_else(|| { errors::ScraperError::SelectorNotFound })?;
                    let regional_code = input.inner_html();
                    let input = doctor_document.select(&dob_selector).next().ok_or_else(|| { errors::ScraperError::SelectorNotFound })?;
                    let dob = input.inner_html();
                    let input = doctor_document.select(&typology_selector).next().ok_or_else(|| { errors::ScraperError::SelectorNotFound })?;
                    let typology = input.inner_html();
                    let input = doctor_document.select(&degree_selector).next().ok_or_else(|| { errors::ScraperError::SelectorNotFound })?;
                    let degree = input.inner_html();
                    let input = doctor_document.select(&enabling_selector).next().ok_or_else(|| { errors::ScraperError::SelectorNotFound })?;
                    let enabling = input.inner_html();

                    let doctor = Doctor{
                        name: name,
                        regional_code: regional_code.parse::<u16>()?,
                        dob: dob,
                        typology: typology,
                        degree: degree,
                        enabling: enabling
                    };
                    doctors.push(doctor);
                }
            },
            None => {}
        };
    }

    let file = File::create("result.csv")?;
    let mut wtr = csv::Writer::from_writer(file);
    for d in doctors {
        wtr.serialize(d)?;
    };
    wtr.flush()?;



    Ok(())
}