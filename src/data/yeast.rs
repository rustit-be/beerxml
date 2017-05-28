// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::result;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Yeast {
    // name of the hop
    #[serde(skip)]
    pub name:String,
    /// version of the yeast format (normally 1)
    pub version:i64,
    /// yeast type
    #[serde(rename="type")]
    pub type_:YeastType,
    /// yeast form
    pub form:YeastForm,
    /// amount (liter or kg)
    pub amount: f64,
    /// if amount is in kg
    pub amount_is_weight: bool,
    /// name of the producer
    pub laboratory: Option<String>,
    /// manufacturer product id
    pub product_id: Option<String>,
    /// minimum recommended temperature for fermenting this yeast strain in degrees Celsius
    pub min_temperature: Option<f64>,
    /// maximum recommended temperature for fermenting this yeast strain in Celsius
    pub max_temperature: Option<f64>,
    /// yeast flocculation
    pub flocculation: Option<YeastFlocculation>,
    /// attenuation of the yeast in percent
    pub attenuation: Option<f64>,
    /// notes
    pub notes: Option<String>,
    /// styles or types of beer this yeast strain is best suited for
    pub best_for: Option<String>,
    /// number of times this yeast has been reused as a harvested culture.  This number should be zero if this is a product directly from the manufacturer
    pub times_cultured: Option<i64>,
    /// recommended of times this yeast can be reused (recultured from a previous batch)
    pub max_reuse: Option<i64>,
    /// flag denoting that this yeast was added for a secondary (or later) fermentation as opposed to the primary fermentation.  Useful if one uses two or more yeast strains for a single brew (eg: Lambic).  Default value is false
    pub add_to_secondary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum YeastType {
    Ale,
    Lager,
    Wheat,
    Wine,
    Champagne,
}

impl Default for YeastType {
    fn default() -> YeastType {
        YeastType::Ale
    }
}

impl fmt::Display for YeastType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastType::Ale => "Ale",
            YeastType::Lager => "Lager",
            YeastType::Wheat => "Wheat",
            YeastType::Wine => "Wine",
            YeastType::Champagne => "Champagne",
        };
        write!(f, "{}", x)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum YeastForm {
    Liquid,
    Dry,
    Slate,
    Culture,
}

impl Default for YeastForm {
    fn default() -> YeastForm {
        YeastForm::Liquid
    }
}
impl fmt::Display for YeastForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastForm::Liquid => "Liquid",
            YeastForm::Dry => "Dry",
            YeastForm::Slate => "Slate",
            YeastForm::Culture => "Culture",
        };
        write!(f, "{}", x)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum YeastFlocculation {
    Low,
    Medium,
    High,
    #[serde(rename = "Very High")]
    VeryHigh,
}

impl Default for YeastFlocculation {
    fn default() -> YeastFlocculation {
        YeastFlocculation::Low
    }
}

impl fmt::Display for YeastFlocculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastFlocculation::Low => "Low",
            YeastFlocculation::Medium => "Medium",
            YeastFlocculation::High => "High",
            YeastFlocculation::VeryHigh => "Very High",
        };
        write!(f, "{}", x)
    }
}