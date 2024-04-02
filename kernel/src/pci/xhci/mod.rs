pub mod context;
pub mod event_ring;
pub mod port_phase;
pub mod software_ring;
pub mod transfer_request_block;
pub mod endpoint_type;

use core::mem::swap;

use common::iter_str::{IterStrFormat, Padding, Radix, ToIterStr};

use crate::{
    font::font_writer::FONT_HEIGHT,
    output_string,
    pixel_writer::pixel_color::PixelColor,
    services::Services,
    util::{get_bits_value, get_unsigned_int_8s, vector2::Vector2},
};

use self::{
    context::{DeviceContextBaseAddressArray, DeviceContexts, InputContexts},
    event_ring::EventRingManagerWithFixedSize,
    port_phase::PortPhase,
    software_ring::SoftwareRingManager,
    transfer_request_block::{
        typed_transfer_request_block::{
            command_completion_event_trb::COMMAND_COMPLETION_CODE_SUCCESS,
            disable_slot_command_trb::DisableSlotCommandTrb,
            enable_slot_command_trb::EnableSlotCommandTrb, IncomingTypedTransferRequestBlock,
            OutgoingTypedTransferRequestBlock, TRB_TYPE_ID_DISABLE_SLOT_COMMAND,
            TRB_TYPE_ID_ENABLE_SLOT_COMMAND,
        },
        TransferRequestBlock,
    },
};

struct PortStatus {
    is_connected: bool,
    is_enabled: bool,
    is_resetting: bool,
}

impl PortStatus {
    pub fn new(is_connected: bool, is_enabled: bool, is_resetting: bool) -> Self {
        Self {
            is_connected,
            is_enabled,
            is_resetting,
        }
    }
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
    pub fn is_resetting(&self) -> bool {
        self.is_resetting
    }
}

const MAX_DEVICE_SLOTS_DESIRED: u8 = 8;
const COMMAND_RING_SIZE: usize = 32;
const TRANSFER_RING_SIZE: usize = 32;
const PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_COUNT: u16 = 1;
const PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_SIZE: u16 = 64;
const MAX_PORT_POSSIBLE: u8 = 255;
pub struct XhcDevice {
    capability_registers: XhcCapabilityRegisters,
    operational_registers: XhcOperationalRegisters,
    device_context_base_address_array:
        DeviceContextBaseAddressArray<{ MAX_DEVICE_SLOTS_DESIRED as usize }>,
    command_ring: SoftwareRingManager<COMMAND_RING_SIZE>,
    primary_interrupter_event_ring: EventRingManagerWithFixedSize<
        PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_SIZE,
        PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_COUNT,
    >,
    default_control_pipe_transfer_rings:
        [SoftwareRingManager<TRANSFER_RING_SIZE>; MAX_DEVICE_SLOTS_DESIRED as usize],
    runtime_registers: XhcRuntimeRegisters,
    doorbell_registers: XhcDoorbellRegisters,
    ports_status: [PortPhase; MAX_PORT_POSSIBLE as usize],
    device_contexts: DeviceContexts<{ MAX_DEVICE_SLOTS_DESIRED as usize }>,
    input_contexts: InputContexts<{ MAX_DEVICE_SLOTS_DESIRED as usize }>,
    port_id_of_slot: [Option<u8>; MAX_DEVICE_SLOTS_DESIRED as usize],
    ports_queue_waiting_for_slot: FixedSizePortQueue<{ MAX_PORT_POSSIBLE as usize }>,
}

pub struct FixedSizePortQueue<const COUNT: usize> {
    buf: [Option<u8>; COUNT],
    count: usize,
    tail: usize,
    head: usize,
}

