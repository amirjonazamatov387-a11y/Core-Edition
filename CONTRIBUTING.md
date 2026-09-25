# Contributing to Core Edition

Thank you for your interest in contributing to **Core Edition**! We are building a high-performance, open-source 3D voxel engine from scratch using Rust and `wgpu`. 

Whether you're fixing a rendering bug, optimizing chunk meshing, improving documentation, or proposing new features, your help is welcome.

---

## 📜 Table of Contents
- [Code of Conduct & Project Philosophy](#-code-of-conduct--project-philosophy)
- [How to Contribute](#-how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Submitting Code](#submitting-code)
- [Development Workflow](#-development-workflow)
  - [Code Style & Formatting](#code-style--formatting)
  - [Pre-Commit Quality Checks](#pre-commit-quality-checks)
- [Git & Commit Conventions](#-git--commit-conventions)
- [Pull Request Process](#-pull-request-process)
- [License Notice](#-license-notice)

---

## 🛡 Code of Conduct & Project Philosophy

Core Edition is dedicated to providing a respectful, collaborative, and inclusive environment for everyone. 

### Core Principles
1. **Pure Scratch Architecture:** We build directly on `wgpu`, `winit`, and custom shaders. We avoid high-level game engines or heavy pre-built frameworks.
2. **Performance First:** Memory safety, zero-cost abstractions, and efficient GPU pipeline design drive all architectural decisions.
3. **No Trademark Infringement:** Do not introduce UI assets, code, or branding that infringe on existing commercial franchises or trademarks (e.g., Mojang/Microsoft assets or titles).

---

## 🛠 How to Contribute

### Reporting Bugs
Before creating a bug report, please check the [Issues](../../issues) tab to see if it has already been reported.

When opening a bug report, please include:
- **Environment details:** Operating System (Linux distro, Windows, macOS), GPU specs, graphics drivers.
- **Steps to reproduce:** Clear, step-by-step instructions.
- **Expected vs. Actual behavior.**
- **Terminal Logs:** Include output from `cargo run` or `env_logger` backtraces (`RUST_BACKTRACE=1 cargo run`).

### Suggesting Features
Feature requests are tracked in [Issues](../../issues). Describe the proposed feature, the problem it solves, and how it aligns with the engine roadmap (e.g., procedural noise, greedy meshing, physics, UI system).

---

## 💻 Development Workflow

### Prerequisites
Make sure you have the stable Rust toolchain installed:
```bash
rustup update stable
