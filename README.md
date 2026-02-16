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
  <a href="#license">License</a>
</p>  
   

<p align="center">
  <img src="media/1.gif" alt="screenshot">
</p>

---
<div id="info"></div>

## 📌 Information
  
Regn is a minimal weather forecast utility for the terminal. Built using [crossterm](https://github.com/crossterm-rs/crossterm)
  
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
> **AccuWeather API**
> Regn uses the [AccuWeather](https://developer.accuweather.com/home) API to get weather information. To use this application, you must supply your own API key. Details on how to generate a key can be found on AccuWeathers developer page linked here.
> Once your key has been generated you add it to a new file in your home ($HOME) directory: `~/.regn`

  
``` terminal
Subcommands
help : print help

Flags
-l <str> : choose city location (default: Stockholm)
-t : view result directly in stdout instead of a TUI

Controls
[Esc] : quit
[Ctrl-C] : quit
```
   
---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/regn/blob/main/LICENSE).  
