//! Vendor-specific HCI commands and types needed for those commands.

extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};

use crate::Controller;

/// Vendor-specific HCI commands.
pub trait HalCommands {
    /// This command is intended to retrieve the firmware revision number.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a
    /// [command complete](crate::event::command::CommandComplete) event.
    async fn get_firmware_revision(&mut self);

    /// This command writes a value to a low level configure data structure. It is useful to setup
    /// directly some low level parameters for the system in the runtime.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn write_config_data(&mut self, config: &ConfigData);

    /// This command requests the value in the low level configure data structure.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn read_config_data(&mut self, param: ConfigParameter);

    /// This command sets the TX power level of the BlueNRG-MS.
    ///
    /// When the system starts up or reboots, the default TX power level will be used, which is the
    /// maximum value of [8 dBm](PowerLevel::Dbm8_0). Once this command is given, the output power
    /// will be changed instantly, regardless if there is Bluetooth communication going on or
    /// not. For example, for debugging purpose, the BlueNRG-MS can be set to advertise all the
    /// time. And use this command to observe the signal strength changing.
    ///
    /// The system will keep the last received TX power level from the command, i.e. the 2nd
    /// command overwrites the previous TX power level. The new TX power level remains until
    /// another Set TX Power command, or the system reboots.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn set_tx_power_level(&mut self, level: PowerLevel);

    /// Retrieve the number of packets sent in the last TX direct test.
    ///
    /// During the Direct Test mode, in the TX tests, the number of packets sent in the test is not
    /// returned when executing the Direct Test End command. This command implements this feature.
    ///
    /// If the Direct TX test is started, a 16-bit counter will be used to count how many packets
    /// have been transmitted. After the Direct Test End, this command can be used to check how many
    /// packets were sent during the Direct TX test.
    ///
    /// The counter starts from 0 and counts upwards. As would be the case if 16-bits are all used,
    /// the counter wraps back and starts from 0 again. The counter is not cleared until the next
    /// Direct TX test starts.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn get_tx_test_packet_count(&mut self);

    /// This command starts a carrier frequency, i.e. a tone, on a specific channel.
    ///
    /// The frequency sine wave at the specific channel may be used for debugging purpose only. The
    /// channel ID is a parameter from 0 to 39 for the 40 BLE channels, e.g. 0 for 2.402 GHz, 1 for
    /// 2.404 GHz etc.
    ///
    /// This command should not be used when normal Bluetooth activities are ongoing.
    /// The tone should be stopped by [`stop_tone`](HalCommands::stop_tone) command.
    ///
    /// # Errors
    ///
    /// - [InvalidChannel](Error::InvalidChannel) if the channel is greater than 39.
    /// - Underlying communication errors
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn start_tone(&mut self, channel: u8, freq_offset: u8) -> Result<(), Error>;

    /// Stops the previously started by the [`start_tone`](HalCommands::start_tone) command.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn stop_tone(&mut self);

    /// This command is intended to return the Link Layer Status and Connection Handles.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn get_link_status(&mut self);

    /// This command sets the bitmask associated to
    /// [End of Radio Activity](crate::vendor::event::VendorEvent::EndOfRadioActivity) event.
    ///
    /// Only the radio activities enabled in the mask will be reported to the application by the
    /// [End of Radio Activity](crate::vendor::event::VendorEvent::EndOfRadioActivity) event.
    async fn set_radio_activity_mask(&mut self, mask: RadioActivityFlags);

    /// This command is intended to retrieve information about the current Anchor Interval and
    /// allocable timing slots.
    ///
    /// # Errors
    ///
    /// Only underlying communication errors are reported.
    ///
    /// # Generated events
    ///
    /// The controller will generate a [command complete](crate::event::command::CommandComplete) event.
    async fn get_anchor_period(&mut self);

    /// This command is used to enable/disable the generation of HAL events.
    ///
    /// If the bit in the [HAL Event Mask](HalEventFlags) is set to one, then the event associated with
    /// that will be enabled.
    async fn set_event_mask(&mut self, mask: HalEventFlags);

