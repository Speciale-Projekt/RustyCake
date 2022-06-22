use std::borrow::{BorrowMut, Borrow};


#[derive(Debug)]
pub(crate) enum COMMANDS {
    LINK_REQUEST = 0x00,
    LINK_ACCEPT = 0x01,
    LINK_ACCEPT_AND_REQUEST = 0x02,
    LINK_REJECT = 0x03,
    ADVERTISEMENT = 0x04,
    UPDATE = 0x05,
    UPDATE_REQUEST = 0x06,
    DATA_REQUEST = 0x07,
    DATA_RESPONSE = 0x08,
    PARENT_REQUEST = 0x09,
    PARENT_RESPONSE = 0x0A,
    CHILD_ID_REQUEST = 0x0B,
    CHILD_ID_RESPONSE = 0x0C,
    CHILD_UPDATE_REQUEST = 0x0D,
    CHILD_UPDATE_RESPONSE = 0x0E,
    ANNOUNCE = 0x0F,
    DISCOVERY_REQUEST = 0x10,
    DISCOVERY_RESPONSE = 0x11,
    UNKNOWN_COMMAND = 0xFF
}
impl COMMANDS {
    fn value(&self) -> u8 {
        match *self {
            COMMANDS::LINK_REQUEST => 0x00,
            COMMANDS::LINK_ACCEPT => 0x01,
            COMMANDS::LINK_ACCEPT_AND_REQUEST => 0x02,
            COMMANDS::LINK_REJECT => 0x03,
            COMMANDS::ADVERTISEMENT => 0x04,
            COMMANDS::UPDATE => 0x05,
            COMMANDS::UPDATE_REQUEST => 0x06,
            COMMANDS::DATA_REQUEST => 0x07,
            COMMANDS::DATA_RESPONSE => 0x08,
            COMMANDS::PARENT_REQUEST => 0x09,
            COMMANDS::PARENT_RESPONSE => 0x0A,
            COMMANDS::CHILD_ID_REQUEST => 0x0B,
            COMMANDS::CHILD_ID_RESPONSE => 0x0C,
            COMMANDS::CHILD_UPDATE_REQUEST => 0x0D,
            COMMANDS::CHILD_UPDATE_RESPONSE => 0x0E,
            COMMANDS::ANNOUNCE => 0x0F,
            COMMANDS::DISCOVERY_REQUEST => 0x10,
            COMMANDS::DISCOVERY_RESPONSE => 0x11,
            COMMANDS::UNKNOWN_COMMAND => 0xFF
        }
    }
}
impl From<u8> for COMMANDS {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => COMMANDS::LINK_REQUEST,
            0x01 => COMMANDS::LINK_ACCEPT,
            0x02 => COMMANDS::LINK_ACCEPT_AND_REQUEST,
            0x03 => COMMANDS::LINK_REJECT,
            0x04 => COMMANDS::ADVERTISEMENT,
            0x05 => COMMANDS::UPDATE,
            0x06 => COMMANDS::UPDATE_REQUEST,
            0x07 => COMMANDS::DATA_REQUEST,
            0x08 => COMMANDS::DATA_RESPONSE,
            0x09 => COMMANDS::PARENT_REQUEST,
            0x0A => COMMANDS::PARENT_RESPONSE,
            0x0B => COMMANDS::CHILD_ID_REQUEST,
            0x0C => COMMANDS::CHILD_ID_RESPONSE,
            0x0D => COMMANDS::CHILD_UPDATE_REQUEST,
            0x0E => COMMANDS::CHILD_UPDATE_RESPONSE,
            0x0F => COMMANDS::ANNOUNCE,
            0x10 => COMMANDS::DISCOVERY_REQUEST,
            0x11 => COMMANDS::DISCOVERY_RESPONSE,
            _ => COMMANDS::UNKNOWN_COMMAND
        }
    }
}

