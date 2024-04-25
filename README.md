<h2 align="center">
	<img src="https://codeabinash.github.io/media/weather-rust/images/256x256.png" width="130" alt="Logo"/><br/>
	<img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="35" width="0px"/>
	Weather CLI Rust
</h2>

<p align="center">
    <img src="https://img.shields.io/github/v/release/codeAbinash/weather-rust?colorA=363a4f&colorB=a6da95&style=for-the-badge">
    <img src="https://img.shields.io/github/languages/top/codeabinash/weather-rust?colorA=363a4f&colorB=f5a97f&style=for-the-badge">
    <img src="https://img.shields.io/github/release-date/codeabinash/weather-rust?colorA=363a4f&colorB=b7bdf8&style=for-the-badge">
    <!-- <img src="https://img.shields.io/github/license/codeAbinash/weather-rust?colorA=363a4f&colorB=ebcb8b&style=for-the-badge"> -->
    <img src="https://img.shields.io/github/license/codeAbinash/weather-rust?colorA=363a4f&colorB=a6da95&style=for-the-badge">
</p>


<p align="center">
  A simple weather CLI app written in Rust.
</p>
<p align="center">
    <img src="https://codeabinash.github.io/media/weather-rust/images/demo.gif">
</p>

## Usage

```bash
$ weather
```

> Prints the weather of the location mentioned in the .env file or the default location.

```bash
$ weather --help # or  -h prints the help message
```

```bash
$ weather --version # or -v prints the version of the app
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
