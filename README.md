# SAT Recruitment

<!-- Table of Contents -->
# :notebook_with_decorative_cover: Table of Contents

- [About the Project](#star2-about-the-project)
    * [Tech Stack](#space_invader-tech-stack)
    * [Endpoints](#dart-endpoints)
    * [Run Locally](#running-run-locally)

<!-- About the Project -->
## :star2: About the Project

<!-- TechStack -->
### :space_invader: Tech Stack

- Rust

<!-- Endpoints -->
### :dart: Endpoints

1. **GET** ``/calculate-dissel-usage-for-distance?distance=...&yearOfProduction=...&fuelUsagePer100KM=...``

- **Inputs** in request url(query parameters):

  + distance - total distance between point A and point B in KM.
  Provided as a natural number.

  + yearOfProduction - year of production of the car.
  Provided as a number.
  
  + fuelUsagePer100KM - natural number that represents
  approximate fuel consumption per 100KM. Provided as
  a number.

- **Returns**:
  + fuelUsage - fuel consumotion.

![get1](https://i.imgur.com/w5j716Z.jpg "Get1")

2. **GET** ``/probability-of-unit-injector-fail?vin=...``

- **Inputs** in request url(query parameters):
    + vin - car vin number.

- **Returns**:
    + failProbability - floating-point number from 0 to 1 that indicates the probability of a failure.

![get2](https://i.imgur.com/c8Vf4SQ.jpg "Get2")

<!-- Run Locally -->
### :running: Run Locally

Clone the project

```bash
  git clone https://github.com/tomaszburas/SAT-Recruitment.git
```

Go to the project directory

```bash
  cd SAT-Recruitment
```

Install dependencies

```bash
  cargo build
```

Start the server

```bash
  cargo run
```

