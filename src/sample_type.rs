#[derive(Debug, Clone, Copy)]
pub enum SampleType {
    I16(i16),
    F32(f32),
}

impl From<i16> for SampleType {
    fn from(s: i16) -> Self {
        SampleType::I16(s)
    }
}

impl From<f32> for SampleType {
    fn from(s: f32) -> Self {
        SampleType::F32(s)
    }
}

impl Into<i16> for SampleType {
    fn into(self) -> i16 {
        match self {
            SampleType::I16(s) => s,
            SampleType::F32(_) => panic!("Cannot convert SampleNumber::F32 to i16"),
        }
    }
}

impl Into<f32> for SampleType {
    fn into(self) -> f32 {
        match self {
            SampleType::I16(_) => panic!("Cannot convert SampleNumber::I16 to f32"),
            SampleType::F32(s) => s,
        }
    }
}
