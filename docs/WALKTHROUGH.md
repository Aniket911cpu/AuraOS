# AuraOS Walkthrough & Feature Tour

Welcome to AuraOS, the operating system designed for **Ethereal Permanence**. This guide will help you navigate the unique "Liquid Glass" interface and the Cognitive Kernel.

## 1. The Liquid Glass Interface

### Windows are Physical Objects
In AuraOS, windows are not 2D planes; they are 3D slabs of virtual glass.
- **Interaction**: Click and drag a window title bar. Notice how the window *accelerates* and *decelerates* with weight (mass). It doesn't just stop instantly; it settles.
- **Refraction**: Look at the background behind a window. It is distorted based on the window's thickness and refractive index.
- **Elevation**: Active windows "float" higher (Z-axis) than inactive ones, casting softer, further shadows.

### The Lighting System
The UI appearance changes based on your real-world time.
- **Day Mode (06:00 - 18:00)**: Sharp shadows, cool sunlight, high contrast.
- **Night Mode (18:00 - 06:00)**: Soft ambient occlusion, deep violet/blue atmosphere, reduced glare.

## 2. The Cognitive Kernel

### The Context Orb
Forget the Start Menu. The **Context Orb** is your central command center, usually floating at the bottom center of the viewport.

#### How to use it:
1.  **Drag & Drop**: Drag *any* file, email, or text snippet onto the Orb.
2.  **Ask**: The Orb will analyze the content and offer context-aware actions.
    *   *Drag a PDF*: "Summarize this?", "Email to team?", "Print?"
    *   *Drag a folder*: "Sort by project?", "Backup to cloud?"
3.  **Voice/Text**: Click the Orb to type or speak natural language commands like "Find that design doc I was working on last Thursday."

### Semantic Filesystem (SFS)
Stop organizing files manually.
- Navigate to `/sfs/` in the file explorer.
- **Search by Vibe**: Create a folder named "Project Alpha Ideas". SFS will populate it with every relevant file from your drive, regardless of where it's actually stored.

## 3. Keyboard Shortcuts

- **Super + Space**: Summon Context Orb.
- **Super + Tab**: Cycle through windows (visually rotates the window stack).
- **Super + Shift + D**: Toggle "Zen Mode" (sends all windows to floor elevation).
