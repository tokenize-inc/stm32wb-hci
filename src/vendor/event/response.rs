//! Return parameters for vendor-specific commands.
//!
//! This module defines the parameters returned in the Command Complete event for vendor-specific
//! commands.  These commands are defined for the BlueNRG controller, but are not standard HCI
//! commands.

use crate::{require_len, require_len_at_least};
use byteorder::{ByteOrder, LittleEndian};
use core::convert::{TryFrom, TryInto};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::time::Duration;

use super::AttributeHandle;

/// Vendor-specific commands that may generate the
/// [Command Complete](crate::event::command::ReturnParameters::Vendor) event. If the commands have defined
/// return parameters, they are included in the enum.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VendorReturnParameters {
    /// Parameters returned by the
    /// [HAL Get Firmware Revision](crate::vendor::command::hal::HalCommands::get_firmware_revision) command.
    HalGetFirmwareRevision(HalFirmwareRevision),

    /// Status returned by the [HAL Write Config Data](crate::vendor::command::hal::HalCommands::write_config_data)
    /// command.
    HalWriteConfigData(crate::Status),

    /// Parameters returned by the [HAL Read Config Data](crate::vendor::command::hal::HalCommands::read_config_data)
    /// command.
    HalReadConfigData(HalConfigData),

    /// Status returned by the [HAL Set Tx Power Level](crate::vendor::command::hal::HalCommands::set_tx_power_level)
    /// command.
    HalSetTxPowerLevel(crate::Status),

    /// Status returned by the
    /// [HAL Device Standby](crate::vendor::command::hal::HalCommands::device_standby) command.
    HalDeviceStandby(crate::Status),

    /// Parameters returned by the
    /// [HAL Get Tx Test Packet Count](crate::vendor::command::hal::HalCommands::get_tx_test_packet_count) command.
    HalGetTxTestPacketCount(HalTxTestPacketCount),

    /// Status returned by the [HAL Start Tone](crate::vendor::command::hal::HalCommands::start_tone) command.
    HalStartTone(crate::Status),

    /// Status returned by the [HAL Stop Tone](crate::vendor::command::hal::HalCommands::stop_tone) command.
    HalStopTone(crate::Status),

    /// Status returned by the [HAL Get Link Status](crate::vendor::command::hal::HalCommands::get_link_status) command.
    HalGetLinkStatus(HalLinkStatus),

    /// Parameters returned by the [HAL Get Anchor Period](crate::vendor::command::hal::HalCommands::get_anchor_period)
    /// command.
    HalGetAnchorPeriod(HalAnchorPeriod),

    /// Parameters returned by the [HAL Get PM Debug Info](crate::vendor::command::hal::HalCommands::get_pm_debug_info)
    /// command.
    HalGetPmDebugInfo(HalPmDebugInfo),

    /// Parameters returned by the [HAL Read RSSI](crate::vendor::command::hal::HalCommands::read_rssi)
    /// command.
    HalReadRssi(u8),

    /// Parameters returned by the [HAL Read Radio Register](crate::vendor::command::hal::HalCommands::read_radio_reg)
    /// command.
    HalReadRadioReg(u8),

    /// Parameters returned by the [HAL Read Raw RSSI](crate::vendor::command::hal::HalCommands::read_raw_rssi)
    /// command.
    HalReadRawRssi(u8),

    /// Status returned by the
    /// [GAP Set Non-Discoverable](crate::vendor::command::gap::GapCommands::gap_set_nondiscoverable)
    /// command.
    GapSetNonDiscoverable(crate::Status),

    /// Status returned by the
    /// [GAP Set Discoverable](crate::vendor::command::gap::GapCommands::set_discoverable)
    /// command.
    GapSetDiscoverable(crate::Status),

    /// Status returned by the
    /// [GAP Set Direct Connectable](crate::vendor::command::gap::GapCommands::set_direct_connectable) command.
    GapSetDirectConnectable(crate::Status),

    /// Status returned by the [GAP Set IO Capability](crate::vendor::command::gap::GapCommands::set_io_capability)
    /// command.
    GapSetIoCapability(crate::Status),

    /// Status returned by the
    /// [GAP Set Authentication Requirement](crate::vendor::command::gap::GapCommands::set_authentication_requirement) command.
    GapSetAuthenticationRequirement(crate::Status),

    /// Status returned by the
    /// [GAP Set Authorization Requirement](crate::vendor::command::gap::GapCommands::set_authorization_requirement) command.
    GapSetAuthorizationRequirement(crate::Status),

    /// Status returned by the
    /// [GAP Pass Key Response](crate::vendor::command::gap::GapCommands::pass_key_response)
    /// command.
    GapPassKeyResponse(crate::Status),

    /// Status returned by the
    /// [GAP Authorization Response](crate::vendor::command::gap::GapCommands::authorization_response) command.
    GapAuthorizationResponse(crate::Status),

    /// Parameters returned by the [GAP Init](crate::vendor::command::gap::GapCommands::init) command.
    GapInit(GapInit),

    /// Parameters returned by the
    /// [GAP Set Non-Connectable](crate::vendor::command::gap::GapCommands::set_nonconnectable) command.
    GapSetNonConnectable(crate::Status),

    /// Parameters returned by the
    /// [GAP Set Undirected Connectable](crate::vendor::command::gap::GapCommands::set_undirected_connectable) command.
    GapSetUndirectedConnectable(crate::Status),

    /// Parameters returned by the
    /// [GAP Update Advertising Data](crate::vendor::command::gap::GapCommands::update_advertising_data) command.
    GapUpdateAdvertisingData(crate::Status),

    /// Parameters returned by the
    /// [GAP Delete AD Type](crate::vendor::command::gap::GapCommands::delete_ad_type)
    /// command.
    GapDeleteAdType(crate::Status),

    /// Parameters returned by the
    /// [GAP Get Security Level](crate::vendor::command::gap::GapCommands::get_security_level) command.
    GapGetSecurityLevel(GapSecurityLevel),

    /// Parameters returned by the
    /// [GAP Set Event Mask](crate::vendor::command::gap::GapCommands::set_event_mask)
    /// command.
    GapSetEventMask(crate::Status),

    /// Parameters returned by the
    /// [GAP Configure White List](crate::vendor::command::gap::GapCommands::configure_white_list) command.
    GapConfigureWhiteList(crate::Status),

    /// Parameters returned by the
    /// [GAP Clear Security Database](crate::vendor::command::gap::GapCommands::clear_security_database) command.
    GapClearSecurityDatabase(crate::Status),

    /// Parameters returned by the
    /// [GAP Allow Rebond](crate::vendor::command::gap::GapCommands::allow_rebond) command.
    GapAllowRebond(crate::Status),

    /// Parameters returned by the
    /// [GAP Terminate Procedure](crate::vendor::command::gap::GapCommands::terminate_gap_procedure) command.
    GapTerminateProcedure(crate::Status),

    /// Parameters returned by the
    /// [GAP Resolve Private Address](crate::vendor::command::gap::GapCommands::resolve_private_address) command.
    GapResolvePrivateAddress(GapResolvePrivateAddress),

    /// Parameters returned by the
    /// [GAP Get Bonded Devices](crate::vendor::command::gap::GapCommands::get_bonded_devices) command.
    GapGetBondedDevices(GapBondedDevices),

    /// Parameters returned by the
    /// [GAP Set Broadcast Mode](crate::vendor::command::gap::GapCommands::set_broadcast_mode) command.
    GapSetBroadcastMode(crate::Status),

    /// Parameters returned by the
    /// [GAP Start Observation Procedure](crate::vendor::command::gap::GapCommands::start_observation_procedure) command.
    GapStartObservationProcedure(crate::Status),

    /// Parameters returned by the
    /// [GAP Is Device Bonded](crate::vendor::command::gap::GapCommands::is_device_bonded)
    /// command.
    GapIsDeviceBonded(crate::Status),

    /// Parameters returned by the
    /// [GAP Add Devices To List](crate::vendor::command::gap::GapCommands::add_devices_to_list) command.
    GapAddDevicesToList(crate::Status),

    /// Parameters returned by the
    /// [GAP Add Devices To Resolving List](crate::vendor::command::gap::GapCommands::add_devices_to_resolving_list) command.
    GapAddDevicesToResolvingList(crate::Status),

    /// Parameters returned by the
    /// [GAP Adv Clear Sets](crate::vendor::command::gap::GapCommands::adv_clear_sets) command.
    GapAdvClearSets(crate::Status),

    /// Parameters returned by the
    /// [GAP Adv Set Configuration](crate::vendor::command::gap::GapCommands::adv_set_configuration) command.
    GapAdvSetConfiguration(crate::Status),

    /// Return parameters of a known vendor command whose decoder is disabled by its `return-*`
    /// cargo feature.
    Unparsed(crate::event::command::UnparsedReturnParameters),

    /// Parameters returned by the
    /// [GAP Adv Set Enable](crate::vendor::command::gap::GapCommands::adv_set_enable) command.
    GapAdvSetEnable(crate::Status),

    /// Parameters returned by the
    /// [GAP Adv Set Adv Data](crate::vendor::command::gap::GapCommands::adv_set_adv_data) command.
    GapAdvSetAdvData(crate::Status),
    
    /// Parameters returned by the
    /// [GAP Adv Set Scan Response Data](crate::vendor::command::gap::GapCommands::adv_set_scan_response_data) command.
    GapAdvSetScanResponseData(crate::Status),
    
    /// Parameters returned by the
    /// [GAP Adv Remove Set](crate::vendor::command::gap::GapCommands::adv_remove_set) command.
    GapAdvRemoveSet(crate::Status),

    /// Parameters returned by the
    /// [GATT Init](crate::vendor::command::gatt::GattCommands::init) command.
    GattInit(crate::Status),

    /// Parameters returned by the
    /// [GATT Add Service](crate::vendor::command::gatt::GattCommands::add_service) command.
    GattAddService(GattService),

    /// Parameters returned by the
    /// [GATT Include Service](crate::vendor::command::gatt::GattCommands::include_service)
    /// command.
    GattIncludeService(GattService),

    /// Parameters returned by the
    /// [GATT Add Characteristic](crate::vendor::command::gatt::GattCommands::add_characteristic) command.
    GattAddCharacteristic(GattCharacteristic),

    /// Parameters returned by the
    /// [GATT Add Characteristic Descriptor](crate::vendor::command::gatt::GattCommands::add_characteristic_descriptor) command.
    GattAddCharacteristicDescriptor(GattCharacteristicDescriptor),

    /// Parameters returned by the
    /// [GATT Update Characteristic Value](crate::vendor::command::gatt::GattCommands::update_characteristic_value) command.
    GattUpdateCharacteristicValue(crate::Status),

    /// Parameters returned by the
    /// [GATT Delete Characteristic](crate::vendor::command::gatt::GattCommands::delete_characteristic) command.
    GattDeleteCharacteristic(crate::Status),

    /// Parameters returned by the
    /// [GATT Delete Service](crate::vendor::command::gatt::GattCommands::delete_service)
    /// command.
    GattDeleteService(crate::Status),

    /// Parameters returned by the
    /// [GATT Delete Included Service](crate::vendor::command::gatt::GattCommands::delete_included_service) command.
    GattDeleteIncludedService(crate::Status),

    /// Parameters returned by the [GATT Set Event Mask](crate::vendor::command::gatt::GattCommands::set_event_mask)
    /// command.
    GattSetEventMask(crate::Status),

    /// Parameters returned by the
    /// [GATT Write Without Response](crate::vendor::command::gatt::GattCommands::write_without_response) command.
    GattWriteWithoutResponse(crate::Status),

    /// Parameters returned by the
    /// [GATT Signed Write Without Response](crate::vendor::command::gatt::GattCommands::signed_write_without_response) command.
    GattSignedWriteWithoutResponse(crate::Status),

    /// Parameters returned by the
    /// [GATT Confirm Indication](crate::vendor::command::gatt::GattCommands::confirm_indication) command.
    GattConfirmIndication(crate::Status),

    /// Parameters returned by the [GATT Write Response](crate::vendor::command::gatt::GattCommands::write_response)
    /// command.
    GattWriteResponse(crate::Status),

    /// Parameters returned by the [GATT Allow Read](crate::vendor::command::gatt::GattCommands::allow_read) command.
    GattAllowRead(crate::Status),

    /// Parameters returned by the
    /// [GATT Set Security Permission](crate::vendor::command::gatt::GattCommands::set_security_permission) command.
    GattSetSecurityPermission(crate::Status),

    /// Parameters returned by the
    /// [GATT Set Descriptor Value](crate::vendor::command::gatt::GattCommands::set_descriptor_value) command.
    GattSetDescriptorValue(crate::Status),

    /// Parameters returned by the
    /// [GATT Read Handle Value](crate::vendor::command::gatt::GattCommands::read_handle_value) command.
    GattReadHandleValue(GattHandleValue),

    /// Parameters returned by the
    /// [GATT Read Handle Value](crate::vendor::command::gatt::GattCommands::read_handle_value_offset) command.
    GattReadHandleValueOffset(GattHandleValue),

    /// Parameters returned by the
    /// [GATT Update Long Characteristic Value](crate::vendor::command::gatt::GattCommands::update_characteristic_value_ext) command.
    GattUpdateLongCharacteristicValue(crate::Status),

    /// Status returned by the
    /// [L2CAP Connection Parameter Update Response](crate::vendor::command::l2cap::L2capCommands::connection_parameter_update_response) command.
    L2CapConnectionParameterUpdateResponse(crate::Status),

    L2CapCocConnect(crate::Status),
    L2CapCocConnectConfirm(CocConnectConfirm),
    L2CapCocReconfig(crate::Status),
    L2CapCocReconfigConfirm(crate::Status),
    L2CapCocDisconnect(crate::Status),
    L2CapCocFlowControl(crate::Status),
    L2CapCocTxData(crate::Status),
}

