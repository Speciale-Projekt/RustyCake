use std::borrow::{BorrowMut, Borrow};


#[derive(Debug)]
pub(crate) enum COMMANDS {
    LinkRequest = 0x00,
    LinkAccept = 0x01,
    LinkAcceptAndRequest = 0x02,
    LinkReject = 0x03,
    Advertisement = 0x04,
    Update = 0x05,
    UpdateRequest = 0x06,
    DataRequest = 0x07,
    DataResponse = 0x08,
    ParentRequest = 0x09,
    ParentResponse = 0x0A,
    ChildIdRequest = 0x0B,
    ChildIdResponse = 0x0C,
    ChildUpdateRequest = 0x0D,
    ChildUpdateResponse = 0x0E,
    Announce = 0x0F,
    DiscoveryRequest = 0x10,
    DiscoveryResponse = 0x11,
    UnknownCommand = 0xFF
}
impl COMMANDS {
    fn value(&self) -> u8 {
        match *self {
            COMMANDS::LinkRequest => 0x00,
            COMMANDS::LinkAccept => 0x01,
            COMMANDS::LinkAcceptAndRequest => 0x02,
            COMMANDS::LinkReject => 0x03,
            COMMANDS::Advertisement => 0x04,
            COMMANDS::Update => 0x05,
            COMMANDS::UpdateRequest => 0x06,
            COMMANDS::DataRequest => 0x07,
            COMMANDS::DataResponse => 0x08,
            COMMANDS::ParentRequest => 0x09,
            COMMANDS::ParentResponse => 0x0A,
            COMMANDS::ChildIdRequest => 0x0B,
            COMMANDS::ChildIdResponse => 0x0C,
            COMMANDS::ChildUpdateRequest => 0x0D,
            COMMANDS::ChildUpdateResponse => 0x0E,
            COMMANDS::Announce => 0x0F,
            COMMANDS::DiscoveryRequest => 0x10,
            COMMANDS::DiscoveryResponse => 0x11,
            COMMANDS::UnknownCommand => 0xFF
        }
    }
}
impl From<u8> for COMMANDS {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => COMMANDS::LinkRequest,
            0x01 => COMMANDS::LinkAccept,
            0x02 => COMMANDS::LinkAcceptAndRequest,
            0x03 => COMMANDS::LinkReject,
            0x04 => COMMANDS::Advertisement,
            0x05 => COMMANDS::Update,
            0x06 => COMMANDS::UpdateRequest,
            0x07 => COMMANDS::DataRequest,
            0x08 => COMMANDS::DataResponse,
            0x09 => COMMANDS::ParentRequest,
            0x0A => COMMANDS::ParentResponse,
            0x0B => COMMANDS::ChildIdRequest,
            0x0C => COMMANDS::ChildIdResponse,
            0x0D => COMMANDS::ChildUpdateRequest,
            0x0E => COMMANDS::ChildUpdateResponse,
            0x0F => COMMANDS::Announce,
            0x10 => COMMANDS::DiscoveryRequest,
            0x11 => COMMANDS::DiscoveryResponse,
            _ => COMMANDS::UnknownCommand
        }
    }
}

#[derive(Debug)]
pub(crate) enum TLVS {
    SourceAddressTLV = 0x00,
    ModeTLV = 0x01,
    TimeoutTLV = 0x02,
    ChallengeTLV = 0x03,
    ResponseTLV = 0x04,
    LinkLayerFrameCounterTLV = 0x05,
    LinkQualityTLV = 0x06,
    NetworkParameterTLV = 0x07,
    MLEFrameCounterTLV = 0x08,
    Route64TLV = 0x09,
    Address16TLV = 0x0A,
    LeaderDataTLV = 0x0B,
    NetworkDataTLV = 0x0C,
    TLVRequestTLV = 0x0D,
    ScanMaskTLV = 0x0E,
    ConnectivityTLV = 0x0F,
    LinkMarginTLV = 0x10,
    StatusTLV = 0x11,
    VersionTLV = 0x12,
    AddressRegistrationTLV = 0x13,
    ChannelTLV = 0x14,
    PanIDTLV = 0x15,
    ActiveTimestampTLV = 0x16,
    PendingTimestampTLV = 0x17,
    ActiveOperationalDatasetTLV = 0x18,
    PendingOperationalDatasetTLV = 0x19,
    ThreadDiscoveryTLV = 0x1A,
    UnknownTLV = 0xFF
}
impl TLVS {
    
