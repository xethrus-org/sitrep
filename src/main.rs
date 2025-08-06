#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use std::env;


use eframe::egui;
use serde::{Serialize, Deserialize};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SitRep interface v0",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyAppUnconverted>::default())
        }),
    )
    //grab_sat_tiff(); 
}

struct MyAppUnconverted {
    name: String,
    map: String,
    map_size: u32, //miles^2
    center_long: String,
    center_lat: String,
}

impl Default for MyAppUnconverted {
    fn default() -> Self {
        Self {
            name: "Your Name".to_owned(),
            map: "00".to_owned(),
            map_size: 000,
            center_long: "0000".to_owned(),
            center_lat: "0000".to_owned(),
        }
    }
} //TIL do not try to impl TextBuffer for u32 lol

impl eframe::App for MyAppUnconverted {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SitRep");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("map number: ");
                ui.text_edit_singleline(&mut self.map)
                    .labelled_by(name_label.id);
            });

            //move to a better use
            ui.add(egui::Slider::new(&mut self.map_size, 0..=120).text("map size"));
            if ui.button("Increment").clicked() {
                self.map_size += 1;
            }
            ui.label(format!("Hello '{}', today we will be working on map {}", self.name, self.map));
            ui.horizontal(|ui| {
                let name_label = ui.label("longitude center point: ");
                ui.text_edit_singleline(&mut self.center_long)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("latitude center point: ");
                ui.text_edit_singleline(&mut self.center_lat)
                    .labelled_by(name_label.id);
            });

            ui.label(format!("At: long '{}' and lat '{}', which a square area of '{}' sqr miles", 
                self.center_long, self.center_lat, self.map_size));

            ui.image(egui::include_image!(
                "../../assets/Sit_Rep_Logo.png"
            ));
        });
    }
}


//now do something with the data
//
struct UsableMapRequestData {
    user_name_log: String,
    user_map_number_store: u32,
    user_map_size_store_log: u32,
    user_map_long_meta: f32,
    user_map_lat_meta: f32,
}

//tries grabbing from cli
/*
async fn grab_sat_image_tif<tiff>() -> Result<tiff, reqwest::Error> {
 let mut file = std::fs::File::create("image.png").unwrap();
    reqwest::blocking::get("https://example.com/image.png")
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
}
*/
/*
use reqwest::StatusCode;
pub async fn grab_sat_tiff() -> Option<StatusCode> {
    let response = reqwest::get("https://m2m.cr.usgs.gov/api/api/json/stable/dataset-catalogs")

        .await
        .unwrap() //unwrap result
        .text() // convert to text
        .await;
        println!("{:?}", response);
    match response.expect("REASON").status() {
        reqwest::StatusCode::OK => {
            println!("success! {:?}",response);
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("fail! {:?}", response);
        },
        _ => {
            panic!("Uh oh!");
        },
    };
}
*/

#[derive(Serialize, Deserialize)]
struct LongLat {
    latitude: f32,
    longitude: f32,
}
#[derive(Deserialize, Serialize)]
struct spatialFilter {
    filterType: String,
    lowerLeft: LongLat,
    upperRight: LongLat,
}
/*
struct metadataFilter {
    usage: bool,
}
*/

//request
#[derive(Deserialize, Serialize)]
struct cloudCoverFilter {
    max: u32,
    min: u32,
    includeUnknown: bool,
}

#[derive(Deserialize, Serialize)]
struct dateRange {
    end: String,
    start: String,
}

#[derive(Deserialize, Serialize)]
struct SceneFilter {
    spatialFilter: spatialFilter,
    metadataFilter: bool,
    cloudCoverFilter: cloudCoverFilter,
    ingestFilter: dateRange
}

#[derive(Deserialize, Serialize)]
struct requestHeaders {
    maxResult: String,    
    datasetName: String,
    SceneFilter: SceneFilter,
    metadataType: String,
    sortDirection: String,
    startingNumber: u32,
}
#[derive(Deserialize, Serialize)]
struct request {
    headers: requestHeaders,
}
//
//response
#[derive(Deserialize, Serialize)]
struct Response {
    data: data,
}
#[derive(Deserialize, Serialize)]
struct data {
    results: results,
    version: String,
    errorCode: Option<String>,
    requestId: u32, 
    sessionId: u32, 
    errorMessage: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct results {
    browse: browse,
    options: options,
    entityId: String,
    metadata: metadata,
    selected: selected,
    displayId: String,
    cloudCover: bool,
    publishDate: String,
    spatialBounds: spatialBounds, 
    spatialCoverage: spatialCoverage,
    temporalCoverage: temporalCoverage,
    //totalHits: u32,
    //totalHitsAccuracy: String,
    //isCustomized: bool,
    //startingNumber: u32,
    //nextRecord: u32,
    //numExcluded: u32, perhaps useful
    //recordsReturned: u32, perhaps useful
    errorMessage: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct browse {
    browseName: Option<String>,
    browsePath: Option<String>,
    overlayPath: Option<String>,
    overlayType: Option<String>,
    thumbnailPath: Option<String>,
}
#[derive(Deserialize, Serialize)]
struct options {
    bulk: bool,
    order: bool,
    download: bool,
    secondary: bool,
}
#[derive(Deserialize, Serialize)]
struct metadata {
    value: String, //acquisition date
}
#[derive(Deserialize, Serialize)]
struct selected {
    bulk: bool,
    order: bool,
    compare: bool,
}
#[derive(Deserialize, Serialize)]
struct spatialBounds {
    r#type: String,
    coordinates: Vec<LongLat>,
}
#[derive(Deserialize, Serialize)]
struct spatialCoverage {
    r#type: String,
    coordinates: Vec<LongLat>,
}
#[derive(Deserialize, Serialize)]
struct temporalCoverage {
    endDate: String,
    startDate: String,
}



//
async fn login_request() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let api_token: std::string::String = 
        env::var("M2M_API_TOKEN").expect("M2M_API_TOKEN enviorment var is not set!!");
    let username: std::string::String =
        env::var("M2M_USERNAME").expect("M2M_USERNAME enviorment var is not set!!");
    let client = reqwest::Client::new(); 
    let login_res = client
        .get("https://m2m.cr.usgs.gov/api/api/json/stable/login-token")
        .header("username", username)
        .header("token", api_token)
        .send()
        .await?;
    Ok(client)
    // need something like: if login_res.errorCode == null
}
async fn make_scene_request(client: reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    //let client = reqwest::Client::new();
    let res = client
        .get("http://m2m.cr.usgs.gov/api/api/json/stable/scene-search");
    let body = res.json::<Response>(Response).await?;
    println!("body = {:?}", body.data.results.errorMessage);
    Ok(())
  /*
    let response = client
        .get("https://m2m.cr.usgs.gov/api/api/json/stable/scene-search")
        .unwrap()
        .json::<Response>()
        .unwrap();
    println!("{:?}", response);

    let response = client.get("https://m2m.cr.usgs.gov/api/api/json/stable/scene-search");
    if let Err(e) = response.request {
        if e.is_redirect() {
            if let Some(final_stop) = e.url() {
                println!("oh fuck dude");
            }
        }
    }
    */
}

async fn grab_sat_tiff() {
    let client = reqwest::Client::new();
    let response = reqwest::get("https://m2m.cr.usgs.gov/api/api/json/stable/dataset-catalogs")
        .await
        .unwrap() //unwrap result
        .text() // convert to text
        .await;
    println!("{:?}",response);
}




//https://m2m.cr.usgs.gov/api/api/json/stable/dataset-catalogs
//the stable/ appending data is where we can dictate the EE or otherwise

