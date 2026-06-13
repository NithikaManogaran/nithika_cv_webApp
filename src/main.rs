use askama::Template;
use axum::{routing::get, Router};

struct Project {
    name: String,
    tech: String,
    desc: String,
}

struct Education {
    degree: String,
    institute: String,
    gpa: String,
}

struct Certificate {
    name: String,
    institute: String,
    year: String,
}

#[derive(Template)]
#[template(path = "nithi_cv.html")]
struct CvStructure {
    name: String,
    profile: String,
    skills: Vec<String>,
    projects: Vec<Project>, 
    certificates: Vec<Certificate>,
    education: Education,   
    email: String,
    mobile: String,
    address: String,
    github: String,
    linkedin: String,
    is_github_present: bool,
    is_linkedin_present: bool,
}

#[tokio::main]
async fn main() {
    use tower_http::services::ServeDir;

    let app = Router::new()
    .route("/", get(render_cv))
    .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Web server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn render_cv() -> impl axum::response::IntoResponse {
    
    // Skills Vector
    let tech_skills = vec![
        "Java".to_string(), "Dart".to_string(), "PHP".to_string(), "C#".to_string(),
        "Flutter".to_string(), "Laravel".to_string(),"Python (basics)".to_string(), "MySQL".to_string(), 
        "PostgreSQL".to_string(), "Supabase".to_string(), "Drift".to_string(),
        "Rust (Learning)".to_string()
    ];

    // Projects Vector
    let my_projects = vec![
        Project {
            name: "AAROKKIYAAM (Individual Project)".to_string(),
            tech: "Flutter, Drift, PowerSync, Supabase".to_string(),
            desc: "Multi-tenant Hospital Management System built with a local-first architecture for regional connectivity efficiency.".to_string(),
        },
        Project {
            name: "ABHAYAM (Individual Project)".to_string(),
            tech: "Flutter & Mobile UI".to_string(),
            desc: "A mobile application ecosystem designed to bridge and connect blood and organ donors seamlessly.".to_string(),
        },
        Project {
            name: "Face Recognition Attendance System (Group Project)".to_string(),
            tech: "Python, OpenCV".to_string(),
            desc: "An automated attendance system for students using face recognition technology.".to_string(),
        },
        Project {
            name: "Lions District 306 D12 Official Website".to_string(),
            tech: "Web Development & Designing".to_string(),
            desc: "The official web platform developed for managing and showcasing Lions District organizational activities.".to_string(),
        }
    ];

    //Certificates Vector
    let my_certificates = vec![
        Certificate {
            name: "CERTIFIED ETHICAL HACKER (CEH V11) TRAINING COURSE".to_string(),
            institute: "Mars Tech".to_string(),
            year: "2023".to_string(),
        },
        Certificate {
        
            name: "CERTIFICATE COURSE IN COMPUTER SCIENCE".to_string(),
            institute: "Buldosoft Academy".to_string(),
            year: "2023".to_string(),
        },
        Certificate {
            name: "CERTIFICATE COURSE IN CYBERSECURITY".to_string(),
            institute: "CISCO Networking Academy".to_string(),
            year: "2023".to_string(),
        }
    ];

    // Education Object
    let edu_details = Education {
        degree: "Higher National Diploma in Information Technology (HNDIT)".to_string(),
        institute: "Advanced Technological Institute (ATI), Jaffna".to_string(),
        gpa: "1st Year: GPA 3.94 & 4.0 /4.0".to_string(),
    };

    let my_cv = CvStructure {
        name: "Miss.Nithika Manogaran".to_string(),
        profile: "Highly motivated HNDIT student at ATI Jaffna with a strong foundation in software development and relational database ecosystems. Fast and self-driven learner focused on mastering backend architectures and modern frameworks.".to_string(),
        skills: tech_skills,
        projects: my_projects,
        education: edu_details,
        certificates: my_certificates,
        email: "nittymano971@gmail.com".to_string(),
        mobile: "+94 75 338 0337".to_string(),
        address: "Chavakachcheri, Jaffna".to_string(),
        github: "https://github.com/NithikaManogaran".to_string(),
        linkedin: "https://www.linkedin.com/in/nithika-manogaran-a4b836276/".to_string(),
        is_github_present: true,
        is_linkedin_present: true,
    };

    my_cv
}