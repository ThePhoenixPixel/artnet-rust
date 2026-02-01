# Art-Net DMX Controller in Rust 🎭

A simple and efficient Art-Net DMX512 controller implementation in Rust for controlling lighting fixtures over Ethernet.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🎯 About

This project provides a lightweight Rust implementation for controlling Art-Net nodes. Art-Net is an Ethernet protocol for transmitting DMX512 lighting control data, commonly used in stage lighting, architectural lighting, and entertainment applications.

## ✨ Features

- 🚀 Simple and intuitive API
- 📦 Minimal dependencies
- 🎨 Control up to 512 DMX channels per universe
- 🌐 Support for multiple Art-Net universes
- ⚡ High-performance UDP communication
- 🔧 Easy integration into existing projects
- 📝 Well-documented code examples

## 📦 Prerequisites

- **Rust** 1.70.0 or higher ([Install Rust](https://rustup.rs/))
- An Art-Net compatible node or software (e.g., QLC+, Enttec ODE, DMXKing)
- Basic network connectivity

## 🔧 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/ThePhoenixPixel/artnet-rust.git
cd artnet-rust
```

### 2. Build the Project

```bash
cargo build
```

### 3. Run the Example

```bash
cargo run --example examples
```

## 🚀 Usage

### Basic Setup

```rust
use artnet::controller::ArtNetController;
use artnet::package::ArtNetDmxPacket;
use bx::network::address::Address;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure your Art-Net node IP address
    let address = Address::new(&"192.168.178.125".to_string(), &6454);
    
    // Create controller instance
    let mut controller = ArtNetController::new(address.clone())?;
    
    // Create a DMX packet for Universe 0
    let mut packet = ArtNetDmxPacket::new(0);
    
    // Set channel values (1-512, values 0-255)
    packet.set_channel(1, 255);
    packet.set_channel(2, 128);
    packet.set_channel(3, 0);
    packet.set_channel(4, 0);
    
    // Send the packet
    controller.send_dmx(packet)?;
    
    Ok(())
}
```

## 📚 Examples

### Example 1: Set Individual Channels

Control individual DMX channels with specific values:

```rust
let mut packet = ArtNetDmxPacket::new(0); // Universe 0
packet.set_channel(1, 255); // Channel 1: 100%
packet.set_channel(2, 255); // Channel 2: 100%
packet.set_channel(3, 255); // Channel 3: 100%
packet.set_channel(4, 255); // Channel 4: 100%
controller.send_dmx(packet)?;
println!("✓ Channels 1-4 set to 255 (100%)");
```

### Example 2: Set Multiple Channels at Once

Use arrays for efficient multi-channel control:

```rust
let mut packet = ArtNetDmxPacket::new(0);
packet.set_channels(1, &[255, 0, 0, 255]); // Channels 1-4
controller.send_dmx(packet)?;
println!("✓ Channels 1,4: 255 (100%) | Channels 2,3: 0 (0%)");
```

### Example 3: Smooth Fade Effect

Create dynamic lighting effects with gradual transitions:

```rust
println!("Starting fade effect...");
for brightness in (0..=255).step_by(5) {
    let mut packet = ArtNetDmxPacket::new(0);
    packet.set_channel(1, 0);
    packet.set_channel(2, 0);
    packet.set_channel(3, brightness);
    controller.send_dmx(packet)?;
    thread::sleep(Duration::from_millis(20));
}
println!("✓ Fade complete");
```

### Example 4: RGB LED Control

Control RGB fixtures with ease:

```rust
// Red
let mut packet = ArtNetDmxPacket::new(0);
packet.set_channels(1, &[255, 0, 0]);
controller.send_dmx(packet)?;
thread::sleep(Duration::from_secs(1));

// Green
packet.set_channels(1, &[0, 255, 0]);
controller.send_dmx(packet)?;
thread::sleep(Duration::from_secs(1));

// Blue
packet.set_channels(1, &[0, 0, 255]);
controller.send_dmx(packet)?;
```

### Example 5: Blackout (Turn Off All Lights)

```rust
let packet = ArtNetDmxPacket::new(0);
// All channels default to 0
controller.send_dmx(packet)?;
println!("✓ All lights off");
```

## ⚙️ Configuration

### Configuring the IP Address

Update the IP address in your code:

```rust
// Replace with your Art-Net node's IP address
let address = Address::new(&"192.168.178.125".to_string(), &6454);
```

> **Note**: Art-Net typically uses UDP port `6454`

### Setting the Universe

Art-Net supports multiple universes (0-32767), each with 512 channels:

```rust
let packet = ArtNetDmxPacket::new(0);  // Universe 0
let packet = ArtNetDmxPacket::new(1);  // Universe 1
```

## 🎛️ DMX Channel Reference

DMX512 provides **512 channels** per universe with values ranging from **0-255**:

- `0` = Off / Minimum
- `127` = 50%
- `255` = On / Maximum

### Common Fixture Types

#### RGB Light (3 channels)
```
Channel 1: Red (0-255)
Channel 2: Green (0-255)
Channel 3: Blue (0-255)
```

#### RGBW Light (4 channels)
```
Channel 1: Red
Channel 2: Green
Channel 3: Blue
Channel 4: White
```

#### RGBWA Light (5 channels)
```
Channel 1: Red
Channel 2: Green
Channel 3: Blue
Channel 4: White
Channel 5: Amber
```

#### Simple Dimmer (1 channel)
```
Channel 1: Brightness (0-255)
```

> **Tip**: Always consult your fixture's manual for the exact DMX channel mapping!

## 🔍 Troubleshooting

### Connection Issues

**Problem**: Cannot connect to Art-Net node

**Solutions**:
- ✅ Verify the IP address is correct
- ✅ Check both devices are on the same network/subnet
- ✅ Disable firewall or allow UDP port 6454
- ✅ Try broadcast address `255.255.255.255` for discovery
- ✅ Ensure the Art-Net node is powered on and connected

### No Response from Fixtures

**Problem**: Lights don't respond to commands

**Solutions**:
- ✅ Verify DMX addresses on your fixtures
- ✅ Check universe number matches your node configuration
- ✅ Confirm channel mapping in fixture manual
- ✅ Test with Art-Net viewer software (e.g., QLC+)
- ✅ Check network traffic with Wireshark

### Performance Issues

**Problem**: Choppy or laggy light updates

**Solutions**:
- ✅ Limit DMX update rate to ~44 Hz (max for DMX512)
- ✅ Add appropriate delays between updates
- ✅ Reduce packet sending frequency
- ✅ Check network bandwidth and latency


## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Art-Net™ is a trademark of Artistic Licence Holdings Ltd.
- Thanks to the Rust community for excellent tools and libraries

## 💡 Use Cases

- 🎭 **Stage Lighting**: Control theater and concert lighting
- 🏛️ **Architectural Lighting**: Building and monument illumination
- 🎉 **Event Production**: DJ setups, weddings, corporate events
- 🏠 **Home Automation**: Smart home lighting integration
- 🎨 **Art Installations**: Interactive light art
- 🎮 **Entertainment**: Theme parks, escape rooms

---

**Made with ❤️ and Rust**

If you find this project helpful, please consider giving it a ⭐!