#[derive(Debug)]
pub(crate) enum TLVS {
    SOURCE_ADDRESS_TLV = 0x00,
    MODE_TLV = 0x01,
    TIMEOUT_TLV = 0x02,
    CHALLENGE_TLV = 0x03,
    RESPONSE_TLV = 0x04,
    LINK_LAYER_FRAME_COUNTER_TLV = 0x05,
    LINK_QUALITY_TLV = 0x06,
    NETWORK_PARAMETER_TLV = 0x07,
    MLE_FRAME_COUNTER_TLV = 0x08,
    ROUTE64_TLV = 0x09,
    ADDRESS16_TLV = 0x0A,
    LEADER_DATA_TLV = 0x0B,
    NETWORK_DATA_TLV = 0x0C,
    TLV_REQUEST_TLV = 0x0D,
    SCAN_MASK_TLV = 0x0E,
    CONNECTIVITY_TLV = 0x0F,
    LINK_MARGIN_TLV = 0x10,
    STATUS_TLV = 0x11,
    VERSION_TLV = 0x12,
    ADDRESS_REGISTRATION_TLV = 0x13,
    CHANNEL_TLV = 0x14,
    PAN_ID_TLV = 0x15,
    ACTIVE_TIMESTAMP_TLV = 0x16,
    PENDING_TIMESTAMP_TLV = 0x17,
    ACTIVE_OPERATIONAL_DATASET_TLV = 0x18,
    PENDING_OPERATIONAL_DATASET_TLV = 0x19,
    THREAD_DISCOVERY_TLV = 0x1A,
    UNKNOWN_TLV = 0xFF
}
impl TLVS {
    
    fn value(&self) -> u8 {
        match *self {
            TLVS::SOURCE_ADDRESS_TLV => 0x00,
            TLVS::MODE_TLV => 0x01,
            TLVS::TIMEOUT_TLV => 0x02,
            TLVS::CHALLENGE_TLV => 0x03,
            TLVS::RESPONSE_TLV => 0x04,
            TLVS::LINK_LAYER_FRAME_COUNTER_TLV => 0x05,
            TLVS::LINK_QUALITY_TLV => 0x06,
            TLVS::NETWORK_PARAMETER_TLV => 0x07,
            TLVS::MLE_FRAME_COUNTER_TLV => 0x08,
            TLVS::ROUTE64_TLV => 0x09,
            TLVS::ADDRESS16_TLV => 0x0A,
            TLVS::LEADER_DATA_TLV => 0x0B,
            TLVS::NETWORK_DATA_TLV => 0x0C,
            TLVS::TLV_REQUEST_TLV => 0x0D,
            TLVS::SCAN_MASK_TLV => 0x0E,
            TLVS::CONNECTIVITY_TLV => 0x0F,
            TLVS::LINK_MARGIN_TLV => 0x10,
            TLVS::STATUS_TLV => 0x11,
            TLVS::VERSION_TLV => 0x12,
            TLVS::ADDRESS_REGISTRATION_TLV => 0x13,
            TLVS::CHANNEL_TLV => 0x14,
            TLVS::PAN_ID_TLV => 0x15,
            TLVS::ACTIVE_TIMESTAMP_TLV => 0x16,
            TLVS::PENDING_TIMESTAMP_TLV => 0x17,
            TLVS::ACTIVE_OPERATIONAL_DATASET_TLV => 0x18,
            TLVS::PENDING_OPERATIONAL_DATASET_TLV => 0x19,
            TLVS::THREAD_DISCOVERY_TLV => 0x1A,
            TLVS::UNKNOWN_TLV => 0xFF
        }
    }
}

impl From<u8> for TLVS {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => TLVS::SOURCE_ADDRESS_TLV,
            0x01 => TLVS::MODE_TLV,
            0x02 => TLVS::TIMEOUT_TLV,
            0x03 => TLVS::CHALLENGE_TLV,
            0x04 => TLVS::RESPONSE_TLV,
            0x05 => TLVS::LINK_LAYER_FRAME_COUNTER_TLV,
            0x06 => TLVS::LINK_QUALITY_TLV,
            0x07 => TLVS::NETWORK_PARAMETER_TLV,
            0x08 => TLVS::MLE_FRAME_COUNTER_TLV,
            0x09 => TLVS::ROUTE64_TLV,
            0x0A => TLVS::ADDRESS16_TLV,
            0x0B => TLVS::LEADER_DATA_TLV,
            0x0C => TLVS::NETWORK_DATA_TLV,
            0x0D => TLVS::TLV_REQUEST_TLV,
            0x0E => TLVS::SCAN_MASK_TLV,
            0x0F => TLVS::CONNECTIVITY_TLV,
            0x10 => TLVS::LINK_MARGIN_TLV,
            0x11 => TLVS::STATUS_TLV,
            0x12 => TLVS::VERSION_TLV,
            0x13 => TLVS::ADDRESS_REGISTRATION_TLV,
            0x14 => TLVS::CHANNEL_TLV,
            0x15 => TLVS::PAN_ID_TLV,
            0x16 => TLVS::ACTIVE_TIMESTAMP_TLV,
            0x17 => TLVS::PENDING_TIMESTAMP_TLV,
            0x18 => TLVS::ACTIVE_OPERATIONAL_DATASET_TLV,
            0x19 => TLVS::PENDING_OPERATIONAL_DATASET_TLV,
            0x1A => TLVS::THREAD_DISCOVERY_TLV,
            _ => TLVS::UNKNOWN_TLV
        }
    }
}

