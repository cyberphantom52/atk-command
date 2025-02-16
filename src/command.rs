use super::types::{CommandId, EEPROMAddress};

/*
| Command ID | Command Status | EEPROM Address | Data Valid Length |     Data     | Checksum |
|------------|----------------|----------------|-------------------|--------------|----------|
|   1 Byte   |     1 Byte     |     2 Bytes    |       1 Byte      |   10 Bytes   |  1 Byte  |
|------------|----------------|----------------|-------------------|--------------|----------|
BASE_OFFSET: The offset of the first byte of the data field
*/
pub trait Command {
    fn base_offset(&self) -> usize;

    fn id(&self) -> CommandId;

    fn set_id(&mut self, id: CommandId);

    fn status(&self) -> u8;

    fn set_status(&mut self, status: u8);

    fn eeprom_address(&self) -> EEPROMAddress;

    fn set_eeprom_address(&mut self, address: EEPROMAddress);

    fn valid_data_len(&self) -> u8;

    fn set_valid_data_len(&mut self, len: u8);

    fn checksum(&self) -> u8;

    fn set_checksum(&mut self);

    fn as_bytes(&self) -> &[u8];
}
