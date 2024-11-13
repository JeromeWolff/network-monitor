# Network Packet Sniffer

A simple Rust application that captures and monitors network packets on a specified network interface, periodically
printing and saving packet metrics to a JSON file.

## Features

- Captures packets on a specified network interface.
- Counts the total packets and bytes captured.
- Outputs metrics to the console every 5 seconds.
- Saves metrics snapshots to a JSON file (`metrics.json`).

## Prerequisites

- **Rust**: Ensure that Rust is installed. [Install Rust](https://www.rust-lang.org/tools/install)
- **pnet crate**: This application uses the `pnet` crate for network interface handling and packet processing.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/network-packet-sniffer.git
   cd network-packet-sniffer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application by specifying a network interface to capture packets on. You can find available interfaces using the
`ip a` command on Linux.

```bash
cargo run --release <interface_name>
```

Replace `<interface_name>` with the name of the network interface to capture packets on.

## Contributing

Contributions are welcome! Please open an issue to discuss what you would like to change.

### Contributors

<!-- readme: collaborators,contributors -start -->
<table>
	<tbody>
		<tr>
            <td align="center">
                <a href="https://github.com/JeromeWolff">
                    <img src="https://avatars.githubusercontent.com/u/34198274?v=4" width="100;" alt="JeromeWolff"/>
                    <br />
                    <sub><b>Jerome Wolff</b></sub>
                </a>
            </td>
		</tr>
	<tbody>
</table>
<!-- readme: collaborators,contributors -end -->