impl<const COUNT: usize> FixedSizePortQueue<COUNT> {
    pub const fn new() -> Self {
        const RESET_VALUE: Option<u8> = None;
        Self {
            buf: [RESET_VALUE; COUNT],
            count: 0,
            tail: 0,
            head: 0,
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        match self.buf[self.tail] {
            None => None,
            Some(_) => {
                let mut prev_val = None;
                swap(&mut prev_val, &mut self.buf[self.tail]);
                self.tail = (self.tail + 1) % COUNT;
                self.count -= 1;
                prev_val
            }
        }
    }

    pub fn front(&self) -> &Option<u8> {
        &self.buf[self.tail]
    }

    pub fn push(&mut self, v: u8) -> Result<(), ()> {
        match self.buf[self.head] {
            Some(_) => Err(()),
            None => {
                self.buf[self.head] = Some(v);
                self.head = (self.head + 1) % COUNT;
                self.count += 1;
                Ok(())
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl XhcDevice {
    pub fn new(base_address: u64) -> Self {
        let capability_registers = XhcCapabilityRegisters::new(base_address);
        let operational_registers_offset = capability_registers.capability_register_length();
        let runtime_registers_offset = capability_registers.runtime_register_space_offset();
        let doorbell_registers_offset = capability_registers.doorbell_offset();
        const PORTS_STATUS_RESET_VALUE: PortPhase = PortPhase::NotConnected;

        Self {
            capability_registers,
            operational_registers: XhcOperationalRegisters::new(
                base_address + operational_registers_offset as u64,
            ),
            device_context_base_address_array: DeviceContextBaseAddressArray::new(),
            command_ring: SoftwareRingManager::new(XhcOperationalRegisters::new(
                base_address + operational_registers_offset as u64,
            )),
            primary_interrupter_event_ring: EventRingManagerWithFixedSize::new(
                XhcRuntimeRegisters::new(base_address + runtime_registers_offset as u64)
                    .get_interrupter_register_set(0),
            ),
            default_control_pipe_transfer_rings: [(); MAX_DEVICE_SLOTS_DESIRED as usize].map(
                |()| {
                    SoftwareRingManager::new(XhcOperationalRegisters::new(
                        base_address + operational_registers_offset as u64,
                    ))
                },
            ),
            runtime_registers: XhcRuntimeRegisters::new(
                base_address + runtime_registers_offset as u64,
            ),
            doorbell_registers: XhcDoorbellRegisters::new(
                base_address + doorbell_registers_offset as u64,
            ),
            ports_status: [PORTS_STATUS_RESET_VALUE; MAX_PORT_POSSIBLE as usize],
            device_contexts: DeviceContexts::new(),
            input_contexts: InputContexts::new(),
            port_id_of_slot: [None; MAX_DEVICE_SLOTS_DESIRED as usize],
            ports_queue_waiting_for_slot: FixedSizePortQueue::new(),
        }
    }

    pub fn initialize(&mut self, services: &Services, height: &mut u32) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Check USB status host controller halted bit.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        if (self.operational_registers.usb_status() & USB_STATUS_HOST_CONTROLLER_HALTED_MASK) == 0 {
            self.operational_registers.usb_command_stop();

            'a: loop {
                match services.time_services().wait_for_nano_seconds(1_000_000) {
                    Ok(()) => (),
                    Err(()) => return Err(()),
                }
                if (self.operational_registers.usb_status()
                    & USB_STATUS_HOST_CONTROLLER_HALTED_MASK)
                    != 0
                {
                    break 'a ();
                }
            }
        }

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Do USB host controller reset.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.operational_registers
            .usb_command_host_controller_reset();

        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.operational_registers.usb_command() & USB_COMMAND_HOST_CONTROLLER_RESET_MASK
                == 0
            {
                break 'a ();
            }
        }
        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.operational_registers.usb_status() & USB_STATUS_CONTROLLER_NOT_READY == 0 {
                break 'a ();
            }
        }

        let max_device_slots = get_bits_value(
            self.capability_registers
                .host_controller_structural_parameters_1(),
            0,
            7,
        ) as u8;
        let slots_enabled = if max_device_slots < MAX_DEVICE_SLOTS_DESIRED {
            max_device_slots
        } else {
            MAX_DEVICE_SLOTS_DESIRED
        };
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                b"Set max device slots to ".to_iter_str(IterStrFormat::none()),
                slots_enabled.to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.operational_registers
            .set_number_of_device_slots_enabled(slots_enabled);

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Set DCBAAP.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.operational_registers
            .set_device_context_base_address_array_pointer(
                self.device_context_base_address_array.pointer() as u64,
            );

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Set command ring.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.command_ring.initialize();

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Set primary interrupter event ring.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.primary_interrupter_event_ring.initialize();
        self.primary_interrupter_event_ring.set_interrupt_pending();

        self.operational_registers.usb_command_interrupter_enable();

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Succeeded in Xhc initialization.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        Ok(())
    }

    pub fn run(&self, services: &Services, height: &mut u32) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Run usb.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.operational_registers.usb_command_run();
        match services.time_services().wait_for_nano_seconds(1_000_000) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }

        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.operational_registers.usb_status() & USB_STATUS_HOST_CONTROLLER_HALTED_MASK == 0
            {
                break 'a ();
            }
        }

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Usb started.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        Ok(())
    }

    fn is_context_size_64(&self) -> bool {
        (self
            .capability_registers
            .host_controller_cabability_parameters_1()
            & 0x0000_0004)
            != 0
    }

    fn get_port_speed(&self, index: u8) -> u8 {
        (self
            .operational_registers
            .port_status_and_control_register(index)
            >> 10) as u8
            & 0xF
    }

    fn get_port_status(&self, index: u8) -> PortStatus {
        let port_status_and_control_register = self
            .operational_registers
            .port_status_and_control_register(index);
        PortStatus::new(
            (port_status_and_control_register & 0x1) != 0,
            (port_status_and_control_register & 0x2) != 0,
            (port_status_and_control_register & 0x10) != 0,
        )
    }

    fn set_port_status(&self, index: u8, reset: bool) {
        let port_status_and_control_register = self
            .operational_registers
            .port_status_and_control_register(index);
        self.operational_registers
            .set_port_status_and_control_register(
                index,
                (port_status_and_control_register & 0x0E01_C3E0)
                    + 0x0026_0000
                    + if reset { 0x10 } else { 0x0 },
            );
    }

    fn enqueue_for_enabling_slot(
        &mut self,
        index: u8,
        services: &Services,
        height: &mut u32,
    ) -> Result<(), ()> {
        match self.ports_queue_waiting_for_slot.push(index) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        self.ports_status[index as usize - 1] = PortPhase::EnablingSlot;
        if self.ports_queue_waiting_for_slot.count == 1 {
            match self.enable_slot(services, height) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
        }
        Ok(())
    }

    fn on_port_status_changed(
        &mut self,
        index: u8,
        services: &Services,
        height: &mut u32,
    ) -> Result<(), ()> {
        let port_status = self.get_port_status(index);
        let mut to_reset = false;
        match self.ports_status[index as usize - 1] {
            PortPhase::NotConnected => {
                if port_status.is_connected() {
                    match output_string!(
                        services,
                        PixelColor::new(128, 0, 0),
                        Vector2::new(0, *height),
                        [
                            index.to_iter_str(IterStrFormat::none()),
                            b"-th usb port is connected.".to_iter_str(IterStrFormat::none()),
                        ]
                    ) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                    *height += FONT_HEIGHT;
                    if port_status.is_enabled() {
                        match self.enqueue_for_enabling_slot(index, services, height) {
                            Ok(()) => (),
                            Err(()) => return Err(()),
                        }
                    } else {
                        to_reset = true;
                        self.ports_status[index as usize - 1] = PortPhase::ResettingPort;
                        match output_string!(
                            services,
                            PixelColor::new(128, 0, 0),
                            Vector2::new(0, *height),
                            [
                                b"Start Resetting ".to_iter_str(IterStrFormat::none()),
                                index.to_iter_str(IterStrFormat::none()),
                                b"-th usb port.".to_iter_str(IterStrFormat::none()),
                            ]
                        ) {
                            Ok(()) => (),
                            Err(()) => return Err(()),
                        }
                        *height += FONT_HEIGHT;
                    }
                }
            }
            PortPhase::ResettingPort => {
                if port_status.is_connected() {
                    match output_string!(
                        services,
                        PixelColor::new(128, 0, 0),
                        Vector2::new(0, *height),
                        [
                            index.to_iter_str(IterStrFormat::none()),
                            b"-th usb port is connected.".to_iter_str(IterStrFormat::none()),
                        ]
                    ) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                    *height += FONT_HEIGHT;
                    if port_status.is_enabled() {
                        match self.enqueue_for_enabling_slot(index, services, height) {
                            Ok(()) => (),
                            Err(()) => return Err(()),
                        }
                    } else if !port_status.is_resetting() {
                        _ = output_string!(
                            services,
                            PixelColor::new(128, 0, 0),
                            Vector2::new(0, *height),
                            [
                                b"Failed to reset ".to_iter_str(IterStrFormat::none()),
                                index.to_iter_str(IterStrFormat::none()),
                                b"-th usb port.".to_iter_str(IterStrFormat::none()),
                            ]
                        );
                        *height += FONT_HEIGHT;
                        return Err(());
                    }
                } else {
                    match self.disconnected_port(index, services, height) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                }
            }
            PortPhase::EnablingSlot => {
                if !port_status.is_connected() {
                    match self.disconnected_port(index, services, height) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                }
            }
            PortPhase::HasSlot => {
                if !port_status.is_connected() {
                    match self.disconnected_port(index, services, height) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                }
            }
        }
        self.set_port_status(index, to_reset);
        Ok(())
    }

    pub fn reset_ports(&mut self, services: &Services, height: &mut u32) -> Result<(), ()> {
        let port_count = self.max_ports();
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                b"Maximum ".to_iter_str(IterStrFormat::none()),
                port_count.to_iter_str(IterStrFormat::none()),
                b" ports.".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        for i in 1..=port_count {
            match self.on_port_status_changed(i, services, height) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
        }
        Ok(())
    }

    fn max_ports(&self) -> u8 {
        get_unsigned_int_8s(
            self.capability_registers
                .host_controller_structural_parameters_1(),
        )
        .3
    }

    fn enable_slot(&mut self, services: &Services, height: &mut u32) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Enabling slot.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.command_ring.push(
            OutgoingTypedTransferRequestBlock::EnableSlotCommandTrb(EnableSlotCommandTrb::new())
                .into_transfer_request_block(),
        );

        self.doorbell_registers.set(0, 0);

        Ok(())
    }

    fn disable_slot(
        &mut self,
        slot_id: u8,
        services: &Services,
        height: &mut u32,
    ) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Enabling slot.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.command_ring.push(
            OutgoingTypedTransferRequestBlock::DisableSlotCommandTrb(DisableSlotCommandTrb::new(
                slot_id,
            ))
            .into_transfer_request_block(),
        );

        self.doorbell_registers.set(0, 0);

        Ok(())
    }

    pub fn process_events(&mut self, services: &Services, height: &mut u32) -> Result<(), ()> {
        'a: loop {
            match self.primary_interrupter_event_ring.pop() {
                Some(event) => {
                    let trb_type = event.trb_type();
                    match IncomingTypedTransferRequestBlock::from_transfer_request_block(event) {
                        Ok(trb) => match trb {
                            IncomingTypedTransferRequestBlock::TransferEventTrb(_) => todo!(),
                            IncomingTypedTransferRequestBlock::CommandCompletionEventTrb(trb) => {
                                match trb.command_completion_code() {
                                    COMMAND_COMPLETION_CODE_SUCCESS => {
                                        match unsafe {
                                            (trb.command_trb_pointer()
                                                as *const TransferRequestBlock)
                                                .read()
                                        }
                                        .trb_type()
                                        {
                                            TRB_TYPE_ID_ENABLE_SLOT_COMMAND => {
                                                let slot_id = trb.slot_id();
                                                match self.ports_queue_waiting_for_slot.pop() {
                                                    Some(port_id) => {
                                                        match output_string!(
                                                            services,
                                                            PixelColor::new(128, 0, 0),
                                                            Vector2::new(0, *height),
                                                            [
                                                                b"Successflly enabled "
                                                                    .to_iter_str(
                                                                        IterStrFormat::none()
                                                                    ),
                                                                slot_id.to_iter_str(
                                                                    IterStrFormat::none()
                                                                ),
                                                                b"-th device slot for "
                                                                    .to_iter_str(
                                                                        IterStrFormat::none()
                                                                    ),
                                                                port_id.to_iter_str(
                                                                    IterStrFormat::none()
                                                                ),
                                                                b"-th usb port.".to_iter_str(
                                                                    IterStrFormat::none()
                                                                ),
                                                            ]
                                                        ) {
                                                            Ok(()) => (),
                                                            Err(()) => return Err(()),
                                                        }
                                                        *height += FONT_HEIGHT;
                                                        self.port_id_of_slot
                                                            [slot_id as usize - 1] = Some(port_id);
                                                        let port_speed =
                                                            self.get_port_speed(port_id);
                                                        if self.is_context_size_64() {
                                                            let input_context = self
                                                                .input_contexts
                                                                .as_mut_64(slot_id as usize);
                                                            input_context
                                                                .set_enable_context(0, true);
                                                            input_context
                                                                .set_enable_context(1, true);
                                                            input_context.set_route_string(0);
                                                            input_context.set_speed(port_speed);
                                                            input_context.set_context_entries(1);
                                                            input_context
                                                                .set_route_hub_port_number(port_id);
                                                            self.device_context_base_address_array
                                                                .register_pointer(
                                                                    slot_id as usize,
                                                                    self.device_contexts
                                                                        .as_ptr_64(slot_id as usize)
                                                                        as u64,
                                                                );
                                                        } else {
                                                            self.device_context_base_address_array
                                                                .register_pointer(
                                                                    slot_id as usize,
                                                                    self.device_contexts
                                                                        .as_ptr_32(slot_id as usize)
                                                                        as u64,
                                                                );
                                                        }
                                                        if self.ports_queue_waiting_for_slot.count()
                                                            > 0
                                                        {
                                                            match self.enable_slot(services, height)
                                                            {
                                                                Ok(()) => (),
                                                                Err(()) => return Err(()),
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        match self
                                                            .disable_slot(slot_id, services, height)
                                                        {
                                                            Ok(()) => (),
                                                            Err(()) => return Err(()),
                                                        }
                                                        self.port_id_of_slot
                                                            [slot_id as usize - 1] = None;
                                                    }
                                                }
                                            }
                                            TRB_TYPE_ID_DISABLE_SLOT_COMMAND => {
                                                let slot_id = trb.slot_id();
                                                match output_string!(
                                                    services,
                                                    PixelColor::new(128, 0, 0),
                                                    Vector2::new(0, *height),
                                                    [
                                                        b"Successfully disabled "
                                                            .to_iter_str(IterStrFormat::none()),
                                                        slot_id.to_iter_str(IterStrFormat::none()),
                                                        b"-th device slot."
                                                            .to_iter_str(IterStrFormat::none()),
                                                    ]
                                                ) {
                                                    Ok(()) => (),
                                                    Err(()) => return Err(()),
                                                }
                                                *height += FONT_HEIGHT;
                                            }
                                            t => {
                                                _ = output_string!(
                                                    services,
                                                    PixelColor::new(128, 0, 0),
                                                    Vector2::new(0, *height),
                                                    [
                                                        b"Invalid command issuer type "
                                                            .to_iter_str(IterStrFormat::none()),
                                                        t.to_iter_str(IterStrFormat::none()),
                                                        b".".to_iter_str(IterStrFormat::none()),
                                                    ]
                                                );
                                                *height += FONT_HEIGHT;
                                                return Err(());
                                            }
                                        }
                                    }
                                    c => {
                                        _ = output_string!(
                                            services,
                                            PixelColor::new(128, 0, 0),
                                            Vector2::new(0, *height),
                                            [
                                                b"Command failed with code "
                                                    .to_iter_str(IterStrFormat::none()),
                                                c.to_iter_str(IterStrFormat::none()),
                                                b".".to_iter_str(IterStrFormat::none()),
                                            ]
                                        );
                                        *height += FONT_HEIGHT;
                                        return Err(());
                                    }
                                }
                            }
                            IncomingTypedTransferRequestBlock::PortStatusChangeEventTrb(trb) => {
                                let port_id = trb.port_id();
                                match self.on_port_status_changed(port_id, services, height) {
                                    Ok(()) => (),
                                    Err(()) => return Err(()),
                                }
                            }
                        },
                        Err(()) => {
                            match output_string!(
                                services,
                                PixelColor::new(128, 0, 0),
                                Vector2::new(0, *height),
                                [
                                    b"Unknown type ".to_iter_str(IterStrFormat::none()),
                                    trb_type.to_iter_str(IterStrFormat::none()),
                                    b".".to_iter_str(IterStrFormat::none()),
                                ]
                            ) {
                                Ok(()) => (),
                                Err(()) => return Err(()),
                            }
                            *height += FONT_HEIGHT;
                            //break 'a Err(())
                        }
                    }
                    match output_string!(
                        services,
                        PixelColor::new(128, 0, 0),
                        Vector2::new(0, *height),
                        [b"Event correctly processed.".to_iter_str(IterStrFormat::none()),]
                    ) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                    *height += FONT_HEIGHT;
                }
                None => {
                    self.primary_interrupter_event_ring.set_interrupt_pending();
                    match output_string!(
                        services,
                        PixelColor::new(128, 0, 0),
                        Vector2::new(0, *height),
                        [b"No further event found.".to_iter_str(IterStrFormat::none()),]
                    ) {
                        Ok(()) => (),
                        Err(()) => return Err(()),
                    }
                    *height += FONT_HEIGHT;
                    break 'a Ok(());
                }
            }
        }
    }

    fn disconnected_port(
        &mut self,
        port_id: u8,
        services: &Services,
        height: &mut u32,
    ) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                port_id.to_iter_str(IterStrFormat::none()),
                b"-th usb port disconnected.".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.ports_status[port_id as usize - 1] = PortPhase::NotConnected;
        Ok(())
    }
}

