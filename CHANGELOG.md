# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-09-20

- 移除 实时空气质量(beta)
- 实现 实时空气质量(new)
- 实现 空气质量小时预报(new)
- 实现  空气质量每日预报(new)

## [0.3.3] - 2024-09-18

### Fixed

- 监测站数据(new) 的返回值是新的V2版本 

## [0.3.2] - 2024-08-22

### Fixed

- 实时空气质量的返回值里的color类型改为struct

### Changed

- Reqwest default use rust-tls instead of native-tls