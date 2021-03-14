#[derive(Debug, Clone)]
pub enum UnitType {
    MEASUREMENT,
    CURRENCY,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub name: &'static str,
    pub code: &'static str,
    pub symbol: &'static str,
    pub unit_type: UnitType,
}

impl Unit {
    pub fn new(
        name: &'static str,
        code: &'static str,
        symbol: &'static str,
        unit_type: UnitType,
    ) -> Self {
        Self {
            name,
            code,
            symbol,
            unit_type,
        }
    }

    pub fn new_currency(
        code: &'static str,
        name: Option<&'static str>,
        symbol: Option<&'static str>,
    ) -> Self {
        let new_name: &'static str = if name == None { code } else { name.unwrap() };
        let new_symbol: &'static str = if symbol == None {
            code
        } else {
            symbol.unwrap()
        };

        Unit {
            name: new_name,
            code: code,
            symbol: new_symbol,
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
            },
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
