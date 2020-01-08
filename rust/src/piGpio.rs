use gpio::dummy::DummyGpioOut;
use gpio::sysfs::SysFsGpioInput;
use gpio::GpioOut;

pub struct Pins<P> {
  logging_pins: Vec<P>,
}
// pub struct Pins{
//     logging_pins: Vec<Box<dyn gpio::GpioOut<Error = ()>>>,
//     //logging_pins: Box<dyn gpio::GpioOut>
// }
impl<P> Pins<P>
where
  P: gpio::GpioOut<Error = ()>,
{
  // methods go here
}

pub fn testing() {
  let mut pinstruct = Pins {
    logging_pins: (0..26)
      .map(|pin_num| DummyGpioOut::new(move |value| println!("pin {} set to {:?}", pin_num, value)))
      .collect(),
  };

  let _ = pinstruct.logging_pins[3].set_high();

  //let mut logging_pinsreal: Vec<_> = (0..32).map(|pin_num| {
  //    SysFsGpioInput::open(pin_num).unwrap()
  //    //DummyGpioOut::new(|value| println!("pin {} set to {:?}", pin_num, value))
  //  }).collect();

  let mut logging_pins: Vec<_> = (0..26)
    .map(|pin_num| DummyGpioOut::new(move |value| println!("pin {} set to {:?}", pin_num, value)))
    .collect();

  let _ = logging_pins[3].set_high();
  //logging_pins[3].set_value(GpioValue::High);
  let _ = logging_pins[3].set_low();

  let mut dg = DummyGpioOut::new(|_| ());
  //let mut dg = GpioOut::
  let _ = dg.set_value(true);
  //let mut gpio23 = SysFsGpioInput::open(23).unwrap();
}
