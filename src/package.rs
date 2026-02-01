
const ARTNET_OPCODE_DMX: u16 = 0x5000;

pub struct ArtNetDmxPacket {
    header: [u8; 8],      
    opcode: u16,          
    protocol_version: u16,
    sequence: u8,         
    physical: u8,         
    universe: u16,        
    length: u16,          
    data: [u8; 512],      
}

impl ArtNetDmxPacket {
    pub fn new(universe: u16) -> Self {
        ArtNetDmxPacket {
            header: *b"Art-Net\0",
            opcode: ARTNET_OPCODE_DMX,
            protocol_version: 14,
            sequence: 0,
            physical: 0,
            universe,
            length: 512,
            data: [0; 512],
        }
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) {
        if channel > 0 && channel <= 512 {
            self.data[channel - 1] = value;
        }
    }

    pub fn set_channels(&mut self, start_channel: usize, values: &[u8]) {
        if start_channel > 0 && start_channel <= 512 {
            let end = (start_channel - 1 + values.len()).min(512);
            self.data[start_channel - 1..end].copy_from_slice(&values[..end - start_channel + 1]);
        }
    }
    
    pub fn set_sequence(&mut self, seq: u8) {
        self.sequence = seq; 
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(18 + 512);

        // Header
        bytes.extend_from_slice(&self.header);

        // OpCode (Little Endian)
        bytes.extend_from_slice(&self.opcode.to_le_bytes());

        // Protocol Version (Big Endian)
        bytes.extend_from_slice(&self.protocol_version.to_be_bytes());

        // Sequence
        bytes.push(self.sequence);

        // Physical
        bytes.push(self.physical);

        // Universe (Little Endian)
        bytes.extend_from_slice(&self.universe.to_le_bytes());

        // Length (Big Endian)
        bytes.extend_from_slice(&self.length.to_be_bytes());

        // DMX Data
        bytes.extend_from_slice(&self.data);

        bytes
    }
}