    /// This command is used to retreive Tx, Rx, and total buffer count allocated for ACL packets.
    async fn get_pm_debug_info(&mut self);

    /// This command is used to disable/enable the Peripheral latencyy feature during a connection.
    ///
    /// Note that, by default, the Peripheral latency is enabled at connection time.
    async fn set_peripheral_latency(&mut self, enabled: bool);

    /// This command returns the value of the RSSI.
    async fn read_rssi(&mut self);

    /// This command reads a register value from the RF module
    async fn read_radio_reg(&mut self, address: u8);

    /// This command returns the raw value of the RSSI
    async fn read_raw_rssi(&mut self);

    /// This command does set up the RF to listen to a specific RF Channel.
    ///
    /// `rf_channel`: BLE Channel Id, from 0x00 to 0x27 meaning `(2.402 + 0.002 * 0xXX) GHz`.
    /// The device will continously emit 0s, meaning that the tone will be at the channel center
    /// frequency minus the maximum frequency deviation (250 KHz).
    async fn rx_start(&mut self, rf_channel: u8);

    /// This command stops a previous [HAL Rx Start](HalCommands::rx_start) command
    async fn rx_stop(&mut self);

    /// This command is equivalent to [HCI Reset](crate::host::HostHci::reset) but ensures
    /// the sleep mode is entered immediately after its completion.
    async fn stack_reset(&mut self);
}

impl<T: Controller> HalCommands for T {
    async fn get_firmware_revision(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_GET_FIRMWARE_REVISION, &[])
            .await
    }

    impl_variable_length_params!(
        write_config_data,
        ConfigData,
        crate::vendor::opcode::HAL_WRITE_CONFIG_DATA
    );

    async fn read_config_data(&mut self, param: ConfigParameter) {
        self.controller_write(crate::vendor::opcode::HAL_READ_CONFIG_DATA, &[param as u8])
            .await
    }

    async fn set_tx_power_level(&mut self, level: PowerLevel) {
        // Byte 0: enable high power mode - deprecated and ignored on STM32WB
        // Byte 1: PA level
        let mut bytes = [0; 2];
        bytes[1] = level as u8;

        self.controller_write(crate::vendor::opcode::HAL_SET_TX_POWER_LEVEL, &bytes)
            .await
    }

    async fn get_tx_test_packet_count(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_TX_TEST_PACKET_COUNT, &[])
            .await
    }

    async fn start_tone(&mut self, channel: u8, freq_offset: u8) -> Result<(), Error> {
        const MAX_CHANNEL: u8 = 39;
        if channel > MAX_CHANNEL {
            return Err(Error::InvalidChannel(channel));
        }

        self.controller_write(
            crate::vendor::opcode::HAL_START_TONE,
            &[channel, freq_offset],
        )
        .await;

        Ok(())
    }

    async fn stop_tone(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_STOP_TONE, &[])
            .await
    }

    async fn get_link_status(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_GET_LINK_STATUS, &[])
            .await
    }

    async fn get_anchor_period(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_GET_ANCHOR_PERIOD, &[])
            .await
    }

    async fn set_radio_activity_mask(&mut self, mask: RadioActivityFlags) {
        let mut payload = [0; 2];
        LittleEndian::write_u16(&mut payload, mask.bits());
        self.controller_write(crate::vendor::opcode::HAL_SET_RADIO_ACTIVITY_MASK, &payload)
            .await;
    }

    async fn set_event_mask(&mut self, mask: HalEventFlags) {
        let mut payload = [0; 4];
        LittleEndian::write_u32(&mut payload, mask.bits());
        self.controller_write(crate::vendor::opcode::HAL_SET_EVENT_MASK, &payload)
            .await;
    }

    async fn get_pm_debug_info(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_GET_PM_DEBUG_INFO, &[])
            .await;
    }

    async fn set_peripheral_latency(&mut self, enabled: bool) {
        self.controller_write(
            crate::vendor::opcode::HAL_SET_PERIPHERAL_LATENCY,
            &[enabled as u8],
        )
        .await;
    }

    async fn read_rssi(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_READ_RSSI, &[])
            .await;
    }

    async fn read_radio_reg(&mut self, address: u8) {
        self.controller_write(crate::vendor::opcode::HAL_READ_RADIO_REG, &[address])
            .await;
    }

    async fn read_raw_rssi(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_READ_RAW_RSSI, &[])
            .await;
    }

    async fn rx_start(&mut self, rf_channel: u8) {
        self.controller_write(crate::vendor::opcode::HAL_RX_START, &[rf_channel])
            .await;
    }

    async fn rx_stop(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_RX_STOP, &[])
            .await;
    }

    async fn stack_reset(&mut self) {
        self.controller_write(crate::vendor::opcode::HAL_STACK_RESET, &[])
            .await;
    }
}