const USB_STATUS_HOST_CONTROLLER_HALTED_MASK: u32 = 0x0000_0001;
const USB_STATUS_CONTROLLER_NOT_READY: u32 = 0x0000_0800;
const USB_COMMAND_HOST_CONTROLLER_RESET_MASK: u32 = 0x0000_0002;

pub struct XhcCapabilityRegisters {
    base_address: u64,
}

impl XhcCapabilityRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn capability_register_length(&self) -> u8 {
        get_unsigned_int_8s(unsafe { ((self.base_address + 0x00) as *const u32).read() }).0
    }

    pub fn host_controller_structural_parameters_1(&self) -> u32 {
        unsafe { ((self.base_address + 0x04) as *const u32).read() }
    }

    pub fn host_controller_cabability_parameters_1(&self) -> u32 {
        unsafe { ((self.base_address + 0x10) as *const u32).read() }
    }

    pub fn doorbell_offset(&self) -> u32 {
        unsafe { ((self.base_address + 0x14) as *const u32).read() }
    }

    pub fn runtime_register_space_offset(&self) -> u32 {
        unsafe { ((self.base_address + 0x18) as *const u32).read() }
    }
}

pub struct XhcOperationalRegisters {
    base_address: u64,
}

impl XhcOperationalRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn usb_command_host_controller_reset(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFD) + 2;
    }

    pub fn usb_command_interrupter_enable(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFB) + 4;
    }

    pub fn usb_command_run(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFE) + 1;
    }

    pub fn usb_command_stop(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFE) + 0;
    }

    pub fn set_number_of_device_slots_enabled(&self, number: u8) {
        let configure_register = self.configure_register();
        *unsafe { ((self.base_address + 0x38) as *mut u32).as_mut() }.unwrap() =
            (configure_register & 0xFFFF_FF00) + number as u32;
    }

    pub fn set_device_context_base_address_array_pointer(&self, address: u64) {
        *unsafe { ((self.base_address + 0x30) as *mut u64).as_mut() }.unwrap() =
            address & 0xFFFF_FFFF_FFFF_FFC0
    }

    pub fn set_command_ring_control_register(&self, val: u64) {
        *unsafe { ((self.base_address + 0x18) as *mut u64).as_mut() }.unwrap() = val
    }

    pub fn set_port_status_and_control_register(&self, index: u8, val: u32) {
        *unsafe { ((self.base_address + 0x400 + 0x10 * (index - 1) as u64) as *mut u32).as_mut() }
            .unwrap() = val
    }

    pub fn usb_command(&self) -> u32 {
        unsafe { ((self.base_address + 0x00) as *const u32).read() }
    }

    pub fn usb_status(&self) -> u32 {
        unsafe { ((self.base_address + 0x04) as *const u32).read() }
    }

    pub fn command_ring_control_register(&self) -> u64 {
        unsafe { ((self.base_address + 0x18) as *const u64).read() }
    }

    pub fn configure_register(&self) -> u32 {
        unsafe { ((self.base_address + 0x38) as *const u32).read() }
    }

    pub fn port_status_and_control_register(&self, index: u8) -> u32 {
        unsafe { ((self.base_address + 0x400 + 0x10 * (index - 1) as u64) as *const u32).read() }
    }
}

