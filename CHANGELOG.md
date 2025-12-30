# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-29
### Added
- **Architectural Blueprint**: Comprehensive definition of the "Liquid Glass" engine, Cognitive Kernel, and Bottle isolation system.
- **Core Visual Engine**:
    - `liquid_glass.frag`: Vulkan fragment shader implementing Z-depth blur, Snell's law refraction, and Chronometric lighting.
- **Core Structures**:
    - `Window` struct (Rust) with physics properties (mass, density, elevation).
    - `ChronometricLight` engine (C++) for time-synced global illumination.
    - `Bottle` manager (Rust) for isolated application containers.
- **Cognitive Kernel (v0.1.0)**:
    - **Context Orb**: C++ service handling drag-and-drop intents and MIME detection.
    - **Semantic File System**: Mock vector database for natural language file retrieval ("The file I worked on last Tuesday").
- **Compatibility Layer (v0.1.0)**:
    - **Hybrid Installer**: Shell script (`compat/hybrid_installer.sh`) for automating the creation of isolated Wine bottles for legacy Windows apps.
- **Documentation**: Initial README, License, and Changelog.