/// Potential errors from parameter validation.
///
/// Before some commands are sent to the controller, the parameters are validated. This type
/// enumerates the potential validation errors. Must be specialized on the types of communication
/// errors.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// For the [Start Tone](HalCommands::start_tone) command, the channel was greater than the maximum
    /// allowed channel (39). The invalid channel is returned.
    InvalidChannel(u8),
}

/// Low-level configuration parameters for the controller.
pub struct ConfigData {
    /// Offset of the element in the configuration data structure which has to be written.
    ///
    /// Values:
    ///- 0x00: CONFIG_DATA_PUBADDR_OFFSET;
    ///  Bluetooth public address; 6 bytes
    ///- 0x08: CONFIG_DATA_ER_OFFSET;
    ///  Encryption root key used to derive LTK (legacy) and CSRK; 16 bytes
    ///- 0x18: CONFIG_DATA_IR_OFFSET;
    ///  Identity root key used to derive DHK (legacy) and IRK; 16 bytes
    ///- 0x2E: CONFIG_DATA_RANDOM_ADDRESS_OFFSET;
    ///  Static Random Address; 6 bytes
    ///- 0x34: CONFIG_DATA_GAP_ADD_REC_NBR_OFFSET;
    ///  GAP service additional record number; 1 byte
    ///- 0x35: CONFIG_DATA_SC_KEY_TYPE_OFFSET;
    ///  Secure Connection key type (0: "normal", 1: "debug"); 1 byte
    ///- 0xB0: CONFIG_DATA_SMP_MODE_OFFSET;
    ///  SMP mode (0: "normal", 1: "bypass", 2: "no blacklist"); 1 byte
    ///- 0xC0: CONFIG_DATA_LL_SCAN_CHAN_MAP_OFFSET (only for STM32WB);
    ///  LL scan channel map (same format as Primary_Adv_Channel_Map); 1
    ///  byte
    ///- 0xC1: CONFIG_DATA_LL_BG_SCAN_MODE_OFFSET (only for STM32WB);
    ///  LL background scan mode (0: "BG scan disabled", 1: "BG scan
    ///  enabled"); 1 byte
    offset: u8,
    /// Length of the value to be written
    length: u8,
    /// Data to be written
    value_buf: [u8; ConfigData::MAX_LENGTH],
}

impl ConfigData {
    /// Maximum length needed to serialize the data.
    pub const MAX_LENGTH: usize = 0x2E;

