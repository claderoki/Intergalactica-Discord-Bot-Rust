#[derive(Debug, Clone)]
pub enum UnitType {
    MEASUREMENT,
    CURRENCY,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub unit_type: UnitType,
}

impl Unit {
    pub fn new(name: &'static str, code: &'static str, symbol: &'static str, unit_type: UnitType) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            symbol: symbol.to_string(),
            unit_type,
        }
    }

    pub fn new_measurement(name: &'static str, code: &'static str, symbol: &'static str) -> Self {
        Self::new(name, code, symbol, UnitType::MEASUREMENT)
    }

    pub fn new_currency(code: String, name: Option<String>, symbol: Option<String>) -> Self {
        Unit {
            name: name.unwrap_or(code.clone()),
            symbol: symbol.unwrap_or(code.clone()),
            code,
            unit_type: UnitType::CURRENCY,
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
}
