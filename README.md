<h1 align="center">
    Regn
</h1>
  
<p align="center">
  <em>Weather forecast in the terminal.</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/crates/v/regn?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%regn" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <!-- <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" /> -->
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/regn/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#info">Info</a> •
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#dependencies">Dependencies</a> •
  <a href="#license">License</a>
</p>  
   

<p align="center">
  <img src="media/1.gif" alt="screenshot">
</p>

---
<div id="info"></div>

## 📌 Information
  
Regn is a minimal weather forecast utility for the terminal.  
  
---
<div id="install"></div>

## 📦 Install
    
``` bash
cargo install regn
```
   
---
<div id="usage"></div>

## 💻 Usage
    
> [!IMPORTANT]  
> **WeatherAPI**  
> Regn queries [WeatherAPI](https://www.weatherapi.com/) to fetch its weather data. To use this application, you must supply your own API key. Details on how to generate a key can be found on [WeatherAPIs developer page](https://www.weatherapi.com/docs/). Add your key to a new file in your home ($HOME) directory named ".regn": `~/.regn`  

  
``` terminal
Subcommands
help : print help

Flags
-l <str> : choose city location (default: Stockholm. Cities with spaces must be enclosed with double quotes; refer to the example down below!)
-t : view result directly in stdout instead of a TUI
-f <int> : set number of days to forecast (max: 10. default: 5. If a number is missing the default is used, if a number is larger than max the max value will be used.)

Example usage:
regn -l "rio de janeiro" -f 8

Controls
[Esc] : quit
[Ctrl-C] : quit
```
   
---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/regn/blob/main/LICENSE).  
  
---
<div id="dependencies"></div>

## 🛠 Dependencies
  
- [crossterm](https://github.com/crossterm-rs/crossterm)  
- [home](https://crates.io/crates/home/0.5.12)  
- [rand](https://github.com/rust-random/rand)  
- [serde](https://github.com/serde-rs/serde)  
- [reqwest](https://github.com/seanmonstar/reqwest)  
- [tokio](https://github.com/tokio-rs/tokio)  
