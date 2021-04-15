#[derive(Debug, Clone)]
pub enum UnitType {
    MEASUREMENT,
    CURRENCY,
}

#[derive(Debug, Clone)]
pub enum UnitSubType {
    TEMPERATURE,
    LENGTH,
}

impl UnitSubType {
    pub fn to_string(&self) -> String {
        String::from(match self {
            UnitSubType::LENGTH => "length",
            UnitSubType::TEMPERATURE => "temperature",
        })
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub unit_type: UnitType,
    pub subtype: Option<UnitSubType>,
}

impl Unit {
    pub fn new(
        name: &'static str,
        code: &'static str,
        symbol: &'static str,
        unit_type: UnitType,
        subtype: Option<UnitSubType>,
    ) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            symbol: symbol.to_string(),
            unit_type,
            subtype: subtype,
        }
    }

    pub fn new_measurement(
        name: &'static str,
        code: &'static str,
        symbol: &'static str,
        subtype: UnitSubType,
    ) -> Self {
        Self::new(name, code, symbol, UnitType::MEASUREMENT, Some(subtype))
    }

    pub fn new_currency(code: String, name: Option<String>, symbol: Option<String>) -> Self {
        Unit {
            name: name.unwrap_or(code.clone()),
            symbol: symbol.unwrap_or(code.clone()),
            code,
            unit_type: UnitType::CURRENCY,
            subtype: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conversion {
    pub unit: Unit,
    pub value: f64,
}

impl Conversion {
    pub fn to_string(&self) -> String {
        match self.unit.unit_type {
            UnitType::CURRENCY => {
                format!("{}{}", self.value, self.unit.code)
            }
            UnitType::MEASUREMENT => {
                format!("{}{}", self.value, self.unit.symbol)
            }
        }
    }
}

pub struct ConversionResult {
    pub base: Conversion,
    pub to: Vec<Conversion>,
}

impl ConversionResult {
    pub fn new(base: Conversion) -> ConversionResult {
        ConversionResult {
            base: base,
            to: Vec::new(),
        }
    }
    pub fn new_with_to(base: Conversion, to: Vec<Conversion>) -> ConversionResult {
        ConversionResult { base: base, to: to }
    }
}
