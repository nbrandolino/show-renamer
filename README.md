# Show Renamer
`show-renamer` is a command-line utility written in Rust, designed to rename the Seasons and Episodes of a TV show.

## Requirements
- **Rust**: Required to compile the utility.
- **Linux Environment**: Currently designed to work on Linux-based systems.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/nbrandolino/show-renamer.git
   ```
2. Navigate to the project directory:
   ```bash
   cd show-renamer
   ```
3. Install:
   ```bash
   cargo install --path .
   ```

## Usage
Run the tool using the following command:
```bash
show-renamer [OPTIONS]
```

### Available Options
- `-h, --help`: Display help information.
- `-V, --version`: Display version information.
- `-r, --rename`:Renames a TV show's seasons and episodes at the specified path. Optionally, rename the entire show directory.

### Examples
1. Rename TV show files (seasons and episodes):
   ```bash
   show-renamer -r /path/to/show
   ```
2. Rename TV show files (seasons and episodes) and show directory:
   ```bash
   show-renamer -r /path/to/show 'TV Show'
   ```

## License
This tool is licensed under the GNU General Public License (GPL). See the `LICENSE` file for more details.

## Contact
- **Author**: nbrandolino
- **Email**: [nickbrandolino134@gmail.com](mailto:nickbrandolino134@gmail.com)
