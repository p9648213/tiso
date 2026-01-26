# 📂 File Manager - Roadmap

### 🛠 Phase 1: The Foundation
*Goal: Display the contents of a directory on the screen.*

- [x] **Project Setup**: Initialize `cargo` and add `iced` dependencies.
- [ ] **State Management**: Create a struct to hold the `current_path` and a `Vec<FileEntry>`.
- [ ] **File Listing**: Implement `std::fs::read_dir` to fetch contents of the current path.
- [ ] **Basic UI**: Render a simple `Scrollable` list or `Column` displaying file names.
- [ ] **Icons**: Add specific icons for Folders vs. Files.
- [ ] **Error Handling**: Display a toast or text message if a directory cannot be read (permission errors).

### 🧭 Phase 2: Navigation
*Goal: Move between folders.*

- [ ] **Enter Directory**: Handle "double-click" or button press on a directory to update `current_path`.
- [ ] **Address Bar**: Display the current path in a text input (editable to jump to paths).
- [ ] **Navigation History**: Implement a simple `Vec` stack for "Back" and "Forward" buttons.

### ⚡ Phase 3: Core Operations
*Goal: Manipulate files. (Note: Use Iced `Task` for async file IO to avoid freezing the UI).*

- [ ] **File Selection**: Visual state change when clicking a file (highlighting).
- [ ] **Open Files**: :Launch files with default system app. Example: 'xdg-open 'https://www.google.com' 2>/dev/null'
- [ ] **Create Folder**: A simple modal or input field to `fs::create_dir`.
- [ ] **Delete**: Implement `fs::remove_file` / `fs::remove_dir_all` (Add a confirmation dialog!).
- [ ] **Rename**: Input field toggling on a selected file to `fs::rename`.
