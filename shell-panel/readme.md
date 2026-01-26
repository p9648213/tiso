# 🗺️ Roadmap: Wayland Top Panel

### 🛠 Phase 1: The Skeleton
*Goal: Get a static bar rendered at the top of the screen.*

- [x] **Project Setup**: Initialize Rust project with `iced`.
- [x] **Layer Shell Support**: Implement `iced_layershell` (or similar) to anchor the window as a panel (exclusive zone) rather than a floating window.
- [x] **Basic Container Layout**: Create the standard 3-section flexible layout:
    - [x] **Start**: App Launcher.
    - [x] **Center**: Clock/Date.
    - [x] **End**: System status (Volume, Battery, Network).

### ⚡ Phase 2: Actions & Hardware Stats
*Goal: Add interaction and performance monitoring.*

- [ ] **Launcher Button**:
    - [ ] Add a "Menu" or "Logo" icon on the far left.
    - [ ] On click: Spawn an external process to launch apps.
- [ ] **Power Controls**:
    - [ ] Add a power icon button on the far right.
    - [ ] On click: Execute `niri msg action quit` (logout) or `systemctl poweroff`.
- [ ] **Resource Monitor**:
    - [ ] **CPU Widget**: Display current CPU usage %.
    - [ ] **RAM Widget**: Display used memory.
    - *Tip: Use the `sysinfo` crate for this.*

### 🔋 Phase 3: System Widgets
*Goal: Display essential system information.*

- [x] **Clock Widget**:
    - [x] Show date time.
    - [x] Auto-refresh every second/minute.
- [ ] **Audio Widget** (PulseAudio/PipeWire):
    - [ ] Show current volume percentage.
    - [ ] Muted/Unmuted icon toggle.
- [ ] **Battery Widget** (via `upower` or reading `/sys/class/power_supply`):
    - [ ] Show percentage.
    - [ ] Change icon based on charging status.

---
