extern crate influent;
extern crate hyper;
extern crate xml;
#[macro_use]
extern crate clap;

use influent::create_client;
use influent::client::{Client, Credentials};
use influent::client::Precision;
use influent::measurement::{Measurement, Value};
use hyper::{Client as HyperClient};
use std::io::Read;
use hyper::header::{Basic, Authorization, Headers};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use std::borrow::Cow;
use xml::attribute::{Attribute};
use std::{thread, time};
use std::fmt::Write;
use std::collections::HashMap;
use influent::client::http::{HttpClient};
use clap::App;

pub fn get_value(value_name: &str, attributes: &Vec<xml::attribute::OwnedAttribute>) -> String {
    let mut result : String = String::new();
    for attribute in attributes {
        if value_name == attribute.name.local_name {
            result = attribute.value.clone();
            break;
        }
    }
    result
}

pub struct SourceInfo {
    stat: String,
    node: String,
    server: String,
}

pub enum WebSphereMetric {
    CountStatistic { name : String, count : i64, source_info: SourceInfo},
    TimeStatistic { name : String, total_time: i64, source_info: SourceInfo},
    RangeStatistic { name : String, value: i64, source_info: SourceInfo},
    BoundedRangeStatistic { name : String, value: i64, source_info: SourceInfo},
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let db_username = matches.value_of("influxdb_username").unwrap();
    let db_password = matches.value_of("influxdb_password").unwrap();
    let db_name = matches.value_of("influxdb_name").unwrap();
    let ip_ = matches.value_of("websphere_address").unwrap();
    let

    let credentials = Credentials {
        username: db_username,
        password: db_password,
        database: db_name,
    };

    let hosts : Vec<String> = Vec::new();



    let client = create_client(credentials, hosts);
    let hyper_client : HyperClient = HyperClient::new();
    let sleep_time = time::Duration::from_secs(1);
    loop {
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Basic {
                    username: "user".to_string(),
                    password: Some("password".to_string())
                }
            ));
        let mut res = hyper_client.get("http://ip/wasPerfTool/servlet/perfservlet")
            .headers(headers)
            .send()
            .unwrap();
        let mut response_body : String = String::new();
        res.read_to_string(&mut response_body);
        let reader = EventReader::new(response_body.as_bytes());
        let mut node_name : String = String::new();
        let mut server_name : String = String::new();
        let mut current_stat_list : Vec<String> = Vec::new();
        let mut metrics : Vec<WebSphereMetric> = Vec::new();
        let mut stats : Vec<String> = Vec::new();
        for e in reader {
            match e {
                Ok(e) => match e {
                    XmlEvent::StartElement { name, attributes, namespace } => {
                        match name.local_name.as_str() {
                            "CountStatistic" => {
                                let mut real_stat = stats.last().unwrap().clone();
                                real_stat.truncate(64);
                                metrics.push(
                                    WebSphereMetric::CountStatistic {
                                        name: get_value("name", &attributes),
                                        count: get_value("count", &attributes).parse::<i64>().unwrap(),
                                        source_info: SourceInfo {
                                            stat: real_stat,
                                            node: node_name.clone(),
                                            server: server_name.clone(),
                                        },
                                    }
                                );
                            },
                            "TimeStatistic" => {
                                let mut real_stat = stats.last().unwrap().clone();
                                real_stat.truncate(64);
                                metrics.push(
                                    WebSphereMetric::TimeStatistic {
                                        name: get_value("name", &attributes),
                                        total_time: get_value("totalTime", &attributes).parse::<i64>().unwrap(),
                                        source_info: SourceInfo {
                                            stat: real_stat,
                                            node: node_name.clone(),
                                            server: server_name.clone(),
                                        },
                                    }
                                );
                            },
                            "RangeStatistic" => {
                                let mut real_stat = stats.last().unwrap().clone();
                                real_stat.truncate(64);
                                metrics.push(
                                    WebSphereMetric::RangeStatistic {
                                        name: get_value("name", &attributes),
                                        value: get_value("value", &attributes).parse::<i64>().unwrap(),
                                        source_info: SourceInfo {
                                            stat: real_stat,
                                            node: node_name.clone(),
                                            server: server_name.clone(),
                                        },
                                    }
                                );
                            },
                            "BoundedRangeStatistic" => {
                                let mut real_stat = stats.last().unwrap().clone();
                                real_stat.truncate(64);
                                metrics.push(
                                    WebSphereMetric::BoundedRangeStatistic {
                                        name: get_value("name", &attributes),
                                        value: get_value("value", &attributes).parse::<i64>().unwrap(),
                                        source_info: SourceInfo {
                                            stat: real_stat,
                                            node: node_name.clone(),
                                            server: server_name.clone(),
                                        },
                                    }
                                );
                            },
                            "Stat" => {
                                stats.push(get_value("name", &attributes));
                            },
                            "Node" => {
                                node_name = get_value("name", &attributes).clone();
                            },
                            "Server" => {
                                server_name = get_value("name", &attributes).clone();
                            }
                            _ => (),
                        }
                    },
                    XmlEvent::EndElement { name } => {
                        match name.local_name.as_str() {
                            "Stat" => {
                                stats.pop();
                            },
                            _ => (),
                        }
                    }
                    _ => ()
                },
                Err(e) => (),
            }
        }
        let mut measurements: Vec<Measurement> = Vec::new();
        for metric in metrics {
            match metric {
                WebSphereMetric::CountStatistic{ name, count, source_info } => {
                    let mut measurement = Measurement::new(name);
                    measurement.add_field("value", Value::Integer(count));
                    measurement.add_tag("stat".to_string(), source_info.stat);
                    measurement.add_tag("node".to_string(), source_info.node);
                    measurement.add_tag("server".to_string(), source_info.server);
                    measurement.add_tag("ip".to_string(), ip.to_string());
                    measurements.push(measurement);
                },
                WebSphereMetric::TimeStatistic{ name, total_time, source_info } => {
                    let mut measurement = Measurement::new(name);
                    measurement.add_field("value", Value::Integer(total_time));
                    measurement.add_tag("stat".to_string(), source_info.stat);
                    measurement.add_tag("node".to_string(), source_info.node);
                    measurement.add_tag("server".to_string(), source_info.server);
                    measurement.add_tag("ip".to_string(), ip.to_string());
                    measurements.push(measurement);
                },
                WebSphereMetric::RangeStatistic{ name, value, source_info } => {
                    let mut measurement = Measurement::new(name);
                    measurement.add_field("value", Value::Integer(value));
                    measurement.add_tag("stat".to_string(), source_info.stat);
                    measurement.add_tag("node".to_string(), source_info.node);
                    measurement.add_tag("server".to_string(), source_info.server);
                    measurement.add_tag("ip".to_string(), ip.to_string());
                    measurements.push(measurement);
                },
                WebSphereMetric::BoundedRangeStatistic{ name, value, source_info } => {
                    let mut measurement = Measurement::new(name);
                    measurement.add_field("value", Value::Integer(value));
                    measurement.add_tag("stat".to_string(), source_info.stat);
                    measurement.add_tag("node".to_string(), source_info.node);
                    measurement.add_tag("server".to_string(), source_info.server);
                    measurement.add_tag("ip".to_string(), ip.to_string());
                    measurements.push(measurement);
                },
            }
        }
        client.write_many(&measurements, Some(Precision::Nanoseconds));
        measurements.clear();
        thread::sleep(sleep_time);
    }
    */
}