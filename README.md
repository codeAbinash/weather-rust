# Weather (weather-rust)

![GitHub Release](https://img.shields.io/github/v/release/codeAbinash/weather-rust)
![Release Date](https://img.shields.io/github/release-date/codeabinash/weather-rust)
![Languages](https://img.shields.io/github/languages/top/codeabinash/weather-rust)
![License](https://img.shields.io/github/license/codeAbinash/weather-rust)

A simple weather CLI app written in Rust.

## Usage

```bash
$ weather
```

> prints the weather of the location mentioned in the .env file or the default location.

```bash
$ weather --help # or  -v prints the help message
```

```bash
$ weather --version # or -h prints the version of the app
```

## Configuration

Configuration the .env file.

Weather is in development stage. A lot of features are yet to be implemented. Now the configuration is static. And can be modified in the .env file. But in future it will be dynamic and can be changed from the command line.

```js
LATITUDE = 23.2325;
LONGITUDE = 87.0654;
API_KEY = "0e376e0750966cdba160fc85a4bb0427";
LOCATION_NAME = "Bankura";
```

> If there is no .env file the program will use the default configuration which is mentioned above.

> Weather API key can be obtained from [OpenWeatherMap](https://openweathermap.org/).
