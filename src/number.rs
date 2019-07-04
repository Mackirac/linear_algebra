pub trait Number {
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait Integer : Number {
    type Exp;
    fn pow(self, exp: Self::Exp) -> Self;
}

pub trait Float : Number {
    type IExp;
    type FExp;
    fn powi(self, exp: Self::IExp) -> Self;
    fn powf(self, exp: Self::FExp) -> Self;
    fn sqrt(self) -> Self;
}

//////////////////////////////////////////////////////////////////////////////////////
/*                     TRAITS IMPLEMENTATIONS ON STANDARD TYPES                     */
//////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////
// UNSIGNED INTEGER NUMBER TYPES //
///////////////////////////////////

impl Number for u8 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for u8 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for u16 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for u16 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for u32 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for u64 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for u128 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for u128 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for usize {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for usize {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

/////////////////////////////////
// SIGNED INTEGER NUMBER TYPES //
/////////////////////////////////

impl Number for i8 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for i8 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for i16 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for i16 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for i32 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for i64 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for i128 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for i128 {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

impl Number for isize {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
impl Integer for isize {
    type Exp = u32;
    fn pow(self, exp: Self::Exp) -> Self { self.pow(exp) }
}

/////////////////////////////////
// FLOATING POINT NUMBER TYPES //
/////////////////////////////////

impl Number for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}
impl Float for f32 {
    type IExp = i32;
    type FExp = Self;
    fn powi(self, exp: Self::IExp) -> Self { self.powi(exp) }
    fn powf(self, exp: Self::FExp) -> Self { self.powf(exp) }
    fn sqrt(self) -> Self { self.sqrt() }
}

impl Number for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}
impl Float for f64 {
    type IExp = i32;
    type FExp = Self;
    fn powi(self, exp: Self::IExp) -> Self { self.powi(exp) }
    fn powf(self, exp: Self::FExp) -> Self { self.powf(exp) }
    fn sqrt(self) -> Self { self.sqrt() }
}
