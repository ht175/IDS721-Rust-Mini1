use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Rachel",
    about = "Chicken and Rabbit in same case problem"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Rachel")]
    CelsiusToFahrenheit {
        #[clap(short, long)]
        celsius: f32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::CelsiusToFahrenheit { celsius }) => {
            let fahrenheit = myproject::get_fahrenheit(celsius);

            println!("fahrenheit: {fahrenheit}");
        }
        None => println!("No subcommand was used"),
    }
}
