use std::collections::VecDeque;
use colored::{ColoredString, Colorize};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

// Config
struct Config { 
    modules: Vec<Module>,
}
impl Config {
    // Default config
    pub fn default() -> Self {
        // Title
        let title = TitleOpts { user: whoami::username(), hostname: whoami::devicename() };
        let title_line_len = title.user.len() + 1 + title.hostname.len();

        Config {
            modules: vec![
                Module::Title { opts: title },
                Module::Line { len: title_line_len },
                Module::OS,
                Module::Kernel,
                Module::Uptime { short: false },
                Module::Shell,
                Module::Memory,
                Module::CPU,
                Module::GPU,
            ],
            // NOTE: This was deprecated, use system colors
            //colors: vec![ // No less than 5 colors
            //    CustomColor { r: 139, g: 118, b: 187 }, // Purple
            //    CustomColor { r: 247, g: 118, b: 142 }, // Red
            //    CustomColor { r: 125, g: 207, b: 255 }, // Blue
            //    CustomColor { r: 223, g: 167, b: 72 },  // Yellow
            //    CustomColor { r: 158, g: 206, b: 106 }, // Green
            //]
        }
    }
}

// Modules
#[allow(non_camel_case_types, dead_code)]
enum Module {
    Title { opts: TitleOpts },
    Line { len: usize },
    OS,
    Kernel,
    Uptime { short: bool },
    Shell, 
    //DE,
    //WM,
    //Terminal
    Memory,
    CPU,
    GPU,
    CPU_Usage,// TODO:
    Disk,     // TODO:
    Battery,  // TODO:
    Font,     // TODO:
    Song,     // TODO:
    Local_IP, // TODO:
    Public_IP,// TODO:
    Birthday, // TODO:
}

impl Module {
    pub fn display(self) -> ColoredString {
        // Sys struct with processes
        let mut sys = sysinfo::System::new_with_specifics(
            RefreshKind::nothing()
                .with_processes(ProcessRefreshKind::everything())
        );

        // TODO: Custom module subtitles
        // TODO: Custom colors on modules
        match self {
            Module::Title{ opts } => format!(
                "{}{}{}",
                opts.user.bright_blue(),
                "@".yellow(),
                opts.hostname.bright_green(),
            ),
            // TODO: custom line char
            Module::Line { len } => format!(
                "{}", "-".to_string().repeat(len), 
            ),
            Module::OS => color_module(
                "OS", whoami::distro(),
            ),
            Module::Kernel => color_module(
                "Kernel", System::kernel_long_version(),
            ),
            Module::Uptime { short } => color_module(
                "Uptime", format_time(System::uptime(), short), 
            ),
            Module::Shell => color_module(
                "Shell",get_current_shell(&mut sys),
            ),
            Module::Memory => color_module(
                "Memory", get_memory(&mut sys),
            ),
            Module::CPU => color_module(
                "CPU",
                sys.cpus()[0].brand(),
            ),
            Module::GPU => color_module(
                "GPU",
                get_gpu_info(),
            ),
            _ => "Not implemented".to_string()
        }.into()
    }
}

// Module options
struct TitleOpts {
    user: String,
    hostname: String 
}

// Util methods
fn format_time(seconds: u64, short: bool) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    // TODO: Theres probably more edge cases here not covered
    if short {
        match (hours, minutes, secs) {
            (0, 0, s) => format!("{}s", s),
            (0, m, s) => format!("{}m {}s", m, s),
            (h, m, s) => format!("{}h {}m {}s", h, m, s),
        }
    } else { 
        match (hours, minutes, secs) {
            (0, 0, s) => format!("{} seconds", s),
            (0, m, s) => format!("{} minutes, {} seconds", m, s),
            (h, m, s) => format!("{} hours, {} minutes, {} seconds", h, m, s),
        }
    }
}