impl VendorReturnParameters {
    pub(crate) fn new(bytes: &[u8]) -> Result<Self, crate::event::Error> {
        check_len_at_least(bytes, 3)?;

        let opcode = crate::Opcode(LittleEndian::read_u16(&bytes[1..]));
        match opcode {
            crate::vendor::opcode::HAL_GET_FIRMWARE_REVISION => gated!(
                "return-hal-get-firmware-revision",
                Ok(VendorReturnParameters::HalGetFirmwareRevision(
                    to_hal_firmware_revision(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_WRITE_CONFIG_DATA => gated!(
                "return-hal-write-config-data",
                Ok(
                    VendorReturnParameters::HalWriteConfigData(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_READ_CONFIG_DATA => gated!(
                "return-hal-read-config-data",
                Ok(
                    VendorReturnParameters::HalReadConfigData(to_hal_config_data(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_SET_TX_POWER_LEVEL => gated!(
                "return-hal-set-tx-power-level",
                Ok(
                    VendorReturnParameters::HalSetTxPowerLevel(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_DEVICE_STANDBY => gated!(
                "return-hal-device-standby",
                Ok(
                    VendorReturnParameters::HalDeviceStandby(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_TX_TEST_PACKET_COUNT => gated!(
                "return-hal-get-tx-test-packet-count",
                Ok(VendorReturnParameters::HalGetTxTestPacketCount(
                    to_hal_tx_test_packet_count(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_START_TONE => gated!(
                "return-hal-start-tone",
                Ok(VendorReturnParameters::HalStartTone(
                    to_status(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_STOP_TONE => gated!(
                "return-hal-stop-tone",
                Ok(VendorReturnParameters::HalStopTone(to_status(&bytes[3..])?)),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_GET_LINK_STATUS => gated!(
                "return-hal-get-link-status",
                Ok(
                    VendorReturnParameters::HalGetLinkStatus(to_hal_link_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_GET_ANCHOR_PERIOD => gated!(
                "return-hal-get-anchor-period",
                Ok(
                    VendorReturnParameters::HalGetAnchorPeriod(to_hal_anchor_period(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_GET_PM_DEBUG_INFO => gated!(
                "return-hal-get-pm-debug-info",
                Ok(
                    VendorReturnParameters::HalGetPmDebugInfo(to_hal_pm_debug_info(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_READ_RSSI => gated!(
                "return-hal-read-rssi",
                Ok(VendorReturnParameters::HalReadRssi({
                    require_len!(&bytes[3..], 1);
                    bytes[3]
                })),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_READ_RADIO_REG => gated!(
                "return-hal-read-radio-reg",
                Ok(VendorReturnParameters::HalReadRadioReg({
                    require_len!(&bytes[3..], 1);
                    bytes[3]
                })),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::HAL_READ_RAW_RSSI => gated!(
                "return-hal-read-raw-rssi",
                Ok(VendorReturnParameters::HalReadRawRssi({
                    require_len!(&bytes[3..], 1);
                    bytes[3]
                })),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_NONDISCOVERABLE => gated!(
                "return-gap-set-non-discoverable",
                Ok(
                    VendorReturnParameters::GapSetNonDiscoverable(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_DISCOVERABLE => gated!(
                "return-gap-set-discoverable",
                Ok(
                    VendorReturnParameters::GapSetDiscoverable(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_DIRECT_CONNECTABLE => gated!(
                "return-gap-set-direct-connectable",
                Ok(
                    VendorReturnParameters::GapSetDirectConnectable(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_IO_CAPABILITY => gated!(
                "return-gap-set-io-capability",
                Ok(
                    VendorReturnParameters::GapSetIoCapability(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_AUTHENTICATION_REQUIREMENT => gated!(
                "return-gap-set-authentication-requirement",
                Ok(
                    VendorReturnParameters::GapSetAuthenticationRequirement(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_AUTHORIZATION_REQUIREMENT => gated!(
                "return-gap-set-authorization-requirement",
                Ok(
                    VendorReturnParameters::GapSetAuthorizationRequirement(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_PASS_KEY_RESPONSE => gated!(
                "return-gap-pass-key-response",
                Ok(
                    VendorReturnParameters::GapPassKeyResponse(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_AUTHORIZATION_RESPONSE => gated!(
                "return-gap-authorization-response",
                Ok(
                    VendorReturnParameters::GapAuthorizationResponse(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_INIT => gated!(
                "return-gap-init",
                Ok(VendorReturnParameters::GapInit(to_gap_init(&bytes[3..])?)),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_NONCONNECTABLE => gated!(
                "return-gap-set-non-connectable",
                Ok(
                    VendorReturnParameters::GapSetNonConnectable(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_UNDIRECTED_CONNECTABLE => gated!(
                "return-gap-set-undirected-connectable",
                Ok(
                    VendorReturnParameters::GapSetUndirectedConnectable(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_UPDATE_ADVERTISING_DATA => gated!(
                "return-gap-update-advertising-data",
                Ok(
                    VendorReturnParameters::GapUpdateAdvertisingData(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_DELETE_AD_TYPE => gated!(
                "return-gap-delete-ad-type",
                Ok(
                    VendorReturnParameters::GapDeleteAdType(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_GET_SECURITY_LEVEL => gated!(
                "return-gap-get-security-level",
                Ok(
                    VendorReturnParameters::GapGetSecurityLevel(to_gap_security_level(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_EVENT_MASK => gated!(
                "return-gap-set-event-mask",
                Ok(
                    VendorReturnParameters::GapSetEventMask(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_CONFIGURE_WHITE_LIST => gated!(
                "return-gap-configure-white-list",
                Ok(
                    VendorReturnParameters::GapConfigureWhiteList(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_CLEAR_SECURITY_DATABASE => gated!(
                "return-gap-clear-security-database",
                Ok(
                    VendorReturnParameters::GapClearSecurityDatabase(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ALLOW_REBOND => gated!(
                "return-gap-allow-rebond",
                Ok(VendorReturnParameters::GapAllowRebond(
                    to_status(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_TERMINATE_PROCEDURE => gated!(
                "return-gap-terminate-procedure",
                Ok(
                    VendorReturnParameters::GapTerminateProcedure(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_RESOLVE_PRIVATE_ADDRESS => gated!(
                "return-gap-resolve-private-address",
                Ok(VendorReturnParameters::GapResolvePrivateAddress(
                    to_gap_resolve_private_address(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_GET_BONDED_DEVICES => gated!(
                "return-gap-get-bonded-devices",
                Ok(
                    VendorReturnParameters::GapGetBondedDevices(to_gap_bonded_devices(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_SET_BROADCAST_MODE => gated!(
                "return-gap-set-broadcast-mode",
                Ok(
                    VendorReturnParameters::GapSetBroadcastMode(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_START_OBSERVATION_PROCEDURE => gated!(
                "return-gap-start-observation-procedure",
                Ok(
                    VendorReturnParameters::GapStartObservationProcedure(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_IS_DEVICE_BONDED => gated!(
                "return-gap-is-device-bonded",
                Ok(
                    VendorReturnParameters::GapIsDeviceBonded(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADD_DEVICES_TO_LIST => gated!(
                "return-gap-add-devices-to-list",
                Ok(
                    VendorReturnParameters::GapAddDevicesToList(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADD_DEVICES_TO_RESOLVING_LIST => gated!(
                "return-gap-add-devices-to-resolving-list",
                Ok(
                    VendorReturnParameters::GapAddDevicesToResolvingList(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_INIT => gated!(
                "return-gatt-init",
                Ok(VendorReturnParameters::GattInit(to_status(&bytes[3..])?)),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_ADD_SERVICE => gated!(
                "return-gatt-add-service",
                Ok(VendorReturnParameters::GattAddService(
                    to_gatt_service(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_INCLUDE_SERVICE => gated!(
                "return-gatt-include-service",
                Ok(
                    VendorReturnParameters::GattIncludeService(to_gatt_service(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_ADD_CHARACTERISTIC => gated!(
                "return-gatt-add-characteristic",
                Ok(
                    VendorReturnParameters::GattAddCharacteristic(to_gatt_characteristic(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_ADD_CHARACTERISTIC_DESCRIPTOR => gated!(
                "return-gatt-add-characteristic-descriptor",
                Ok(VendorReturnParameters::GattAddCharacteristicDescriptor(
                    to_gatt_characteristic_descriptor(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_UPDATE_CHARACTERISTIC_VALUE => gated!(
                "return-gatt-update-characteristic-value",
                Ok(
                    VendorReturnParameters::GattUpdateCharacteristicValue(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_DELETE_CHARACTERISTIC => gated!(
                "return-gatt-delete-characteristic",
                Ok(
                    VendorReturnParameters::GattDeleteCharacteristic(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_DELETE_SERVICE => gated!(
                "return-gatt-delete-service",
                Ok(
                    VendorReturnParameters::GattDeleteService(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_DELETE_INCLUDED_SERVICE => gated!(
                "return-gatt-delete-included-service",
                Ok(
                    VendorReturnParameters::GattDeleteIncludedService(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_SET_EVENT_MASK => gated!(
                "return-gatt-set-event-mask",
                Ok(
                    VendorReturnParameters::GattSetEventMask(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_WRITE_WITHOUT_RESPONSE => gated!(
                "return-gatt-write-without-response",
                Ok(
                    VendorReturnParameters::GattWriteWithoutResponse(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_SIGNED_WRITE_WITHOUT_RESPONSE => gated!(
                "return-gatt-signed-write-without-response",
                Ok(
                    VendorReturnParameters::GattSignedWriteWithoutResponse(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_CONFIRM_INDICATION => gated!(
                "return-gatt-confirm-indication",
                Ok(
                    VendorReturnParameters::GattConfirmIndication(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_WRITE_RESPONSE => gated!(
                "return-gatt-write-response",
                Ok(
                    VendorReturnParameters::GattWriteResponse(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_ALLOW_READ => gated!(
                "return-gatt-allow-read",
                Ok(VendorReturnParameters::GattAllowRead(
                    to_status(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_SET_SECURITY_PERMISSION => gated!(
                "return-gatt-set-security-permission",
                Ok(
                    VendorReturnParameters::GattSetSecurityPermission(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_SET_DESCRIPTOR_VALUE => gated!(
                "return-gatt-set-descriptor-value",
                Ok(
                    VendorReturnParameters::GattSetDescriptorValue(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_READ_HANDLE_VALUE => gated!(
                "return-gatt-read-handle-value",
                Ok(
                    VendorReturnParameters::GattReadHandleValue(to_gatt_handle_value(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_READ_HANDLE_VALUE_OFFSET => gated!(
                "return-gatt-read-handle-value-offset",
                Ok(VendorReturnParameters::GattReadHandleValueOffset(
                    to_gatt_handle_value(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GATT_UPDATE_LONG_CHARACTERISTIC_VALUE => gated!(
                "return-gatt-update-long-characteristic-value",
                Ok(
                    VendorReturnParameters::GattUpdateLongCharacteristicValue(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_CONN_PARAM_UPDATE_RESP => gated!(
                "return-l2cap-connection-parameter-update-response",
                Ok(
                    VendorReturnParameters::L2CapConnectionParameterUpdateResponse(to_status(
                        &bytes[3..],
                    )?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_CONNECT => gated!(
                "return-l2cap-coc-connect",
                Ok(
                    VendorReturnParameters::L2CapCocConnect(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_CONNECT_CONFIRM => gated!(
                "return-l2cap-coc-connect-confirm",
                Ok(VendorReturnParameters::L2CapCocConnectConfirm(
                    to_coc_connect_confirm(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_RECONFIG => gated!(
                "return-l2cap-coc-reconfig",
                Ok(
                    VendorReturnParameters::L2CapCocReconfig(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_RECONFIG_CONFIRM => gated!(
                "return-l2cap-coc-reconfig-confirm",
                Ok(
                    VendorReturnParameters::L2CapCocReconfigConfirm(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_FLOW_CONTROL => gated!(
                "return-l2cap-coc-flow-control",
                Ok(
                    VendorReturnParameters::L2CapCocFlowControl(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_TX_DATA => gated!(
                "return-l2cap-coc-tx-data",
                Ok(VendorReturnParameters::L2CapCocTxData(
                    to_status(&bytes[3..])?,
                )),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::L2CAP_COC_DISCONNECT => gated!(
                "return-l2cap-coc-disconnect",
                Ok(
                    VendorReturnParameters::L2CapCocDisconnect(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADV_SET_CONFIGURATION => gated!(
                "return-gap-adv-set-configuration",
                Ok(
                    VendorReturnParameters::GapAdvSetConfiguration(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADV_SET_ENABLE => gated!(
                "return-gap-adv-set-enable",
                Ok(
                    VendorReturnParameters::GapAdvSetEnable(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADV_SET_ADV_DATA => gated!(
                "return-gap-adv-set-adv-data",
                Ok(
                    VendorReturnParameters::GapAdvSetAdvData(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADV_SET_SCAN_RESPONSE_DATA => gated!(
                "return-gap-adv-set-scan-response-data",
                Ok(
                    VendorReturnParameters::GapAdvSetScanResponseData(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADV_REMOVE_SET => gated!(
                "return-gap-adv-remove-set",
                Ok(
                    VendorReturnParameters::GapAdvRemoveSet(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            crate::vendor::opcode::GAP_ADV_CLEAR_SETS => gated!(
                "return-gap-adv-clear-sets",
                Ok(
                    VendorReturnParameters::GapAdvClearSets(to_status(&bytes[3..])?),
                ),
                Ok(VendorReturnParameters::Unparsed(
                    crate::event::command::to_unparsed(opcode, &bytes[3..])?,
                )),
            ),
            other => Err(crate::event::Error::UnknownOpcode(other)),
        }
    }
}

fn check_len_at_least(buffer: &[u8], len: usize) -> Result<(), crate::event::Error> {
    if buffer.len() < len {
        Err(crate::event::Error::BadLength(buffer.len(), len))
    } else {
        Ok(())
    }
}

fn to_status(bytes: &[u8]) -> Result<crate::Status, crate::event::Error> {
    require_len_at_least!(bytes, 1);
    bytes[0].try_into().map_err(crate::event::rewrap_bad_status)
}

/// Parameters returned by the
/// [HAL Get Firmware Revision](crate::vendor::command::hal::HalCommands::get_firmware_revision) command.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HalFirmwareRevision {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// The firmware revision number.
    pub revision: u16,
}

fn to_hal_firmware_revision(bytes: &[u8]) -> Result<HalFirmwareRevision, crate::event::Error> {
    require_len!(bytes, 3);

    Ok(HalFirmwareRevision {
        status: to_status(bytes)?,
        revision: LittleEndian::read_u16(&bytes[1..]),
    })
}

/// Parameters returned by the [HAL Read Config Data](crate::vendor::command::hal::HalCommands::read_config_data)
/// command.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HalConfigData {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Requested value.
    ///
    /// The value is requested by offset, and distinguished upon return by length only. This means
    /// that this event cannot distinguish between the 16-byte encryption keys
    /// ([EncryptionRoot](crate::vendor::command::hal::ConfigParameter::EncryptionRoot) and
    /// [IdentityRoot](crate::vendor::command::hal::ConfigParameter::IdentityRoot)) or between the single-byte values
    /// ([LinkLayerOnly](crate::vendor::command::hal::ConfigParameter::LinkLayerOnly) or
    /// [Role](crate::vendor::command::hal::ConfigParameter::Role)).
    pub value: HalConfigParameter,
}

/// Potential values that can be fetched by
/// [HAL Read Config Data](crate::vendor::command::hal::HalCommands::read_config_data).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HalConfigParameter {
    /// Bluetooth public address. Corresponds to
    /// [PublicAddress](crate::vendor::command::hal::ConfigParameter::PublicAddress).
    PublicAddress(crate::BdAddr),

    /// Bluetooth random address. Corresponds to
    /// [RandomAddress](crate::vendor::command::hal::ConfigParameter::RandomAddress).
    RandomAddress(crate::BdAddr),

    /// Diversifier used to derive CSRK (connection signature resolving key).  Corresponds to
    /// [Diversifier](crate::vendor::command::hal::ConfigParameter::Diversifier).
    Diversifier(u16),

    /// A requested encryption key. Corresponds to either
    /// [EncryptionRoot](crate::vendor::command::hal::ConfigParameter::EncryptionRoot) or
    /// [IdentityRoot](crate::vendor::command::hal::ConfigParameter::IdentityRoot).
    EncryptionKey(crate::host::EncryptionKey),

    /// A single-byte value. Corresponds to either
    /// [LinkLayerOnly](crate::vendor::command::hal::ConfigParameter::LinkLayerOnly) or
    /// [Role](crate::vendor::command::hal::ConfigParameter::Role).
    Byte(u8),
}

fn to_hal_config_data(bytes: &[u8]) -> Result<HalConfigData, crate::event::Error> {
    require_len_at_least!(bytes, 2);
    Ok(HalConfigData {
        status: to_status(bytes)?,
        value: to_hal_config_parameter(&bytes[1..])?,
    })
}

fn to_hal_config_parameter(bytes: &[u8]) -> Result<HalConfigParameter, crate::event::Error> {
    match bytes.len() {
        6 => {
            let mut buf = [0; 6];
            buf.copy_from_slice(bytes);

            Ok(HalConfigParameter::PublicAddress(crate::BdAddr(buf)))
        }
        2 => Ok(HalConfigParameter::Diversifier(LittleEndian::read_u16(
            bytes,
        ))),
        16 => {
            let mut buf = [0; 16];
            buf.copy_from_slice(bytes);

            Ok(HalConfigParameter::EncryptionKey(
                crate::host::EncryptionKey(buf),
            ))
        }
        1 => Ok(HalConfigParameter::Byte(bytes[0])),
        other => Err(crate::event::Error::Vendor(
            super::VendorError::BadConfigParameterLength(other),
        )),
    }
}

/// Parameters returned by the
/// [HAL Get Tx Test Packet Count](crate::vendor::command::hal::HalCommands::get_tx_test_packet_count) command.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HalTxTestPacketCount {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Number of packets sent during the last Direct TX test.
    pub packet_count: u32,
}

fn to_hal_tx_test_packet_count(bytes: &[u8]) -> Result<HalTxTestPacketCount, crate::event::Error> {
    require_len!(bytes, 5);
    Ok(HalTxTestPacketCount {
        status: to_status(bytes)?,
        packet_count: LittleEndian::read_u32(&bytes[1..]),
    })
}

/// Parameters returned by the [HAL Get Link Status](crate::vendor::command::hal::HalCommands::get_link_status) command.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HalLinkStatus {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// State of the client connections.
    pub clients: [ClientStatus; 8],
}

/// State of a client connection.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClientStatus {
    /// Link state for the client.
    pub state: LinkState,

    /// Connection handle for the client
    pub conn_handle: crate::ConnectionHandle,
}

/// Potential states for a connection.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LinkState {
    /// Idle
    Idle,
    /// Advertising
    Advertising,
    /// Connected in peripheral role
    ConnectedAsPeripheral,
    /// Scanning
    Scanning,
    /// Reserved
    Reserved,
    /// Connected in primary role
    ConnectedAsPrimary,
    /// TX Test
    TxTest,
    /// RX Test
    RxTest,
}

impl TryFrom<u8> for LinkState {
    type Error = super::VendorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LinkState::Idle),
            1 => Ok(LinkState::Advertising),
            2 => Ok(LinkState::ConnectedAsPeripheral),
            3 => Ok(LinkState::Scanning),
            4 => Ok(LinkState::Reserved),
            5 => Ok(LinkState::ConnectedAsPrimary),
            6 => Ok(LinkState::TxTest),
            7 => Ok(LinkState::RxTest),
            _ => Err(super::VendorError::UnknownLinkState(value)),
        }
    }
}

fn to_hal_link_status(bytes: &[u8]) -> Result<HalLinkStatus, crate::event::Error> {
    require_len!(bytes, 25);

    let mut status = HalLinkStatus {
        status: to_status(&bytes[0..])?,
        clients: [ClientStatus {
            state: LinkState::Idle,
            conn_handle: crate::ConnectionHandle(0),
        }; 8],
    };

    for client in 0..8 {
        status.clients[client].state = bytes[1 + client]
            .try_into()
            .map_err(crate::event::Error::Vendor)?;
        status.clients[client].conn_handle = crate::ConnectionHandle(LittleEndian::read_u16(
            &bytes[9 + 2 * client..9 + 2 * (client + 1)],
        ));
    }

    Ok(status)
}

/// Parameters returned by the [HAL Get Anchor Period](crate::vendor::command::hal::HalCommands::get_anchor_period)
/// command.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HalAnchorPeriod {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Duration between the beginnings of sniff anchor points.
    pub anchor_interval: Duration,

    /// Maximum available size that can be allocated to a new connection slot.
    pub max_slot: Duration,
}

fn to_hal_anchor_period(bytes: &[u8]) -> Result<HalAnchorPeriod, crate::event::Error> {
    require_len!(bytes, 9);

    Ok(HalAnchorPeriod {
        status: to_status(bytes)?,
        anchor_interval: Duration::from_micros(
            625 * u64::from(LittleEndian::read_u32(&bytes[1..5])),
        ),
        max_slot: Duration::from_micros(625 * u64::from(LittleEndian::read_u32(&bytes[5..9]))),
    })
}

/// Parameters returned by the [HAL Get PM Debug Info](crate::vendor::command::hal::HalCommands::get_pm_debug_info)
/// command.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HalPmDebugInfo {
    /// MBlocks allocated for TXing
    pub tx: u8,
    /// MBlocks allocated for RXing
    pub rx: u8,
    /// Overall allocated MBlocks
    pub mblocks: u8,
}

fn to_hal_pm_debug_info(bytes: &[u8]) -> Result<HalPmDebugInfo, crate::event::Error> {
    require_len!(bytes, 3);

    Ok(HalPmDebugInfo {
        tx: bytes[0],
        rx: bytes[1],
        mblocks: bytes[2],
    })
}

/// Parameters returned by the [GAP Init](crate::vendor::command::gap::GapCommands::init) command.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GapInit {
    /// Did the command fail, and if so, how?
    ///
    /// Should be one of:
    /// - [Success](crate::Status::Success)
    /// - [InvalidParameters](crate::Status::InvalidParameters)
    pub status: crate::Status,

    /// Handle for the GAP service
    pub service_handle: AttributeHandle,

    /// Handle for the device name characteristic added to the GAP service.
    pub dev_name_handle: AttributeHandle,

    /// Handle for the appearance characteristic added to the GAP service.
    pub appearance_handle: AttributeHandle,
}

fn to_gap_init(bytes: &[u8]) -> Result<GapInit, crate::event::Error> {
    require_len!(bytes, 7);

    Ok(GapInit {
        status: to_status(bytes)?,
        service_handle: AttributeHandle(LittleEndian::read_u16(&bytes[1..])),
        dev_name_handle: AttributeHandle(LittleEndian::read_u16(&bytes[3..])),
        appearance_handle: AttributeHandle(LittleEndian::read_u16(&bytes[5..])),
    })
}

/// Parameters returned by the [GAP Get Security Level](crate::vendor::command::gap::GapCommands::get_security_level)
/// command.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GapSecurityLevel {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Is MITM (man-in-the-middle) protection required?
    pub mitm_protection_required: bool,

    /// Is bonding required?
    pub bonding_required: bool,

    /// Is out-of-band data present?
    pub out_of_band_data_present: bool,

    /// Is a pass key required, and if so, how is it generated?
    pub pass_key_required: PassKeyRequirement,
}

/// Options for pass key generation.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PassKeyRequirement {
    /// A pass key is not required.
    NotRequired,
    /// A fixed pin is present which is being used.
    FixedPin,
    /// Pass key required for pairing. An event will be generated when required.
    Generated,
}

impl TryFrom<u8> for PassKeyRequirement {
    type Error = super::VendorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PassKeyRequirement::NotRequired),
            0x01 => Ok(PassKeyRequirement::FixedPin),
            0x02 => Ok(PassKeyRequirement::Generated),
            _ => Err(super::VendorError::BadPassKeyRequirement(value)),
        }
    }
}

fn to_boolean(value: u8) -> Result<bool, super::VendorError> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(super::VendorError::BadBooleanValue(value)),
    }
}

fn to_gap_security_level(bytes: &[u8]) -> Result<GapSecurityLevel, crate::event::Error> {
    require_len!(bytes, 5);

    Ok(GapSecurityLevel {
        status: to_status(&bytes[0..])?,
        mitm_protection_required: to_boolean(bytes[1]).map_err(crate::event::Error::Vendor)?,
        bonding_required: to_boolean(bytes[2]).map_err(crate::event::Error::Vendor)?,
        out_of_band_data_present: to_boolean(bytes[3]).map_err(crate::event::Error::Vendor)?,
        pass_key_required: bytes[4].try_into().map_err(crate::event::Error::Vendor)?,
    })
}

/// Parameters returned by the
/// [GAP Resolve Private Address](crate::vendor::command::gap::GapCommands::resolve_private_address) command.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GapResolvePrivateAddress {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// If the address was successfully resolved, the peer address is returned.  This value is
    /// `None` if the address could not be resolved.
    pub bd_addr: Option<crate::BdAddr>,
}

fn to_gap_resolve_private_address(
    bytes: &[u8],
) -> Result<GapResolvePrivateAddress, crate::event::Error> {
    let status = to_status(bytes)?;
    if status == crate::Status::Success {
        require_len!(bytes, 7);

        let mut addr = [0; 6];
        addr.copy_from_slice(&bytes[1..7]);

        Ok(GapResolvePrivateAddress {
            status,
            bd_addr: Some(crate::BdAddr(addr)),
        })
    } else {
        Ok(GapResolvePrivateAddress {
            status,
            bd_addr: None,
        })
    }
}

/// Parameters returned by the [GAP Get Bonded Devices](crate::vendor::command::gap::GapCommands::get_bonded_devices)
/// command.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GapBondedDevices {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    // Number of peer addresses in the event, and a buffer that can hold all of the addresses.
    address_count: usize,
    address_buffer: [crate::BdAddrType; MAX_ADDRESSES],
}

impl Default for GapBondedDevices {
    fn default() -> Self {
        Self {
            status: crate::Status::Success,
            address_count: 0,
            address_buffer: [crate::BdAddrType::Public(crate::BdAddr([0, 0, 0, 0, 0, 0]));
                MAX_ADDRESSES],
        }
    }
}

// Max packet size (255 bytes) less non-address data (4 bytes) divided by peer address size (7):
const MAX_ADDRESSES: usize = 35;

impl GapBondedDevices {
    /// Return an iterator over the bonded device addresses.
    pub fn bonded_addresses(&self) -> &[crate::BdAddrType] {
        &self.address_buffer[..self.address_count]
    }
}

impl Debug for GapBondedDevices {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{{")?;
        for addr in self.bonded_addresses().iter() {
            write!(f, "{:?}, ", addr)?;
        }
        write!(f, "}}")
    }
}

fn to_gap_bonded_devices(bytes: &[u8]) -> Result<GapBondedDevices, crate::event::Error> {
    let status = to_status(bytes)?;
    match status {
        crate::Status::Success => {
            const HEADER_LEN: usize = 2;
            const ADDR_LEN: usize = 7;

            require_len_at_least!(bytes, HEADER_LEN);
            let address_count = bytes[1] as usize;
            if bytes.len() != HEADER_LEN + ADDR_LEN * address_count {
                return Err(crate::event::Error::Vendor(
                    super::VendorError::PartialBondedDeviceAddress,
                ));
            }

            let mut address_buffer =
                [crate::BdAddrType::Public(crate::BdAddr([0; 6])); MAX_ADDRESSES];
            for (i, byte) in address_buffer.iter_mut().enumerate().take(address_count) {
                let index = HEADER_LEN + i * ADDR_LEN;
                let mut addr = [0; 6];
                addr.copy_from_slice(&bytes[(1 + index)..(7 + index)]);
                *byte = crate::to_bd_addr_type(bytes[index], crate::BdAddr(addr)).map_err(|e| {
                    crate::event::Error::Vendor(super::VendorError::BadBdAddrType(e.0))
                })?;
            }

            Ok(GapBondedDevices {
                status,
                address_count,
                address_buffer,
            })
        }
        _ => Ok(GapBondedDevices {
            status,
            address_count: 0,
            address_buffer: [crate::BdAddrType::Public(crate::BdAddr([0; 6])); MAX_ADDRESSES],
        }),
    }
}

/// Parameters returned by the [GATT Add Service](crate::vendor::command::gatt::GattCommands::add_service) and
/// [GATT Include Service](crate::vendor::command::gatt::GattCommands::include_service) commands.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GattService {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Handle of the Service
    ///
    /// When this service is added to the server, a handle is allocated by the server to this
    /// service. Also server allocates a range of handles for this service from `service_handle` to
    /// `service_handle +
    /// [max_attribute_records](crate::vendor::command::gatt::AddServiceParameters::max_attribute_records)`.
    pub service_handle: AttributeHandle,
}

fn to_gatt_service(bytes: &[u8]) -> Result<GattService, crate::event::Error> {
    require_len!(bytes, 3);

    Ok(GattService {
        status: to_status(bytes)?,
        service_handle: AttributeHandle(LittleEndian::read_u16(&bytes[1..3])),
    })
}

/// Parameters returned by the [GATT Add Characteristic](crate::vendor::command::gatt::GattCommands::add_characteristic)
/// command.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GattCharacteristic {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Handle of the characteristic.
    pub characteristic_handle: AttributeHandle,
}

fn to_gatt_characteristic(bytes: &[u8]) -> Result<GattCharacteristic, crate::event::Error> {
    require_len!(bytes, 3);

    Ok(GattCharacteristic {
        status: to_status(bytes)?,
        characteristic_handle: AttributeHandle(LittleEndian::read_u16(&bytes[1..3])),
    })
}

/// Parameters returned by the
/// [GATT Add Characteristic Descriptor](crate::vendor::command::gatt::GattCommands::add_characteristic_descriptor) command.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GattCharacteristicDescriptor {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    /// Handle of the characteristic.
    pub descriptor_handle: AttributeHandle,
}

fn to_gatt_characteristic_descriptor(
    bytes: &[u8],
) -> Result<GattCharacteristicDescriptor, crate::event::Error> {
    require_len!(bytes, 3);

    Ok(GattCharacteristicDescriptor {
        status: to_status(bytes)?,
        descriptor_handle: AttributeHandle(LittleEndian::read_u16(&bytes[1..3])),
    })
}

/// Parameters returned by the [GATT Read Handle Value](crate::vendor::command::gatt::GattCommands::read_handle_value)
/// command.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GattHandleValue {
    /// Did the command fail, and if so, how?
    pub status: crate::Status,

    value_buf: [u8; GattHandleValue::MAX_VALUE_BUF],
    value_len: usize,
}

impl Debug for GattHandleValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{{")?;
        write!(f, "status: {:?}; value: {{", self.status)?;
        for addr in self.value().iter() {
            write!(f, "{:?}, ", addr)?;
        }
        write!(f, "}}}}")
    }
}

impl GattHandleValue {
    // Maximum length of the handle value. The spec says the length can be 2 bytes (up to 65535),
    // but the communication layer is limited to 255 bytes in a packet. There are 6 bytes reserved
    // for data other than the value, so the maximum length of the value buffer is 249 bytes.
    const MAX_VALUE_BUF: usize = 249;

    /// Return the handle value. Only valid bytes are returned.
    pub fn value(&self) -> &[u8] {
        &self.value_buf[..self.value_len]
    }
}

fn to_gatt_handle_value(bytes: &[u8]) -> Result<GattHandleValue, crate::event::Error> {
    require_len_at_least!(bytes, 3);

    let status = to_status(bytes)?;
    let value_len = LittleEndian::read_u16(&bytes[1..3]) as usize;
    require_len!(bytes, 3 + value_len);

    let mut handle_value = GattHandleValue {
        status,
        value_buf: [0; GattHandleValue::MAX_VALUE_BUF],
        value_len,
    };
    handle_value.value_buf[..value_len].copy_from_slice(&bytes[3..]);

    Ok(handle_value)
}

/// Parameters returned by the [L2Cap CoC Connect Confirm](crate::vendor::command::l2cap::L2capCommands::coc_connect_confirm) command.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CocConnectConfirm {
    /// Did the command fail, and if so, how?
    ///
    /// Should be one of:
    /// - [Success](crate::Status::Success)
    /// - [InvalidParameters](crate::Status::InvalidParameters)
    pub status: crate::Status,

    /// Handle for the channel indices
    value_buf: [u8; CocConnectConfirm::MAX_NUM_CHANNELS],
    value_len: usize,
}

impl CocConnectConfirm {
    const MAX_NUM_CHANNELS: usize = 5;

    /// Return the handle value. Only valid bytes are returned.
    pub fn channels(&self) -> &[u8] {
        &self.value_buf[..self.value_len]
    }
}

fn to_coc_connect_confirm(bytes: &[u8]) -> Result<CocConnectConfirm, crate::event::Error> {
    require_len_at_least!(bytes, 2);

    let status = to_status(bytes)?;
    let value_len = bytes[1] as usize;
    require_len!(bytes, 2 + value_len);

    let mut confirm = CocConnectConfirm {
        status,
        value_buf: [0u8; CocConnectConfirm::MAX_NUM_CHANNELS],
        value_len,
    };
    confirm.value_buf[..value_len].copy_from_slice(&bytes[2..]);

    Ok(confirm)
}