#[derive(Debug)]
pub(crate) struct Command {
    pub command: COMMANDS,
    pub tlvs: Vec<Tlv>,
    pub payload: Vec<u8>
}
impl ToString for Command {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("{:?}", self.command));
        s.push_str("\n");
        for tlv in &self.tlvs {
            s.push_str(&format!("{:?}\n", tlv));
        }
        s.push_str("\n");
        s.push_str(&format!("{:X?}", self.payload));
        s
    }
}

#[derive(Debug)]
pub(crate) struct Tlv {
    pub tlv_type: TLVS,
    pub length: usize,
    pub value: Vec<u8>,
}

impl ToString for Tlv {
    fn to_string(&self) -> String {
        format!("{:?},{:?},{:X?}", self.tlv_type, self.length, self.value)
    }
}

fn assign_tlvs(msg: Vec<u8>) -> (usize,  Vec<Tlv>)  {
    // if msg is empty return
    let mut tlvs: Vec<Tlv> = Vec::new();


    if msg.len() == 0 {
        return (0, Vec::new());
    }
    // If the next index is either 0xFF or (0x00 and after that 0x15) then return the result and the next index
    let mut next_index: usize = 0;
    loop {
        if msg[next_index] == 0xFF || (msg[next_index] == 0x00 && msg[next_index + 1] == 0x15) {
            return (next_index, tlvs);
        }
        let mut tlv_length = msg[next_index+1] as usize;
        if tlv_length > msg.len() - 2 {
            tlv_length = msg.len() - 2;
        }
        match msg[next_index] {
            x if x >= TLVS::SOURCE_ADDRESS_TLV.value() as u8 && x <= TLVS::THREAD_DISCOVERY_TLV.value() as u8 => {
               tlvs.push(Tlv {
                    tlv_type: x.into(),
                    length: tlv_length,
                    value: msg[2..tlv_length + 2].to_vec(),
                });
            }
            _ => {
                tlvs.push (Tlv {
                    tlv_type: TLVS::UNKNOWN_TLV,
                    length: tlv_length,
                    value: msg[2..tlv_length + 2].to_vec(),
                });
                }
            }
        next_index += tlv_length + 2;
        if next_index >= msg.len() {
            break;
        }
    }
    return (0, tlvs);
}

pub(crate) fn assign_command(message: Vec<u8>) -> Vec<Command> {
    // If message is empty just return nothing.
    if message.len() == 0 {
        return Vec::new();
    }
    let mut commands: Vec<Command> = Vec::new();
    let mut next_index = 0;
    let mut command_index = 0;
    let mut vv: Vec<Tlv> = Vec::new();

    loop {
        if (command_index >= message.len()) {
            break;
        }
        // If message starts with b'\xFF' then the command type is in the second byte.
        if message[0] == 0xFF {
            command_index += 1;

        } else if message[0] == 0x00 && message[1] == 0x15 {
            command_index += 11;
        } else {
            command_index += 1;
        }

        (next_index, vv) = assign_tlvs(message[command_index + 1..].to_vec());
        if next_index == 0 {
            next_index = message.len() - 1;
        }
        match message[command_index] {
            x if x <= COMMANDS::DISCOVERY_RESPONSE.value() && x >= COMMANDS::LINK_REQUEST.value() => 
            commands.append(&mut vec![Command {
                command: x.into(),
                tlvs: vv,
                payload: message[command_index-1.. next_index].to_vec(),
            }]),
                _ => {
                    commands.append(&mut vec![Command {
                        command: COMMANDS::UNKNOWN_COMMAND,
                        tlvs: vv,
                        payload: message[command_index-1.. next_index].to_vec(),
                    }]);
                }
        }

        command_index += next_index + 1;
    }
    
    return commands;
}   
    
