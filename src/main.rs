use std::{thread, time};
use clap::Parser;
use serde::{Serialize,Deserialize};
use serde_json::Value;

/// Args manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Api Key 
    #[arg(short, short)]
    api: String,

    /// City name
    #[arg(short, short,default_value = "")]
    city: String,
    
    /// Timeout
    #[arg(short, short)]
    timeout: u32, 

    /// Pollution 
	#[arg(long, long, action)]
    air: bool,
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
#[derive(Debug)]
enum Quality{
    Good,
    Fair,
    Moderate,
    Poor,
    VeryPoor
}

pub type Cities = Vec<City>;

//icone dei weather

async fn weather(cord:(f64,f64),api:String) -> Result<serde_json::Value,reqwest::Error>{

    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",cord.0,cord.1,api);
 
    let body = reqwest::get(url).await?.json::<serde_json::Value>().await?;  
 
    Ok(body)
}

async fn resolve_city(name:String,api:String) -> Result<(f64,f64),reqwest::Error>{
   
    let url = format!("https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",name,api);
   
    let body = reqwest::get(url).await?.json::<Cities>().await?;

    let temp = body.get(0).unwrap();

    Ok((temp.lat,temp.lon))
}

async fn auto() -> Result<(f64,f64),reqwest::Error>{
	//curl -sf https://location.services.mozilla.com/v1/geolocate?key=geoclue
	//{"location": {"lat": 44.0425, "lng": 12.421}, "accuracy": 20000.0}

    let url = format!("https://location.services.mozilla.com/v1/geolocate?key=geoclue");
 
    let body = reqwest::get(url).await?.json::<serde_json::Value>().await?; 
	let cord = body.get("location").unwrap().clone();

	let lat = cord.get("lat").and_then(Value::as_f64).unwrap();

	let lon = cord.get("lng").and_then(Value::as_f64).unwrap();
	
    Ok((lat,lon))
}

async fn pollution(cord:(f64,f64),api:String) -> Result<serde_json::Value,reqwest::Error>{

	let url = format!("http://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",cord.0,cord.1,api);
    let body = reqwest::get(url).await?.json::<serde_json::Value>().await?;  
 
    Ok(body)
}

fn air_quality(data:serde_json::Value) -> Quality{
	//let flag = data["list"][0]["components"]["nh3"].as_f64().clone().unwrap();
	let flag = data["list"][0]["main"]["aqi"].as_i64().clone();
	//println!("{:#?}",flag);
  	match flag{
        Some(1) => Quality::Good,
		Some(2) => Quality::Fair,
		Some(3) => Quality::Moderate,
		Some(4) => Quality::Poor,
		Some(5) => Quality::VeryPoor,
		Some(i64::MIN..=0_i64) | Some(6_i64..=i64::MAX) => todo!(),
		None => todo!(),
	}
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
 
    let cord:(f64,f64);

    if args.city.is_empty(){
        cord = auto().await.unwrap();
    }else{
        cord = resolve_city(args.city,args.api.clone()).await.unwrap();
    }

    loop{
        let weather = weather(cord,args.api.clone()).await;

        let dati = weather.unwrap().get("weather").unwrap()[0].clone();

        println!("{}",dati["main"].to_string());
        println!("{}",dati["description"].to_string());
        println!("{}",dati["icon"].to_string());

        if args.air == false{
            println!("Arguments Air Quality not setting")
        }else{
            let pollution = pollution(cord,args.api.clone()).await;

			let check = air_quality(pollution.unwrap());
            println!("{:#?}",check);
        }

        //5 secondi di pausa
        thread::sleep(time::Duration::from_secs(args.timeout.into()));
        
    }
 
}
//Api Airquality
//http://api.openweathermap.org/data/2.5/air_pollution?lat={lat}&lon={lon}&appid={API key}

/*
{
  "coord":[
    50,
    50
  ],
  "list":[
    {
      "dt":1605182400,
      "main":{
        "aqi":1
      },
      "components":{
        "co":201.94053649902344,
        "no":0.01877197064459324,
        "no2":0.7711350917816162,
        "o3":68.66455078125,
        "so2":0.6407499313354492,
        "pm2_5":0.5,
        "pm10":0.540438711643219,
        "nh3":0.12369127571582794
      }
    }
  ]
}
*/


//Api di openweathermap per ottenere la lat e lot.
//http://api.openweathermap.org/geo/1.0/direct?q={city name},{state code},{country code}&limit={limit}&appid={API key}


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



