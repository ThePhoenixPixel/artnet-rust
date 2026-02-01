use artnet::controller::ArtNetController;
use artnet::package::ArtNetDmxPacket;
use bx::network::address::Address;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("  Art-Net DMX Controller - Examples");
    println!("===========================================\n");

    // Your Art-Net node IP address
    let address = Address::new(&"192.168.178.125".to_string(), &6454);
    let mut controller = ArtNetController::new(address.clone())?;

    println!("Connected to Art-Net Node: {}\n", address.to_string());

    // ===========================================
    // Example 1: Set Individual Channels
    // ===========================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ Example 1: Set Individual Channels      │");
    println!("└─────────────────────────────────────────┘");
    
    let mut packet = ArtNetDmxPacket::new(0); // Universe 0
    packet.set_channel(1, 255);
    packet.set_channel(2, 255);
    packet.set_channel(3, 255);
    packet.set_channel(4, 255);
    controller.send_dmx(packet)?;
    
    println!("✓ Set channels 1, 2, 3, 4 to 255 (100%)");
    println!();
    
    thread::sleep(Duration::from_secs(2));

    // ===========================================
    // Example 2: Set Multiple Channels at Once
    // ===========================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ Example 2: Set Multiple Channels        │");
    println!("└─────────────────────────────────────────┘");
    
    let mut packet = ArtNetDmxPacket::new(0);
    packet.set_channels(1, &[255, 0, 0, 255]); // Channels 1-4
    controller.send_dmx(packet)?;
    
    println!("✓ Channels 1 and 4: 255 (100%)");
    println!("✓ Channels 2 and 3: 0 (0%)");
    println!();
    
    thread::sleep(Duration::from_secs(2));

    // ===========================================
    // Example 3: Smooth Fade Effect
    // ===========================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ Example 3: Fade Effect (Blue)           │");
    println!("└─────────────────────────────────────────┘");
    
    println!("Fading from 0 to 255...");
    for brightness in (0..=255).step_by(5) {
        let mut packet = ArtNetDmxPacket::new(0);
        packet.set_channel(1, 0);
        packet.set_channel(2, 0);
        packet.set_channel(3, brightness);
        packet.set_channel(4, 0);
        controller.send_dmx(packet)?;
        thread::sleep(Duration::from_millis(20));
    }
    
    println!("✓ Fade complete");
    println!();
    
    thread::sleep(Duration::from_secs(1));

    // ===========================================
    // Blackout - Turn Off All Lights
    // ===========================================
    println!("┌─────────────────────────────────────────┐");
    println!("│ Blackout - All Channels Off             │");
    println!("└─────────────────────────────────────────┘");
    
    let packet = ArtNetDmxPacket::new(0);
    controller.send_dmx(packet)?;
    
    println!("✓ All lights turned off");
    println!();
    
    println!("===========================================");
    println!("  Examples Complete!");
    println!("===========================================");

    Ok(())
}
