# AuraOS: Architectural Blueprint

> **Status**: DRAFT 1.0
> **Codename**: LIQUID_GLASS

## 1. Visual Engine Architecture ("Aether")

The Visual Engine in AuraOS is not a standard compositor; it is a physical simulation environment.

### 1.1 The Liquid Glass Material
Every UI element is rendered as a 3D object with specific material properties:
- **Refraction Index ($n$)**: Defaults to 1.52 (Crown Glass).
- **Roughness**: Determines the "frost" level of the glass.
- **Thickness**: Affects the parallax shift of background elements.

### 1.2 The Infinite Z-Axis
Typical OS compositors use layers (z-index 1, 2, 3). AuraOS uses a continuous float value `elevation`.
- **Resting State (0.0)**: Windows on the "floor".
- **Hover State (1.0)**: Slight levitation.
- **Active State (5.0 - 10.0)**: High focus, significant background blur.

### 1.3 Chronometric Lighting
The lighting engine is tied to the system clock.
- **06:00 - 18:00**: Solar travel (East to West). Hard shadows, cool to warm shift.
- **18:00 - 06:00**: Lunar/Ambient mode. Soft shadows, deep blue/purple hues.

## 2. Animation & Physics System

Animations are never pre-baked (no keyframes). All motion is driven by spring physics and force vectors.

### 2.1 Spring Dynamics
$$ F = -k(x - x_{target}) - b(v) $$
- **Stiffness ($k$)**: How rigid the spring is.
- **Damping ($b$)**: How generally it settles.

### 2.2 Fluidity
Windows carry momentum. Throwing a window to the side bar is not a tween; it is a physics simulation where the window creates friction against the "air" and impacts the dock with calculated force.

## 3. The Cognitive Kernel (Userspace)

### 3.1 The Context Orb
A centralized intent resolver.
- **Input**: Text, Voice, File Drag-and-Drop.
- **Processing**: LLM-based intent application.
- **Output**: System Action (e.g., "Sort these files", "Email this to Bob").

### 3.2 Semantic Filesystem (`/sfs`)
A virtual filesystem layer relying on vector embeddings.
- **Path**: `/sfs/search/projects working on last week`
- **Returns**: Symlinks to relevant files across the physical drive.

## 4. System Services

### 4.1 Window Manager (`Orbital`)
Manages the `Window` structs, handles input routing, and steps the physics simulation.

### 4.2 Compositor (`Prism`)
Vulkan-based renderer that consumes `Window` states and renders the Liquid Glass shader passes.