pub struct XhcRuntimeRegisters {
    base_address: u64,
}

impl XhcRuntimeRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn get_interrupter_register_set(&self, index: u64) -> XhcInterrupterRegisterSet {
        XhcInterrupterRegisterSet::new(self.base_address + 0x20 + 32 * index)
    }
}

pub struct XhcDoorbellRegisters {
    base_address: u64,
}

impl XhcDoorbellRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn set(&self, index: u8, val: u32) {
        *unsafe { ((self.base_address + 4 * index as u64) as *mut u32).as_mut() }.unwrap() = val;
    }
}

pub struct XhcInterrupterRegisterSet {
    base_address: u64,
}

impl XhcInterrupterRegisterSet {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn interrupter_management_register(&self) -> u32 {
        unsafe { ((self.base_address + 0x00) as *const u32).read() }
    }

    pub fn event_ring_segment_table_size(&self) -> u32 {
        unsafe { ((self.base_address + 0x08) as *const u32).read() }
    }

    pub fn event_ring_segment_table_base_address(&self) -> u64 {
        unsafe { ((self.base_address + 0x10) as *const u64).read() }
    }

    pub fn event_ring_dequeue_pointer(&self) -> u64 {
        unsafe { ((self.base_address + 0x18) as *const u64).read() }
    }

    pub fn set_interrupt_enable(&self) {
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (self.interrupter_management_register() & 0xFFFF_FFFD) + 2;
    }

    pub fn set_interrupt_pending(&self) {
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (self.interrupter_management_register() & 0xFFFF_FFFE) + 1;
    }

    pub fn set_event_ring_segment_table_size(&self, size: u16) {
        *unsafe { ((self.base_address + 0x08) as *mut u32).as_mut() }.unwrap() =
            (size as u32) + (self.event_ring_segment_table_size() & 0xFFFF_0000);
    }

    pub fn set_event_ring_segment_table_base_address(&self, address: u64) {
        *unsafe { ((self.base_address + 0x10) as *mut u64).as_mut() }.unwrap() = (address
            & 0xFFFF_FFFF_FFFF_FFC0)
            + (self.event_ring_segment_table_base_address() & 0x3F);
    }

    pub fn set_event_ring_dequeue_pointer(&self, address: u64, index: u16) {
        *unsafe { ((self.base_address + 0x18) as *mut u64).as_mut() }.unwrap() = (address
            & 0xFFFF_FFFF_FFFF_FFF0)
            + (self.event_ring_dequeue_pointer() & 0x8)
            + (index as u64 & 0x7);
    }
}
