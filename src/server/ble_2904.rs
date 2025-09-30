use alloc::sync::Arc;

use crate::{utilities::mutex::Mutex, BLEDescriptor};

#[repr(u8)]
pub enum BLE2904Format {
  BOOLEAN = 1,
  UINT2 = 2,
  UINT4 = 3,
  UINT8 = 4,
  UINT12 = 5,
  UINT16 = 6,
  UINT24 = 7,
  UINT32 = 8,
  UINT48 = 9,
  UINT64 = 10,
  UINT128 = 11,
  SINT8 = 12,
  SINT12 = 13,
  SINT16 = 14,
  SINT24 = 15,
  SINT32 = 16,
  SINT48 = 17,
  SINT64 = 18,
  SINT128 = 19,
  FLOAT32 = 20,
  FLOAT64 = 21,
  SFLOAT16 = 22,
  SFLOAT32 = 23,
  IEEE20601 = 24,
  UTF8 = 25,
  UTF16 = 26,
  OPAQUE = 27,
}

#[repr(packed)]
struct Data {
  format: BLE2904Format,
  exponent: u8,
  unit: u16,
  namespace: u8,
  description: u16,
}

pub struct BLE2904 {
  descriptor: Arc<Mutex<BLEDescriptor>>,
}

impl BLE2904 {
  pub(super) fn new(descriptor: Arc<Mutex<BLEDescriptor>>) -> Self {
    descriptor.lock().set_from(&Data {
      format: BLE2904Format::OPAQUE,
      exponent: 0,
      unit: 0,
      namespace: 1,
      description: 0,
    });

    Self { descriptor }
  }

  pub fn format(&mut self, value: BLE2904Format) -> &mut Self {
    self
      .descriptor
      .lock()
      .value_mut()
      .as_mut::<Data>()
      .unwrap()
      .format = value;

    self
  }

  pub fn exponent(&mut self, value: u8) -> &mut Self {
    self
      .descriptor
      .lock()
      .value_mut()
      .as_mut::<Data>()
      .unwrap()
      .exponent = value;

    self
  }

  pub fn unit(&mut self, value: u16) -> &mut Self {
    self
      .descriptor
      .lock()
      .value_mut()
      .as_mut::<Data>()
      .unwrap()
      .unit = value;

    self
  }

  pub fn namespace(&mut self, value: u8) -> &mut Self {
    self
      .descriptor
      .lock()
      .value_mut()
      .as_mut::<Data>()
      .unwrap()
      .namespace = value;

    self
  }
  pub fn description(&mut self, value: u16) -> &mut Self {
    self
      .descriptor
      .lock()
      .value_mut()
      .as_mut::<Data>()
      .unwrap()
      .description = value;

    self
  }
}
