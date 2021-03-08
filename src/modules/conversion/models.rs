
#[derive(Debug)]
pub enum UnitType {
    MEASUREMENT,
    CURRENCY,
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub unit_type: UnitType,
}

impl Unit {
    pub fn new_currency(code : String, name : Option<String>, symbol : Option<String>) -> Self {
        let new_name : String = if name == None {String::from(code.as_str())} else {name.unwrap()};
        let new_symbol : String = if symbol == None {String::from(code.as_str())} else {symbol.unwrap()};

        Unit {
            name:      new_name,
            code:      String::from(code.as_str()),
            symbol:    new_symbol,
            unit_type: UnitType::CURRENCY,
        }
    }

    pub fn celsius() -> Self {
        Unit {
            name: String::from("celsius"),
            code: String::from("c"),
            symbol: String::from("°C"),
            unit_type: UnitType::MEASUREMENT,
        }
    }

    pub fn fahrenheit() -> Self {
        Unit {
            name: String::from("fahrenheit"),
            code: String::from("f"),
            symbol: String::from("°F"),
            unit_type: UnitType::MEASUREMENT,
        }
    }
}

#[derive(Debug)]
pub struct Conversion {
    pub unit: Unit,
    pub value: f64,
}

impl Conversion {
    pub fn to_string(&self) -> String {
        format!("{}{}", self.value, self.unit.symbol)
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
