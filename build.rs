fn main() {
    let cmd = std::process::Command::new("npx")
        .args(["tailwindcss", "-i", "./input.css", "-o", "./style/output.css"])
        .status();

    if let Err(e) = cmd {
        println!("cargo:warning=TailwindCSS Error: {}", e);
    }
}
