use mh_z19;

const PACKET_SIZE:usize = 9;

// Arrays can't be converted to iter for now, see
// https://doc.rust-lang.org/std/array/struct.IntoIter.html
pub struct PacketIter {
    packet: mh_z19::Packet,
    pos: usize
}

impl From<mh_z19::Packet> for PacketIter {
    fn from(packet: mh_z19::Packet) -> Self {
        PacketIter { packet, pos: 0 }
    }
}

impl Iterator for PacketIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < PACKET_SIZE {
            Some(self.packet[self.pos])
        } else {
            None
        }
    }
}
