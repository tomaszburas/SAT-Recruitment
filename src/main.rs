#![allow(non_snake_case)]
use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Serialize)]
struct ProbabilityOfUnitInjectorFailRes {
    failProbability: f32,
}

#[derive(Serialize)]
struct FuelUsageForDistanceRes {
    fuelUsage: f32,
}

#[derive(Deserialize)]
struct FuelUsageForDistanceReq {
    distance: u16,
    yearOfProduction: u16,
    fuelUsagePer100KM: u8,
}

#[derive(Deserialize)]
struct FailProbabilityReq {
    vin: String,
}

#[get("/calculate-dissel-usage-for-distance")]
async fn calculateFuelUsageForDistance(req: web::Query<FuelUsageForDistanceReq>) -> Result<impl Responder> {
    let value = (req.distance as f32)*(req.fuelUsagePer100KM as f32)/100.0;
    let res = FuelUsageForDistanceRes {
        fuelUsage: value,
    };
    return Ok(web::Json(res))
}

#[get("/probability-of-unit-injector-fail")]
async fn probabilityOfUnitInjectorFail(req: web::Query<FailProbabilityReq>) -> Result<impl Responder> {
    let value = (rand::thread_rng().gen_range(0..101) as f32)/100.0;
    let res = ProbabilityOfUnitInjectorFailRes {
        failProbability: value,
    };
    return Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculateFuelUsageForDistance)
            .service(probabilityOfUnitInjectorFail)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