    fn value(&self) -> u8 {
        match *self {
            TLVS::SourceAddressTLV => 0x00,
            TLVS::ModeTLV => 0x01,
            TLVS::TimeoutTLV => 0x02,
            TLVS::ChallengeTLV => 0x03,
            TLVS::ResponseTLV => 0x04,
            TLVS::LinkLayerFrameCounterTLV => 0x05,
            TLVS::LinkQualityTLV => 0x06,
            TLVS::NetworkParameterTLV => 0x07,
            TLVS::MLEFrameCounterTLV => 0x08,
            TLVS::Route64TLV => 0x09,
            TLVS::Address16TLV => 0x0A,
            TLVS::LeaderDataTLV => 0x0B,
            TLVS::NetworkDataTLV => 0x0C,
            TLVS::TLVRequestTLV => 0x0D,
            TLVS::ScanMaskTLV => 0x0E,
            TLVS::ConnectivityTLV => 0x0F,
            TLVS::LinkMarginTLV => 0x10,
            TLVS::StatusTLV => 0x11,
            TLVS::VersionTLV => 0x12,
            TLVS::AddressRegistrationTLV => 0x13,
            TLVS::ChannelTLV => 0x14,
            TLVS::PanIDTLV => 0x15,
            TLVS::ActiveTimestampTLV => 0x16,
            TLVS::PendingTimestampTLV => 0x17,
            TLVS::ActiveOperationalDatasetTLV => 0x18,
            TLVS::PendingOperationalDatasetTLV => 0x19,
            TLVS::ThreadDiscoveryTLV => 0x1A,
            TLVS::UnknownTLV => 0xFF
        }
    }
}

impl From<u8> for TLVS {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => TLVS::SourceAddressTLV,
            0x01 => TLVS::ModeTLV,
            0x02 => TLVS::TimeoutTLV,
            0x03 => TLVS::ChallengeTLV,
            0x04 => TLVS::ResponseTLV,
            0x05 => TLVS::LinkLayerFrameCounterTLV,
            0x06 => TLVS::LinkQualityTLV,
            0x07 => TLVS::NetworkParameterTLV,
            0x08 => TLVS::MLEFrameCounterTLV,
            0x09 => TLVS::Route64TLV,
            0x0A => TLVS::Address16TLV,
            0x0B => TLVS::LeaderDataTLV,
            0x0C => TLVS::NetworkDataTLV,
            0x0D => TLVS::TLVRequestTLV,
            0x0E => TLVS::ScanMaskTLV,
            0x0F => TLVS::ConnectivityTLV,
            0x10 => TLVS::LinkMarginTLV,
            0x11 => TLVS::StatusTLV,
            0x12 => TLVS::VersionTLV,
            0x13 => TLVS::AddressRegistrationTLV,
            0x14 => TLVS::ChannelTLV,
            0x15 => TLVS::PanIDTLV,
            0x16 => TLVS::ActiveTimestampTLV,
            0x17 => TLVS::PendingTimestampTLV,
            0x18 => TLVS::ActiveOperationalDatasetTLV,
            0x19 => TLVS::PendingOperationalDatasetTLV,
            0x1A => TLVS::ThreadDiscoveryTLV,
            _ => TLVS::UnknownTLV
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
            x if x >= TLVS::SourceAddressTLV.value() as u8 && x <= TLVS::ThreadDiscoveryTLV.value() as u8 => {
               tlvs.push(Tlv {
                    tlv_type: x.into(),
                    length: tlv_length,
                    value: msg[2..tlv_length + 2].to_vec(),
                });
            }
            _ => {
                tlvs.push (Tlv {
                    tlv_type: TLVS::UnknownTLV,
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
            x if x <= COMMANDS::DiscoveryResponse.value() && x >= COMMANDS::LinkRequest.value() => 
            commands.append(&mut vec![Command {
                command: x.into(),
                tlvs: vv,
                payload: message[command_index-1.. next_index].to_vec(),
            }]),
                _ => {
                    commands.append(&mut vec![Command {
                        command: COMMANDS::UnknownCommand,
                        tlvs: vv,
                        payload: message[command_index-1.. next_index].to_vec(),
                    }]);
                }
        }

        command_index += next_index + 1;
    }
    
    return commands;
}   
    
