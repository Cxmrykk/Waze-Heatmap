extern crate reqwest;
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Deserialize;
use serde_json::Value;
use std::collections::VecDeque;
use std::time::Duration;
use config_file::FromConfigFile;

#[derive(Deserialize)]
struct Config {
    interval: u64,
}

#[derive(Insertable)]
#[diesel(table_name = alerts)]
struct Alert {
    uuid: String,
    millis: i64,
    x: f32,
    y: f32,
}

table! {
    alerts (uuid) {
        uuid -> Text,
        millis -> BigInt,
        x -> Float,
        y -> Float,
    }
}

struct Area {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl Area {
    fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self { top, bottom, left, right }
    }

    fn split(&self) -> Vec<Self> {
        let mid_vertical = self.left + ((self.right - self.left) / 2.0);
        let mid_horizontal = self.top + ((self.bottom - self.top) / 2.0);
        vec![
            Self::new(self.top, mid_horizontal, self.left, mid_vertical),
            Self::new(self.top, mid_horizontal, mid_vertical, self.right),
            Self::new(mid_horizontal, self.bottom, self.left, mid_vertical),
            Self::new(mid_horizontal, self.bottom, mid_vertical, self.right),
        ]
    }
}

async fn get_alerts(area: &Area) -> Result<Value, reqwest::Error> {
    let url = format!("https://www.waze.com/live-map/api/georss?top={}&bottom={}&left={}&right={}&env=row&types=alerts", area.top, area.bottom, area.left, area.right);
    let resp = reqwest::get(&url).await?;
    resp.json().await
}

fn process_alerts(alerts: &[Value], conn: &mut SqliteConnection) {
    for alert in alerts {
        if alert["type"].as_str().unwrap() == "POLICE" {
            let new_alert = Alert {
                uuid: alert["uuid"].as_str().unwrap().to_string(),
                millis: alert["pubMillis"].as_i64().unwrap(),
                x: alert["location"]["x"].as_f64().unwrap() as f32,
                y: alert["location"]["y"].as_f64().unwrap() as f32,
            };
            match diesel::insert_into(alerts::table)
                .values(&new_alert)
                .execute(conn) {
                    Ok(_) => (),
                    Err(e) => println!("Error saving new alert: {:?}", e),
                }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from_config_file("Config.toml").unwrap();
    let mut interval = tokio::time::interval(Duration::from_secs(config.interval));
    
    loop {
        let initial_area = Area::new(-29.378291639798185, -35.61245928409031, 133.48388671875003, 158.79638671875003);
        let mut queue: VecDeque<Area> = VecDeque::new();
        queue.push_back(initial_area);

        let mut conn = SqliteConnection::establish("db.sqlite")
            .expect("Error connecting to the database");

        while let Some(area) = queue.pop_front() {
            match get_alerts(&area).await {
                Ok(json) => {
                    println!("Checking top={}, bottom={}, left={}, right={}:", area.top, area.bottom, area.left, area.right);

                    let alerts = json["alerts"].as_array();
                    if let Some(alerts) = alerts {
                        if alerts.len() >= 200 {
                            println!("Too many alerts. Splitting chunks...");
                            queue.extend(area.split());
                        } else {
                            println!("Found {} alerts, adding POLICE alerts to database:", alerts.len());
                            process_alerts(alerts, &mut conn);
                        }
                    }
                },
                Err(e) => println!("Error getting alerts: {:?}", e),
            }
        }

        interval.tick().await;
    }
}