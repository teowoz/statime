use crate::datastructures::{WireFormat, WireFormatError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    /// The seconds field of the timestamp.
    /// 48-bit, must be less than 281474976710656
    pub seconds: u64,
    /// The nanoseconds field of the timestamp.
    /// Must be less than 10^9
    pub nanos: u32,
}

impl WireFormat for Timestamp {
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, WireFormatError> {
        buffer[0..6].copy_from_slice(&self.seconds.to_be_bytes()[2..8]);
        buffer[6..10].copy_from_slice(&self.nanos.to_be_bytes());
        Ok(10)
    }

    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), WireFormatError> {
        let mut seconds_buffer = [0; 8];
        seconds_buffer[2..8].copy_from_slice(&buffer[0..6]);

        Ok((
            Self {
                seconds: u64::from_be_bytes(seconds_buffer.try_into().unwrap()),
                nanos: u32::from_be_bytes(buffer[6..10].try_into().unwrap()),
            },
            10,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_wireformat() {
        let representations = [
            (
                [0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01u8],
                Timestamp {
                    seconds: 0x0000_0000_0002,
                    nanos: 0x0000_0001,
                },
            ),
            (
                [0x10, 0x00, 0x00, 0x00, 0x00, 0x02, 0x10, 0x00, 0x00, 0x01u8],
                Timestamp {
                    seconds: 0x1000_0000_0002,
                    nanos: 0x1000_0001,
                },
            ),
        ];

        for (byte_representation, object_representation) in representations {
            // Test the serialization output
            let mut serialization_buffer = [0; 10];
            object_representation
                .serialize(&mut serialization_buffer)
                .unwrap();
            assert_eq!(serialization_buffer, byte_representation);

            // Test the deserialization output
            let deserialized_data = Timestamp::deserialize(&byte_representation).unwrap().0;
            assert_eq!(deserialized_data, object_representation);
        }
    }
}
