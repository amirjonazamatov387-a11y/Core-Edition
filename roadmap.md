---

## 🟢 Phase 1: Seed Prototype (Completed)

- [x] **Graphics Pipeline:** Setup native `wgpu` rendering context and event loop using `winit`.
- [x] **Camera Controls:** 3D FPS camera with view/projection matrix math via `glam` and input handling.
- [x] **Voxel Rendering:** Basic 3D block geometry generation with vertex buffers and custom WGSL shaders.
- [x] **Chunk Mesh & Basic Culling:** Render $16 \times 16 \times 16$ voxel grid with internal face culling.
- [x] **Open Source Setup:** Repository initialized under GPLv3 license with CI/CD readiness.

---

## 🟡 Phase 2: World Engine & Rendering Pipeline (Current Focus)

Focuses on terrain generation, chunk management, and rendering optimizations to achieve high framerates at far render distances.

- [ ] **Procedural Terrain Generation:**
  - [ ] Multi-octave 2D/3D noise algorithms (`noise-rs` for Perlin/Simplex).
  - [ ] Dynamic heightmaps generating natural hills, mountains, valleys, and underground cave networks.
  - [ ] Biome generation system (temperature/humidity noise maps affecting top-soil layers and flora).
- [ ] **Chunk Streaming & World Management:**
  - [ ] $16 \times 256 \times 16$ vertical chunk column structure.
  - [ ] Asynchronous multi-threaded chunk loading/unloading around player position.
  - [ ] Frustum culling to omit rendering out-of-view chunks.
- [ ] **Greedy Meshing Algorithm:**
  - [ ] Quad-merging optimization to reduce vertex counts and draw calls by 80–90%.
  - [ ] Smooth lighting and baked ambient occlusion (AO) per vertex.
- [ ] **Texture System:**
  - [ ] $16 \times 16$ pixel texture atlas pipeline with UV mapping.
  - [ ] Dynamic directional lighting and day/night skybox rendering.

---

## 🔵 Phase 3: Core Gameplay, Blocks & Items

Introduces physics, block registries, inventory, game rules, and player interaction.

- [ ] **Physics & Collision:**
  - [ ] Axis-Aligned Bounding Box (AABB) player collision detection and response.
  - [ ] Raycast-based block targeting, breaking, and placement mechanics.
  - [ ] Gravity, jumping, sprinting, and swimming physics.
- [ ] **Data-Driven Block & Item System:**
  - [ ] Extensible registry for hundreds of block types and items (materials, tools, decorative blocks).
  - [ ] Custom block hitboxes and transparent blocks (glass, leaves, water).
  - [ ] Drops, item entities, and floating item physics.
- [ ] **Inventory & UI Systems:**
  - [ ] Figma-designed HUD overlay (hotbar, health/stamina meters, crosshair).
  - [ ] Inventory management grid, container chests, and crafting tables.
  - [ ] In-game pause menu and settings overlay.
- [ ] **Gamemodes & Difficulty Levels:**
  - [ ] **Gamemodes:** Survival (resource harvesting & health), Creative (flight & infinite blocks), Hardcore (permadeath), Spectator.
  - [ ] **Difficulty Levels:** Peaceful (no hostile spawns), Easy, Normal, Hard.
  - [ ] Customizable world rules (gamerules for mob spawns, day cycle, block drops).

---

## 🟣 Phase 4: Entities, Mobs & Dimensions

Adds living entities, artificial intelligence, and multi-world travel.

- [ ] **Entity Component System (ECS):**
  - [ ] High-performance entity architecture (using `hecs` or `bevy_ecs`) for handling thousands of concurrent entities.
- [ ] **Mob AI & Behaviors:**
  - [ ] **Passive Mobs:** Wildlife and farm animals with flocking and wandering behaviors.
  - [ ] **Hostile Mobs:** Enemy entities equipped with 3D pathfinding algorithms, line-of-sight detection, and combat AI.
  - [ ] Entity animations and status effect systems.
- [ ] **Dimensions & Alternate Worlds:**
  - [ ] Multi-world dimension manager capable of running distinct world instances simultaneously.
  - [ ] Custom generation rules for distinct dimensions (e.g., Overworld, subterranean caves, altered void realms).
  - [ ] Seamless dimension transitioning via portal structures.

---

## 🟠 Phase 5: WASM Modding Architecture & Extensibility

Empowers the community to write performant mods in any language that compiles to WebAssembly.

- [ ] **WASM Plugin Runtime:**
  - [ ] Embed high-speed WebAssembly runtime (e.g., `wasmtime` or `wasmer`) sandboxed inside the engine.
- [ ] **Extensible Modding API:**
  - [ ] Bindings for adding custom blocks, items, recipes, and biomes via mods.
  - [ ] Event hook system for custom mob behaviors, world generation passes, and custom UI panels.
- [ ] **Mod Loader & Adjustability:**
  - [ ] In-game Mod Manager to enable, disable, and configure mods on the fly.
  - [ ] Deep engine adjustability: exposed graphics settings, custom keybind remap system, exposed shader config.

---

## 🔴 Phase 6: Networking, Audio & Full Release

Turns Core Edition into a complete, playable multi-user platform.

- [ ] **Multiplayer Architecture:**
  - [ ] Dedicated client-server model over UDP/QUIC.
  - [ ] Client-side prediction and server reconciliation for smooth player movement.
- [ ] **Spatial Audio:**
  - [ ] 3D positional audio engine for block sounds, mob ambient audio, and background soundtracks.
- [ ] **World Save & Load Format:**
  - [ ] Custom efficient region file format for saving terrain modifications and player data.
- [ ] **Cross-Platform Release:**
  - [ ] Native builds for Linux, Windows, macOS, and WebAssembly (`WebGPU`).

---

## 🤝 Contribution & Feedback

This roadmap is a living document. Features may be prioritized based on community interest and contributor bandwidth. Check our [Issues](../../issues) page to join active development tasks!
