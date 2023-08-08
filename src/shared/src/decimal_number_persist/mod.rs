static DECIMAL_POINTS: u32 = 3;
static DECIMAL_MODIFIER: u32 = (10 as u32).pow(DECIMAL_POINTS);

pub struct DecimalNumberWrapper {
  pub whole: u32,
  pub fractional: u32,
}

impl DecimalNumberWrapper {
  pub fn new(whole: u32, fractional: u32) -> Self {
    let res = DecimalNumberWrapper { whole, fractional };
    res.validate()
  }

  fn validate(self) -> Self {
    if self.fractional >= DECIMAL_MODIFIER {
      panic!("franctional number exceeded the limit")
    }
    self
  }

  fn prepare_number(number: DecimalNumberWrapper) -> u32 {
    number.whole * DECIMAL_MODIFIER + number.fractional
  }

  pub fn add(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> DecimalNumberWrapper {
    let result = DecimalNumberWrapper::prepare_number(a.validate())
      + DecimalNumberWrapper::prepare_number(b.validate());
    let whole = result / DECIMAL_MODIFIER;

    DecimalNumberWrapper {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  pub fn mul(a: DecimalNumberWrapper, b: DecimalNumberWrapper) -> DecimalNumberWrapper {
    let result = DecimalNumberWrapper::prepare_number(a.validate())
      * DecimalNumberWrapper::prepare_number(b.validate());
    let result = result / DECIMAL_MODIFIER;
    let whole = result / DECIMAL_MODIFIER;
    DecimalNumberWrapper {
      whole,
      fractional: result - (whole * DECIMAL_MODIFIER),
    }
    .validate()
  }

  pub fn as_tuple(&self) -> (u32, u32) {
    (self.whole, self.fractional)
  }
}

impl From<(u32, u32)> for DecimalNumberWrapper {
  fn from(value: (u32, u32)) -> Self {
    DecimalNumberWrapper {
      whole: value.0,
      fractional: value.1,
    }
  }
}

#[cfg(test)]
mod test;
