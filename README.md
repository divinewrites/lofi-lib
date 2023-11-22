## lofi-lib 🎧

A simple and fast way to scrape lofi-collection albums

## Features

- **List Albums**: Lists all found albums from a lofi-collection (based on given URL) mapped to the URL (containing all tracks).
- **User Interaction**: Utilize a command-line interface (CLI) to input URLs, specify output file names, set sleep durations, and configure retry attempts.
- **Logging Enhancements**: Improved logging with timestamps and configurable logging levels.
- **Concurrency**: Fetch data from multiple URLs concurrently for faster execution.
- **Customization**: Allow users to customize the CSV output format, such as choosing the delimiter or including/excluding specific fields.
- **Error Handling and Recovery**: Detailed error messages and proper error handling mechanisms for a robust experience.

## Getting Started

### Prerequisites

Ensure you have Rust installed. If not, download it from [rustup](https://rustup.rs/).

### Clone the Repository

```bash
git clone https://github.com/divinewrites/lofi-lib.git
cd lofi-lib
```

## Usage/Examples

Keep in mind `https://vinyl.lofirecords.com/collections/lofi?page=[number]` only goes from 1 (default) - 6 (max)

Run the program with the default lofi-collection URL (Default can be edited within scraper.rs):

```bash
cargo run
```

To scrape additional lofi-collection pages or other genres (e.g., synthwave), provide the URLs as command-line arguments:

```bash
cargo run "https://vinyl.lofirecords.com/collections/lofi?page=2" "https://vinyl.lofirecords.com/collections/lofi?page=3"
```

## Contributing

Feel free to contribute by opening issues or submitting pull requests. Your feedback and suggestions are valuable.

## Important

ONLY TO BE USED FOR EDUCATIONAL / DEMONSTRATIONAL PURPOSES

Please remember that this website is providing this data for free out of the goodness of their heart. Don't Take Advantage!

Enjoy lofi music with lofi-lib! 🎧😝