    /// Serializes the data into the given buffer.
    ///
    /// Returns the number of valid bytes in the buffer.
    ///
    /// # Panics
    ///
    /// The buffer must be large enough to support the serialized data (at least
    /// [`MAX_LENGTH`](ConfigData::MAX_LENGTH) bytes).
    pub fn copy_into_slice(&self, bytes: &mut [u8]) -> usize {
        bytes[0] = self.offset;
        bytes[1] = self.length;

        let len = self.length as usize;
        bytes[2..2 + len].copy_from_slice(&self.value_buf[..len]);

        2 + len
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn public_address(addr: crate::BdAddr) -> ConfigDataDiversifierBuilder {
        let mut data = Self {
            offset: 0,
            length: 6,
            value_buf: [0; Self::MAX_LENGTH],
        };

        data.value_buf[0..6].copy_from_slice(&addr.0);

        ConfigDataDiversifierBuilder { data }
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn random_address(addr: crate::BdAddr) -> ConfigDataDiversifierBuilder {
        let mut data = Self {
            offset: 0x2E,
            length: 6,
            value_buf: [0; Self::MAX_LENGTH],
        };

        data.value_buf[0..6].copy_from_slice(&addr.0);

        ConfigDataDiversifierBuilder { data }
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn diversifier(d: u16) -> ConfigDataEncryptionRootBuilder {
        let mut data = Self {
            offset: 6,
            length: 2,
            value_buf: [0; Self::MAX_LENGTH],
        };
        LittleEndian::write_u16(&mut data.value_buf[0..2], d);

        ConfigDataEncryptionRootBuilder { data }
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn encryption_root(key: &crate::host::EncryptionKey) -> ConfigDataIdentityRootBuilder {
        let mut data = Self {
            offset: 8,
            length: 16,
            value_buf: [0; Self::MAX_LENGTH],
        };
        data.value_buf[0..16].copy_from_slice(&key.0);

        ConfigDataIdentityRootBuilder { data }
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn identity_root(key: &crate::host::EncryptionKey) -> ConfigDataLinkLayerOnlyBuilder {
        let mut data = Self {
            offset: 24,
            length: 16,
            value_buf: [0; Self::MAX_LENGTH],
        };
        data.value_buf[0..16].copy_from_slice(&key.0);
        ConfigDataLinkLayerOnlyBuilder { data }
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn link_layer_only(ll_only: bool) -> ConfigDataRoleBuilder {
        let mut data = Self {
            offset: 40,
            length: 1,
            value_buf: [0; Self::MAX_LENGTH],
        };
        data.value_buf[0] = ll_only as u8;
        ConfigDataRoleBuilder { data }
    }

    /// Builder for [ConfigData].
    ///
    /// The controller allows us to write any _contiguous_ portion of the [ConfigData] structure in
    /// [`write_config_data`](HalCommands::write_config_data).  The builder associated functions allow
    /// us to start with any field, and the returned builder allows only either chaining the next
    /// field or building the structure to write.
    pub fn role(role: Role) -> ConfigDataCompleteBuilder {
        let mut data = Self {
            offset: 41,
            length: 1,
            value_buf: [0; Self::MAX_LENGTH],
        };
        data.value_buf[0] = role as u8;
        ConfigDataCompleteBuilder { data }
    }
}

/// Builder for [`ConfigData`].
pub struct ConfigDataDiversifierBuilder {
    data: ConfigData,
}

impl ConfigDataDiversifierBuilder {
    /// Specify the diversifier and continue building.
    pub fn diversifier(mut self, d: u16) -> ConfigDataEncryptionRootBuilder {
        let len = self.data.length as usize;
        LittleEndian::write_u16(&mut self.data.value_buf[len..2 + len], d);
        self.data.length += 2;

        ConfigDataEncryptionRootBuilder { data: self.data }
    }

    /// Build the [ConfigData] as-is. It includes only the public address.
    pub fn build(self) -> ConfigData {
        self.data
    }
}

/// Builder for [`ConfigData`].
pub struct ConfigDataEncryptionRootBuilder {
    data: ConfigData,
}

impl ConfigDataEncryptionRootBuilder {
    /// Specify the encryption root and continue building.
    pub fn encryption_root(
        mut self,
        key: &crate::host::EncryptionKey,
    ) -> ConfigDataIdentityRootBuilder {
        let len = self.data.length as usize;
        self.data.value_buf[len..16 + len].copy_from_slice(&key.0);
        self.data.length += 16;

        ConfigDataIdentityRootBuilder { data: self.data }
    }

    /// Build the [ConfigData] as-is. It includes the diversifier, and may include fields before it,
    /// but does not include any fields after it (including the encryption root).
    pub fn build(self) -> ConfigData {
        self.data
    }
}

/// Builder for [`ConfigData`].
pub struct ConfigDataIdentityRootBuilder {
    data: ConfigData,
}

impl ConfigDataIdentityRootBuilder {
    /// Specify the identity root and continue building.
    pub fn identity_root(
        mut self,
        key: &crate::host::EncryptionKey,
    ) -> ConfigDataLinkLayerOnlyBuilder {
        let len = self.data.length as usize;
        self.data.value_buf[len..16 + len].copy_from_slice(&key.0);
        self.data.length += 16;

        ConfigDataLinkLayerOnlyBuilder { data: self.data }
    }

    /// Build the [ConfigData] as-is. It includes the encryption root, and may include fields before
    /// it, but does not include any fields after it (including the identity root).
    pub fn build(self) -> ConfigData {
        self.data
    }
}

/// Builder for [`ConfigData`].
pub struct ConfigDataLinkLayerOnlyBuilder {
    data: ConfigData,
}

impl ConfigDataLinkLayerOnlyBuilder {
    /// Specify whether to use the link layer only and continue building.
    pub fn link_layer_only(mut self, ll_only: bool) -> ConfigDataRoleBuilder {
        self.data.value_buf[self.data.length as usize] = ll_only as u8;
        self.data.length += 1;
        ConfigDataRoleBuilder { data: self.data }
    }

    /// Build the [ConfigData] as-is. It includes the identity root, and may include fields before
    /// it, but does not include any fields after it (including the link layer only flag).
    pub fn build(self) -> ConfigData {
        self.data
    }
}

/// Builder for [`ConfigData`].
pub struct ConfigDataRoleBuilder {
    data: ConfigData,
}

impl ConfigDataRoleBuilder {
    /// Specify the device role and continue building.
    pub fn role(mut self, role: Role) -> ConfigDataCompleteBuilder {
        self.data.value_buf[self.data.length as usize] = role as u8;
        self.data.length += 1;
        ConfigDataCompleteBuilder { data: self.data }
    }

    /// Build the [ConfigData] as-is. It includes the link layer only flag, and may include fields
    /// before it, but does not include any fields after it (including the role).
    pub fn build(self) -> ConfigData {
        self.data
    }
}

/// Builder for [`ConfigData`].
pub struct ConfigDataCompleteBuilder {
    data: ConfigData,
}

impl ConfigDataCompleteBuilder {
    /// Build the [ConfigData] as-is. It includes the role field, and may include fields before it.
    pub fn build(self) -> ConfigData {
        self.data
    }
}

/// Roles that the server can adopt.
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Role {
    /// Peripheral and primary device.
    /// - Only one connection.
    /// - 6 KB of RAM retention.
    Peripheral6Kb = 1,

    /// Peripheral and primary device.
    /// - Only one connection.
    /// - 12 KB of RAM retention.
    Peripheral12Kb = 2,

    /// Primary device and peripheral
    /// - Up to 8 connections
    /// - 12 KB of RAM retention
    Primary12Kb = 3,

    /// Primary device and peripheral.
    /// - Simultaneous advertising and scanning
    /// - Up to 4 connections
    /// - This mode is available starting from BlueNRG-MS FW stack version 7.1.b
    SimultaneousAdvertisingScanning = 4,
}

/// Configuration parameters that are readable by the
/// [`read_config_data`](HalCommands::read_config_data) command.
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigParameter {
    /// Bluetooth public address.
    PublicAddress = 0,

    /// Bluetooth random address.
    RandomAddress = 0x2E,

    /// Diversifier used to derive CSRK (connection signature resolving key).
    Diversifier = 6,

    /// Encryption root key used to derive the LTK (long-term key) and CSRK (connection signature
    /// resolving key).
    EncryptionRoot = 8,

    /// Identity root key used to derive the LTK (long-term key) and CSRK (connection signature
    /// resolving key).
    IdentityRoot = 24,

    /// Switch on/off Link Layer only mode.
    LinkLayerOnly = 40,

    /// BlueNRG-MS roles and mode configuration.
    Role = 41,
}

/// Transmitter power levels available for the system.
///
/// STM32WB5x uses single byte parameter for PA level.
#[derive(Clone, Copy)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerLevel {
    /// -40 dBm.
    Minus40dBm = 0x00,

    /// -20.85 dBm.
    Minus20_85dBm = 0x01,

    /// -19.75 dBm.
    Minus19_75dBm = 0x02,

    /// -18.85 dBm.
    Minus18_85dBm = 0x03,

    /// 17.6 dBm.
    Minus17_6dBm = 0x04,

    /// -16.5 dBm.
    Minus16_5dBm = 0x05,

    /// -15.25 dBm.
    Minus15_25dBm = 0x06,

    /// -14.1 dBm.
    Minus14_1dBm = 0x07,

    /// -13.15 dBm.
    Minus13_15dBm = 0x08,

    /// -12.05 dBm.
    Minus12_05dBm = 0x09,

    /// -10.9 dBm.
    Minus10_9dBm = 0x0A,

    /// -9.9 dBm.
    Minus9_9dBm = 0x0B,

    /// -8.85 dBm.
    Minus8_85dBm = 0x0C,

    /// -7.8 dBm.
    Minus7_8dBm = 0x0D,

    /// -6.9 dBm.
    Minus6_9dBm = 0x0E,

    /// -5.9 dBm.
    Minus5_9dBm = 0x0F,

    /// -4.95 dBm.
    Minus4_95dBm = 0x10,

    /// -4 dBm.
    Minus4dBm = 0x11,

    /// -3.15 dBm.
    Minus3_15dBm = 0x12,

    /// -2.45 dBm.
    Minus2_45dBm = 0x13,

    /// -1.8 dBm.
    Minus1_8dBm = 0x14,

    /// -1.3 dBm.
    Minus1_3dBm = 0x15,

    /// -0.85 dBm.
    Minus0_85dBm = 0x16,

    /// -0.5 dBm.
    Minus0_5dBm = 0x17,

    /// -0.15 dBm.
    Minus0_15dBm = 0x18,

    /// 0 dBm.
    ZerodBm = 0x19,

    /// 1 dBm.
    Plus1dBm = 0x1A,

    /// 2 dBm.
    Plus2dBm = 0x1B,

    /// 3 dBm.
    Plus3dBm = 0x1C,

    /// 4 dBm.
    Plus4dBm = 0x1D,

    /// 5 dBm.
    Plus5dBm = 0x1E,

    /// 6 dBm.
    Plus6dBm = 0x1F,
}

#[cfg(not(feature = "defmt"))]
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct RadioActivityFlags: u16 {
        /// Idle
        const IDLE = 0x0001;
        /// Advertising
        const ADVERTISING = 0x0002;
        /// Peripheral connection
        const PERIPHERAL_CONN = 0x0004;
        /// Scanning
        const SCANNING = 0x0008;
        /// Central connection
        const CENTRAL_CONN = 0x0020;
        /// Tx test mode
        const TX_TEST = 0x0040;
        /// Rx test mode
        const RX_TEST = 0x0080;
    }
}

#[cfg(feature = "defmt")]
defmt::bitflags! {
    pub struct RadioActivityFlags: u16 {
        /// Idle
        const IDLE = 0x0001;
        /// Advertising
        const ADVERTISING = 0x0002;
        /// Peripheral connection
        const PERIPHERAL_CONN = 0x0004;
        /// Scanning
        const SCANNING = 0x0008;
        /// Central connection
        const CENTRAL_CONN = 0x0020;
        /// Tx test mode
        const TX_TEST = 0x0040;
        /// Rx test mode
        const RX_TEST = 0x0080;
    }
}

#[cfg(not(feature = "defmt"))]
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct HalEventFlags: u32 {
        /// [HAL Scan Request Report](crate::vendor::event::VendorEvent::HalScanReqReport) event
        const SCAN_REQ_REPORT = 0x00000001;
    }
}

#[cfg(feature = "defmt")]
defmt::bitflags! {
    pub struct HalEventFlags: u32 {
        /// [HAL Scan Request Report](crate::vendor::event::VendorEvent::HalScanReqReport) event
        const SCAN_REQ_REPORT = 0x00000001;
    }
}