fn get_current_shell(sys: &mut System) -> String {
    // Get current pid
    let pid = match sysinfo::get_current_pid() {
        Ok(pid) => pid,
        Err(_) => return "unknown".into(),
    };

    // Attempt to get parent pid
    let ppid = match sys.process(pid) {
        Some(p) => match p.parent() {
            Some(p) => p,
            None => return "unknown".into()
        },
        None => return "unknown".into()
    };

    // Get and return parent process name
    match sys.process(ppid) {
        Some(p) => {
            // Get string
            let p = p.name().to_string_lossy();

            // Split at . if found
            match p.split_once(".") {
                Some((p, _)) => p.to_string(),
                None => p.to_string()
            }
        }
        None => return "unknown".into()
    }
}

fn get_memory(sys: &mut System) -> String {
    sys.refresh_memory();
    format!("{:.2} MiB / {:.2} MiB", 
        (sys.used_memory() as f64 / 1_048_576.0), 
        sys.total_memory() as f64 / 1_048_576.0
    )
}

// TODO: Check for OS, if on linux call seperate function
fn get_gpu_info() -> String {
    // Get graphics device
    let mut graphics = winsafe::EnumDisplayDevices(None, None);
    let graphics = graphics.next();

    // Not found
    if graphics.is_none() || graphics.unwrap().is_err() { 
        return "Unknown".to_string();
    }
    let graphics = graphics.unwrap().unwrap();

    // Return graphics string
    graphics.DeviceString()
}

// Creates a string with the default module style and colors
fn color_module(
    subtitle: impl AsRef<str>,
    info: impl AsRef<str>, 
) -> String {
    format!(
        "{}: {}",
        subtitle.as_ref().bright_red(),
        info.as_ref().bright_purple()
    )
}

fn main() {
    // Get config
    // TODO: Read from config file
    let conf = Config::default();

    // Default ascii art
    // TODO: Alternative default ascii arts
    let window = vec![
      vec!["      ".clear()," _.-;".red(),";-._ ".green(),],
      vec!["'-..-'".clear(),"|   |".red(),"|   |".green()],
      vec!["'-..-'".clear(),"|_.-;".red(),";-._|".green()],
      vec!["'-..-'".clear(),"|   |".blue(),"|   |".yellow()],
      vec!["'-..-'".clear(),"|_.-'".blue(),"'-._|".yellow()],
    ];

    // Color default ascii art
    let mut art = VecDeque::new();
    for line in window {
        let line = line.iter()
            .map(|l| l.to_string()) // Map to string
            .collect::<Vec<_>>() // Vector
            .join(""); // Join line
        art.push_back(line); // Add to vec
    }

    // Get padding length
    let pad = strip_ansi_escapes::strip_str(art[0].clone()).len();
    art.push_front(" ".to_string().repeat(pad)); // Add padding to top

    // Start printing!
    println!();

    // Loop over modules
    for module in conf.modules {
        // Get next line of ascii art
        // HACK: This will cut off ascii art if art is longer than module listing
        let a = art.pop_front().unwrap_or(" ".to_string().repeat(pad));

        // Print art and module
        println!("  {}  {}", a, module.display());
    }
    // TODO: continue printing art if vec not exhausted

    // Color line
    // TODO: Do this more betterer
    println!();
    print!("{}", " ".to_string().repeat(pad + 4));
    print!("{}", "████".black());
    print!("{}", "████".red());
    print!("{}", "████".green());
    print!("{}", "████".yellow());
    print!("{}", "████".blue());
    print!("{}", "████".purple());
    print!("{}", "████".cyan());
    print!("{}", "████".white());
    println!();
    print!("{}", " ".to_string().repeat(pad + 4));
    print!("{}", "████".bright_black());
    print!("{}", "████".bright_red());
    print!("{}", "████".bright_green());
    print!("{}", "████".bright_yellow());
    print!("{}", "████".bright_blue());
    print!("{}", "████".bright_purple());
    print!("{}", "████".bright_cyan());
    print!("{}", "████".bright_white());
    println!();

    // Newline at end
    println!();
}
