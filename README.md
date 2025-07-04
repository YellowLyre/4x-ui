[English](/README.md) | [فارسی](/README.fa_IR.md) | [العربية](/README.ar_EG.md) |  [中文](/README.zh_CN.md) | [Español](/README.es_ES.md) | [Русский](/README.ru_RU.md)

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./media/4x-ui-dark.png">
    <img alt="4x-ui" src="./media/4x-ui-light.png">
  </picture>
</p>

[![](https://img.shields.io/github/v/release/mhsanaei/4x-ui.svg?style=for-the-badge)](https://github.com/MHSanaei/4x-ui/releases)
[![](https://img.shields.io/github/actions/workflow/status/mhsanaei/4x-ui/release.yml.svg?style=for-the-badge)](https://github.com/MHSanaei/4x-ui/actions)
[![GO Version](https://img.shields.io/github/go-mod/go-version/mhsanaei/4x-ui.svg?style=for-the-badge)](#)
[![Downloads](https://img.shields.io/github/downloads/mhsanaei/4x-ui/total.svg?style=for-the-badge)](https://github.com/MHSanaei/4x-ui/releases/latest)
[![License](https://img.shields.io/badge/license-GPL%20V3-blue.svg?longCache=true&style=for-the-badge)](https://www.gnu.org/licenses/gpl-3.0.en.html)

# 4x-ui

> 基于3x-ui深度重构，支持区块链钱包登录（TON/SUI/OKX），Rust微服务高安全，现代UI，适合自建和二次开发。

## 🚀 项目亮点
- **区块链钱包登录**：支持TON、SUI、OKX三大钱包体系
- **Rust微服务**：高性能、零信任认证、JWT令牌
- **Go主服务**：继承3x-ui全部代理管控能力
- **现代化UI**：响应式前端，极致体验
- **一键安装**：官方install.sh脚本，支持主流Linux

## 🏗️ 系统架构
- Rust微服务（blockchain-auth）：负责区块链登录与验签
- Go主服务（4x-ui）：负责面板、API、策略下发
- 前端（web/html）：钱包登录、管理后台

## 🛠️ 快速安装
```bash
curl -fsSL https://raw.githubusercontent.com/YellowLyre/4x-ui/main/install.sh | bash
```

## 🌐 访问地址
- 主面板：http://<你的服务器IP>:54321
- 区块链登录：http://<你的服务器IP>:54321/blockchain-login

## 📦 项目地址
- GitHub: https://github.com/YellowLyre/4x-ui

## 📄 许可证
MIT

---

> 本项目为3x-ui的深度定制升级，适合自建、二开、Web3场景。欢迎Star、Fork、PR！

## Quick Start

```bash
bash <(curl -Ls https://raw.githubusercontent.com/mhsanaei/4x-ui/master/install.sh)
```

For full documentation, please visit the [project Wiki](https://github.com/MHSanaei/4x-ui/wiki).

## A Special Thanks to

- [alireza0](https://github.com/alireza0/)

## Acknowledgment

- [Iran v2ray rules](https://github.com/chocolate4u/Iran-v2ray-rules) (License: **GPL-3.0**): _Enhanced v2ray/xray and v2ray/xray-clients routing rules with built-in Iranian domains and a focus on security and adblocking._
- [Russia v2ray rules](https://github.com/runetfreedom/russia-v2ray-rules-dat) (License: **GPL-3.0**): _This repository contains automatically updated V2Ray routing rules based on data on blocked domains and addresses in Russia._

## Support project

**If this project is helpful to you, you may wish to give it a**:star2:

<p align="left">
  <a href="https://buymeacoffee.com/mhsanaei" target="_blank">
    <img src="./media/buymeacoffe.png" alt="Image">
  </a>
</p>

- USDT (TON): `UQDPcOTpjf63Md3VqqCB_KmAh3kmjOAIaiUDrYlg6ayLehoX`

## Stargazers over Time

[![Stargazers over time](https://starchart.cc/MHSanaei/4x-ui.svg?variant=adaptive)](https://starchart.cc/MHSanaei/4x-ui)
