# My CV Web Application

This is a simple and clean CV web application built using the **Rust** programming language. I created this project to learn about backend web development with Rust and template rendering.

## 🛠️ Tech Stack Used

- **Backend Framework:** Axum
- **Template Engine:** Askama (for rendering HTML dynamically)
- **File Serving:** Tower-HTTP (used to display the profile picture)
- **Frontend Styling:** Tailwind CSS (via CDN)

## 📂 Project Folders

- `src/main.rs` - The main backend file containing my server configuration, custom structs, and resume data.
- `templates/nithi_cv.html` - The HTML template file that uses loops to display my skills, projects, and certificates dynamically.
- `assets/` - A folder to store my profile picture (`nithi_profile.jpg`).
- `Cargo.toml` - Contains the dependencies used in this project.
