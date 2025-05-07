pub mod code;

pub enum Naics {
	Agriculture = 11,
	Mining = 21,
	Utilities = 22,
	Construction = 23,
	Manufacturing31 = 31,
	Manufacturing32 = 32,
	Manufacturing33 = 33,
	WholesaleTrade = 42,
	RetailTrade44 = 44,
	RetailTrade45 = 45,
	Transportation48 = 48,
	Transportation49 = 49,
	Information = 51,
	Finance = 52,
	RealEstate = 53,
	ProfessionalServices = 54,
	Management = 55,
	AdminSupport = 56,
	Education = 61,
	Healthcare = 62,
	Arts = 71,
	Accommodation = 72,
	OtherServices = 81,
	PublicAdministration = 92,
}

const NAICS_2DIGIT_CODES: [&str; 24] = [
    "11", "21", "22", "23",
    "31", "32", "33",
    "42", "44", "45",
    "48", "49",
    "51", "52", "53", "54", "55", "56",
    "61", "62", "71", "72", "81", "92",
];