use std::{thread, time};
use clap::Parser;

use serde::{Serialize,Deserialize};

/// Args manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Api Key 
    #[arg(short, short)]
    api: String,

    /// Api Key 
    #[arg(short, short)]
    city: String,
    
    /// Timeout
    #[arg(short, short)]
    timeout: u32, 
}
///City
#[derive(Serialize, Deserialize,Debug)]
pub struct City  {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: String,
}

pub type Cities = Vec<City>;

//icone dei weather

async fn weather(cord:(f64,f64),api:String) -> Result<serde_json::Value,reqwest::Error>{

    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",cord.0,cord.1,api);
 
    let body = reqwest::get(url).await?.json::<serde_json::Value>().await?;  
    //let body = reqwest::get(url).await?.json().await?;
 
    Ok(body)
}

async fn resolve_city(name:String,api:String) -> Result<(f64,f64),reqwest::Error>{
   
    let url = format!("https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",name,api);
   
    let body = reqwest::get(url).await?.json::<Cities>().await?;

    let temp = body.get(0).unwrap();

    Ok((temp.lat,temp.lon))
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
 
    //oppure funzione auto che prende la geoloc
    let cord = resolve_city(args.city,args.api.clone()).await.unwrap();

    loop{
        let weather = weather(cord,args.api.clone()).await;

        //let dati = weather.unwrap();

        let dati = weather.unwrap().get("weather").unwrap()[0].clone();

        println!("{}",dati["main"].to_string());
        println!("{}",dati["description"].to_string());
        println!("{}",dati["icon"].to_string());


        //5 secondi di pausa
        thread::sleep(time::Duration::from_secs(args.timeout.into()));
        
    }
 
}



//use std::collections::HashMap;
//
//let mut icons = HashMap::new();
//
//icons.insert(
//    "01d".to_string(),
//    "".to_string(),
//);


//01d) icon="";;
//01n) icon="";;
//02d) icon="";;
//02n) icon="";;
//03*) icon="";;
//04*) icon="";;
//09d) icon="";;
//09n) icon="";;
//10d) icon="";;
//10n) icon="";;
//11d) icon="";;
//11n) icon="";;
//13d) icon="";;
//13n) icon="";;
//50d) icon="";;
//50n) icon="";;
//*) icon="";

//HashMap::from([
//    ("Norway", 25),
//    ("Denmark", 24),
//    ("Iceland", 12),
//]);













