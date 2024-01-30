/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2021, 2022, 2023  Dirk Stolle
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 -------------------------------------------------------------------------------
*/

use crate::data::Country;

/// Holds all known countries.
pub struct World
{
  all_countries: Vec<Country>
}

impl World
{
  /**
   * Creates a new instance.
   */
  pub fn new() -> World
  {
    World {
      all_countries: vec![
        Country {
          country_id: 1,
          name: "Afghanistan".to_string(),
          population: 38041757,
          geo_id: "AF".to_string(),
          country_code: "AFG".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 2,
          name: "Albania".to_string(),
          population: 2862427,
          geo_id: "AL".to_string(),
          country_code: "ALB".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 3,
          name: "Algeria".to_string(),
          population: 43053054,
          geo_id: "DZ".to_string(),
          country_code: "DZA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 4,
          name: "American Samoa".to_string(),
          population: 55197,
          geo_id: "AS".to_string(),
          country_code: "ASM".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 5,
          name: "Andorra".to_string(),
          population: 76177,
          geo_id: "AD".to_string(),
          country_code: "AND".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 6,
          name: "Angola".to_string(),
          population: 31825299,
          geo_id: "AO".to_string(),
          country_code: "AGO".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 7,
          name: "Anguilla".to_string(),
          population: 14872,
          geo_id: "AI".to_string(),
          country_code: "AIA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 8,
          name: "Antigua and Barbuda".to_string(),
          population: 97115,
          geo_id: "AG".to_string(),
          country_code: "ATG".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 9,
          name: "Argentina".to_string(),
          population: 44780675,
          geo_id: "AR".to_string(),
          country_code: "ARG".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 10,
          name: "Armenia".to_string(),
          population: 2957728,
          geo_id: "AM".to_string(),
          country_code: "ARM".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 11,
          name: "Aruba".to_string(),
          population: 106310,
          geo_id: "AW".to_string(),
          country_code: "ABW".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 12,
          name: "Australia".to_string(),
          population: 25203200,
          geo_id: "AU".to_string(),
          country_code: "AUS".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 13,
          name: "Austria".to_string(),
          population: 8858775,
          geo_id: "AT".to_string(),
          country_code: "AUT".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 14,
          name: "Azerbaijan".to_string(),
          population: 10047719,
          geo_id: "AZ".to_string(),
          country_code: "AZE".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 15,
          name: "Bahamas".to_string(),
          population: 389486,
          geo_id: "BS".to_string(),
          country_code: "BHS".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 16,
          name: "Bahrain".to_string(),
          population: 1641164,
          geo_id: "BH".to_string(),
          country_code: "BHR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 17,
          name: "Bangladesh".to_string(),
          population: 163046173,
          geo_id: "BD".to_string(),
          country_code: "BGD".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 18,
          name: "Barbados".to_string(),
          population: 287021,
          geo_id: "BB".to_string(),
          country_code: "BRB".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 19,
          name: "Belarus".to_string(),
          population: 9452409,
          geo_id: "BY".to_string(),
          country_code: "BLR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 20,
          name: "Belgium".to_string(),
          population: 11455519,
          geo_id: "BE".to_string(),
          country_code: "BEL".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 21,
          name: "Belize".to_string(),
          population: 390351,
          geo_id: "BZ".to_string(),
          country_code: "BLZ".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 22,
          name: "Benin".to_string(),
          population: 11801151,
          geo_id: "BJ".to_string(),
          country_code: "BEN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 23,
          name: "Bermuda".to_string(),
          population: 62508,
          geo_id: "BM".to_string(),
          country_code: "BMU".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 24,
          name: "Bhutan".to_string(),
          population: 763094,
          geo_id: "BT".to_string(),
          country_code: "BTN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 25,
          name: "Bolivia".to_string(),
          population: 11513102,
          geo_id: "BO".to_string(),
          country_code: "BOL".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 26,
          name: "Bonaire, Saint Eustatius and Saba".to_string(),
          population: 25983,
          geo_id: "BQ".to_string(),
          country_code: "BES".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 26,
          name: "Bonaire".to_string(),
          population: 19179,
          geo_id: "XA".to_string(),
          country_code: "BES".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 27,
          name: "Bosnia and Herzegovina".to_string(),
          population: 3300998,
          geo_id: "BA".to_string(),
          country_code: "BIH".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 28,
          name: "Botswana".to_string(),
          population: 2303703,
          geo_id: "BW".to_string(),
          country_code: "BWA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 29,
          name: "Brazil".to_string(),
          population: 211049519,
          geo_id: "BR".to_string(),
          country_code: "BRA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 30,
          name: "British Virgin Islands".to_string(),
          population: 30033,
          geo_id: "VG".to_string(),
          country_code: "VGB".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 31,
          name: "Brunei Darussalam".to_string(),
          population: 433296,
          geo_id: "BN".to_string(),
          country_code: "BRN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 32,
          name: "Bulgaria".to_string(),
          population: 7000039,
          geo_id: "BG".to_string(),
          country_code: "BGR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 33,
          name: "Burkina Faso".to_string(),
          population: 20321383,
          geo_id: "BF".to_string(),
          country_code: "BFA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 34,
          name: "Burundi".to_string(),
          population: 11530577,
          geo_id: "BI".to_string(),
          country_code: "BDI".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 35,
          name: "Cabo Verde".to_string(),
          population: 549936,
          geo_id: "CV".to_string(),
          country_code: "CPV".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 36,
          name: "Cambodia".to_string(),
          population: 16486542,
          geo_id: "KH".to_string(),
          country_code: "KHM".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 37,
          name: "Cameroon".to_string(),
          population: 25876387,
          geo_id: "CM".to_string(),
          country_code: "CMR".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 38,
          name: "Canada".to_string(),
          population: 37411038,
          geo_id: "CA".to_string(),
          country_code: "CAN".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 35,
          name: "Cape Verde".to_string(),
          population: 549936,
          geo_id: "CV".to_string(),
          country_code: "CPV".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 38,
          name: "Cases on an international conveyance Japan".to_string(),
          population: -1,
          geo_id: "JPG11668".to_string(),
          country_code: "".to_string(),
          continent: "Other".to_string()
        },
        Country {
          country_id: 39,
          name: "Cayman Islands".to_string(),
          population: 64948,
          geo_id: "KY".to_string(),
          country_code: "CYM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 40,
          name: "Central African Republic".to_string(),
          population: 4745179,
          geo_id: "CF".to_string(),
          country_code: "CAF".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 41,
          name: "Chad".to_string(),
          population: 15946882,
          geo_id: "TD".to_string(),
          country_code: "TCD".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 42,
          name: "Chile".to_string(),
          population: 18952035,
          geo_id: "CL".to_string(),
          country_code: "CHL".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 43,
          name: "China".to_string(),
          population: 1433783692,
          geo_id: "CN".to_string(),
          country_code: "CHN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 44,
          name: "Colombia".to_string(),
          population: 50339443,
          geo_id: "CO".to_string(),
          country_code: "COL".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 45,
          name: "Comoros".to_string(),
          population: 850891,
          geo_id: "KM".to_string(),
          country_code: "COM".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 46,
          name: "Congo".to_string(),
          population: 5380504,
          geo_id: "CG".to_string(),
          country_code: "COG".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 47,
          name: "Cook Islands".to_string(),
          population: 17_459,
          geo_id: "CK".to_string(),
          country_code: "COK".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 48,
          name: "Costa Rica".to_string(),
          population: 5047561,
          geo_id: "CR".to_string(),
          country_code: "CRI".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 49,
          name: "Côte d’Ivoire".to_string(),
          population: 25716554,
          geo_id: "CI".to_string(),
          country_code: "CIV".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 49,
          name: "Cote d'Ivoire".to_string(),
          population: 25716554,
          geo_id: "CI".to_string(),
          country_code: "CIV".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 50,
          name: "Croatia".to_string(),
          population: 4076246,
          geo_id: "HR".to_string(),
          country_code: "HRV".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 51,
          name: "Cuba".to_string(),
          population: 11333484,
          geo_id: "CU".to_string(),
          country_code: "CUB".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 52,
          name: "Curaçao".to_string(),
          population: 163423,
          geo_id: "CW".to_string(),
          country_code: "CUW".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 53,
          name: "Cyprus".to_string(),
          population: 875899,
          geo_id: "CY".to_string(),
          country_code: "CYP".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 54,
          name: "Czechia".to_string(),
          population: 10649800,
          geo_id: "CZ".to_string(),
          country_code: "CZE".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 55,
          name: "Democratic People's Republic of Korea".to_string(),
          population: 25549604,
          geo_id: "KP".to_string(),
          country_code: "PRK".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 56,
          name: "Democratic Republic of the Congo".to_string(),
          population: 86790568,
          geo_id: "CD".to_string(),
          country_code: "COD".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 57,
          name: "Denmark".to_string(),
          population: 5806081,
          geo_id: "DK".to_string(),
          country_code: "DNK".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 58,
          name: "Djibouti".to_string(),
          population: 973557,
          geo_id: "DJ".to_string(),
          country_code: "DJI".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 59,
          name: "Dominica".to_string(),
          population: 71808,
          geo_id: "DM".to_string(),
          country_code: "DMA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 60,
          name: "Dominican Republic".to_string(),
          population: 10738957,
          geo_id: "DO".to_string(),
          country_code: "DOM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 61,
          name: "Ecuador".to_string(),
          population: 17373657,
          geo_id: "EC".to_string(),
          country_code: "ECU".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 62,
          name: "Egypt".to_string(),
          population: 100388076,
          geo_id: "EG".to_string(),
          country_code: "EGY".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 63,
          name: "El Salvador".to_string(),
          population: 6453550,
          geo_id: "SV".to_string(),
          country_code: "SLV".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 64,
          name: "Equatorial Guinea".to_string(),
          population: 1355982,
          geo_id: "GQ".to_string(),
          country_code: "GNQ".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 65,
          name: "Eritrea".to_string(),
          population: 3497117,
          geo_id: "ER".to_string(),
          country_code: "ERI".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 66,
          name: "Estonia".to_string(),
          population: 1324820,
          geo_id: "EE".to_string(),
          country_code: "EST".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 67,
          name: "Eswatini".to_string(),
          population: 1148133,
          geo_id: "SZ".to_string(),
          country_code: "SWZ".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 68,
          name: "Ethiopia".to_string(),
          population: 112078727,
          geo_id: "ET".to_string(),
          country_code: "ETH".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 69,
          name: "Falkland Islands (Malvinas)".to_string(),
          population: 3372,
          geo_id: "FK".to_string(),
          country_code: "FLK".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 70,
          name: "Faroe Islands".to_string(),
          population: 48677,
          geo_id: "FO".to_string(),
          country_code: "FRO".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 71,
          name: "Fiji".to_string(),
          population: 889955,
          geo_id: "FJ".to_string(),
          country_code: "FJI".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 72,
          name: "Finland".to_string(),
          population: 5517919,
          geo_id: "FI".to_string(),
          country_code: "FIN".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 73,
          name: "France".to_string(),
          population: 67012883,
          geo_id: "FR".to_string(),
          country_code: "FRA".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 74,
          name: "French Guiana".to_string(),
          population: 304557,
          geo_id: "GF".to_string(),
          country_code: "GUF".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 75,
          name: "French Polynesia".to_string(),
          population: 279285,
          geo_id: "PF".to_string(),
          country_code: "PYF".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 76,
          name: "Gabon".to_string(),
          population: 2172578,
          geo_id: "GA".to_string(),
          country_code: "GAB".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 77,
          name: "Gambia".to_string(),
          population: 2347696,
          geo_id: "GM".to_string(),
          country_code: "GMB".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 78,
          name: "Georgia".to_string(),
          population: 3996762,
          geo_id: "GE".to_string(),
          country_code: "GEO".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 79,
          name: "Germany".to_string(),
          population: 83019213,
          geo_id: "DE".to_string(),
          country_code: "DEU".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 80,
          name: "Ghana".to_string(),
          population: 30417858,
          geo_id: "GH".to_string(),
          country_code: "GHA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 81,
          name: "Gibraltar".to_string(),
          population: 33706,
          geo_id: "GI".to_string(),
          country_code: "GIB".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 82,
          name: "Greece".to_string(),
          population: 10724599,
          geo_id: "GR".to_string(),
          country_code: "GRC".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 83,
          name: "Greenland".to_string(),
          population: 56660,
          geo_id: "GL".to_string(),
          country_code: "GRL".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 84,
          name: "Grenada".to_string(),
          population: 112002,
          geo_id: "GD".to_string(),
          country_code: "GRD".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 85,
          name: "Guadeloupe".to_string(),
          population: 395752,
          geo_id: "GP".to_string(),
          country_code: "GLP".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 86,
          name: "Guam".to_string(),
          population: 167295,
          geo_id: "GU".to_string(),
          country_code: "GUM".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 87,
          name: "Guatemala".to_string(),
          population: 17581476,
          geo_id: "GT".to_string(),
          country_code: "GTM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 88,
          name: "Guernsey".to_string(),
          population: 64468,
          geo_id: "GG".to_string(),
          country_code: "GGY".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 89,
          name: "Guinea".to_string(),
          population: 12771246,
          geo_id: "GN".to_string(),
          country_code: "GIN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 90,
          name: "Guinea Bissau".to_string(),
          population: 1920917,
          geo_id: "GW".to_string(),
          country_code: "GNB".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 91,
          name: "Guyana".to_string(),
          population: 782775,
          geo_id: "GY".to_string(),
          country_code: "GUY".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 92,
          name: "Haiti".to_string(),
          population: 11263079,
          geo_id: "HT".to_string(),
          country_code: "HTI".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 93,
          name: "Holy See".to_string(),
          population: 815,
          geo_id: "VA".to_string(),
          country_code: "VAT".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 94,
          name: "Honduras".to_string(),
          population: 9746115,
          geo_id: "HN".to_string(),
          country_code: "HND".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 95,
          name: "Hungary".to_string(),
          population: 9772756,
          geo_id: "HU".to_string(),
          country_code: "HUN".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 96,
          name: "Iceland".to_string(),
          population: 356991,
          geo_id: "IS".to_string(),
          country_code: "ISL".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 97,
          name: "India".to_string(),
          population: 1366417756,
          geo_id: "IN".to_string(),
          country_code: "IND".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 98,
          name: "Indonesia".to_string(),
          population: 270625567,
          geo_id: "ID".to_string(),
          country_code: "IDN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 99,
          name: "Iran".to_string(),
          population: 82913893,
          geo_id: "IR".to_string(),
          country_code: "IRN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 100,
          name: "Iraq".to_string(),
          population: 39309789,
          geo_id: "IQ".to_string(),
          country_code: "IRQ".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 101,
          name: "Ireland".to_string(),
          population: 4904240,
          geo_id: "IE".to_string(),
          country_code: "IRL".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 102,
          name: "Isle of Man".to_string(),
          population: 84589,
          geo_id: "IM".to_string(),
          country_code: "IMN".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 103,
          name: "Israel".to_string(),
          population: 8519373,
          geo_id: "IL".to_string(),
          country_code: "ISR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 104,
          name: "Italy".to_string(),
          population: 60359546,
          geo_id: "IT".to_string(),
          country_code: "ITA".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 105,
          name: "Jamaica".to_string(),
          population: 2948277,
          geo_id: "JM".to_string(),
          country_code: "JAM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 106,
          name: "Japan".to_string(),
          population: 126860299,
          geo_id: "JP".to_string(),
          country_code: "JPN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 107,
          name: "Jersey".to_string(),
          population: 107796,
          geo_id: "JE".to_string(),
          country_code: "JEY".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 108,
          name: "Jordan".to_string(),
          population: 10101697,
          geo_id: "JO".to_string(),
          country_code: "JOR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 109,
          name: "Kazakhstan".to_string(),
          population: 18551428,
          geo_id: "KZ".to_string(),
          country_code: "KAZ".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 110,
          name: "Kenya".to_string(),
          population: 52573967,
          geo_id: "KE".to_string(),
          country_code: "KEN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 111,
          name: "Kiribati".to_string(),
          population: 119940,
          geo_id: "KI".to_string(),
          country_code: "KIR".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 112,
          name: "Kosovo".to_string(),
          population: 1798506,
          geo_id: "XK".to_string(),
          country_code: "XKX".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 113,
          name: "Kuwait".to_string(),
          population: 4207077,
          geo_id: "KW".to_string(),
          country_code: "KWT".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 114,
          name: "Kyrgyzstan".to_string(),
          population: 6415851,
          geo_id: "KG".to_string(),
          country_code: "KGZ".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 115,
          name: "Laos".to_string(),
          population: 7169456,
          geo_id: "LA".to_string(),
          country_code: "LAO".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 116,
          name: "Latvia".to_string(),
          population: 1919968,
          geo_id: "LV".to_string(),
          country_code: "LVA".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 117,
          name: "Lebanon".to_string(),
          population: 6855709,
          geo_id: "LB".to_string(),
          country_code: "LBN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 118,
          name: "Lesotho".to_string(),
          population: 2125267,
          geo_id: "LS".to_string(),
          country_code: "LSO".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 119,
          name: "Liberia".to_string(),
          population: 4937374,
          geo_id: "LR".to_string(),
          country_code: "LBR".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 120,
          name: "Libya".to_string(),
          population: 6777453,
          geo_id: "LY".to_string(),
          country_code: "LBY".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 121,
          name: "Liechtenstein".to_string(),
          population: 38378,
          geo_id: "LI".to_string(),
          country_code: "LIE".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 122,
          name: "Lithuania".to_string(),
          population: 2794184,
          geo_id: "LT".to_string(),
          country_code: "LTU".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 123,
          name: "Luxembourg".to_string(),
          population: 613894,
          geo_id: "LU".to_string(),
          country_code: "LUX".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 124,
          name: "Madagascar".to_string(),
          population: 26969306,
          geo_id: "MG".to_string(),
          country_code: "MDG".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 125,
          name: "Malawi".to_string(),
          population: 18628749,
          geo_id: "MW".to_string(),
          country_code: "MWI".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 126,
          name: "Malaysia".to_string(),
          population: 31949789,
          geo_id: "MY".to_string(),
          country_code: "MYS".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 127,
          name: "Maldives".to_string(),
          population: 530957,
          geo_id: "MV".to_string(),
          country_code: "MDV".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 128,
          name: "Mali".to_string(),
          population: 19658023,
          geo_id: "ML".to_string(),
          country_code: "MLI".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 129,
          name: "Malta".to_string(),
          population: 493559,
          geo_id: "MT".to_string(),
          country_code: "MLT".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 130,
          name: "Marshall Islands".to_string(),
          population: 58791,
          geo_id: "MH".to_string(),
          country_code: "MHL".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 131,
          name: "Martinique".to_string(),
          population: 361225,
          geo_id: "MQ".to_string(),
          country_code: "MTQ".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 132,
          name: "Mauritania".to_string(),
          population: 4525698,
          geo_id: "MR".to_string(),
          country_code: "MRT".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 133,
          name: "Mauritius".to_string(),
          population: 1269670,
          geo_id: "MU".to_string(),
          country_code: "MUS".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 134,
          name: "Mayotte".to_string(),
          population: 256518,
          geo_id: "YT".to_string(),
          country_code: "MYT".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 135,
          name: "Mexico".to_string(),
          population: 127575529,
          geo_id: "MX".to_string(),
          country_code: "MEX".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 136,
          name: "Micronesia".to_string(),
          population: 115021,
          geo_id: "FM".to_string(),
          country_code: "FSM".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 173,
          name: "Moldova".to_string(),
          population: 4043258,
          geo_id: "MD".to_string(),
          country_code: "MDA".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 137,
          name: "Monaco".to_string(),
          population: 33085,
          geo_id: "MC".to_string(),
          country_code: "MCO".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 138,
          name: "Mongolia".to_string(),
          population: 3225166,
          geo_id: "MN".to_string(),
          country_code: "MNG".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 139,
          name: "Montenegro".to_string(),
          population: 622182,
          geo_id: "ME".to_string(),
          country_code: "MNE".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 140,
          name: "Montserrat".to_string(),
          population: 4991,
          geo_id: "MS".to_string(),
          country_code: "MSF".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 141,
          name: "Morocco".to_string(),
          population: 36471766,
          geo_id: "MA".to_string(),
          country_code: "MAR".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 142,
          name: "Mozambique".to_string(),
          population: 30366043,
          geo_id: "MZ".to_string(),
          country_code: "MOZ".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 143,
          name: "Myanmar".to_string(),
          population: 54045422,
          geo_id: "MM".to_string(),
          country_code: "MMR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 144,
          name: "Namibia".to_string(),
          population: 2494524,
          geo_id: "NA".to_string(),
          country_code: "NAM".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 145,
          name: "Nauru".to_string(),
          population: 11550,
          geo_id: "NR".to_string(),
          country_code: "NRU".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 146,
          name: "Nepal".to_string(),
          population: 28608715,
          geo_id: "NP".to_string(),
          country_code: "NPL".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 147,
          name: "Netherlands".to_string(),
          population: 17282163,
          geo_id: "NL".to_string(),
          country_code: "NLD".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 148,
          name: "New Caledonia".to_string(),
          population: 282757,
          geo_id: "NC".to_string(),
          country_code: "NCL".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 149,
          name: "New Zealand".to_string(),
          population: 4783062,
          geo_id: "NZ".to_string(),
          country_code: "NZL".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 150,
          name: "Nicaragua".to_string(),
          population: 6545503,
          geo_id: "NI".to_string(),
          country_code: "NIC".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 151,
          name: "Niger".to_string(),
          population: 23310719,
          geo_id: "NE".to_string(),
          country_code: "NER".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 152,
          name: "Nigeria".to_string(),
          population: 200963603,
          geo_id: "NG".to_string(),
          country_code: "NGA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 153,
          name: "Niue".to_string(),
          population: 1784,
          geo_id: "NU".to_string(),
          country_code: "NIU".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 154,
          name: "North Macedonia".to_string(),
          population: 2077132,
          geo_id: "MK".to_string(),
          country_code: "MKD".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 155,
          name: "Northern Mariana Islands".to_string(),
          population: 57213,
          geo_id: "MP".to_string(),
          country_code: "MNP".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 156,
          name: "Norway".to_string(),
          population: 5328212,
          geo_id: "NO".to_string(),
          country_code: "NOR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 157,
          name: "occupied Palestinian territory, including east Jerusalem".to_string(),
          population: 4981422,
          geo_id: "PS".to_string(),
          country_code: "PSE".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 158,
          name: "Oman".to_string(),
          population: 4974992,
          geo_id: "OM".to_string(),
          country_code: "OMN".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 159,
          name: "Other".to_string(),
          population: -1,
          geo_id: " ".to_string(),
          country_code: String::new(),
          continent: "Other".to_string()
        },
        Country {
          country_id: 160,
          name: "Pakistan".to_string(),
          population: 216565317,
          geo_id: "PK".to_string(),
          country_code: "PAK".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 161,
          name: "Palau".to_string(),
          population: 18092,
          geo_id: "PW".to_string(),
          country_code: "PLW".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 157,
          name: "Palestine".to_string(),
          population: 4981422,
          geo_id: "PS".to_string(),
          country_code: "PSE".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 162,
          name: "Panama".to_string(),
          population: 4246440,
          geo_id: "PA".to_string(),
          country_code: "PAN".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 163,
          name: "Papua New Guinea".to_string(),
          population: 8776119,
          geo_id: "PG".to_string(),
          country_code: "PNG".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 164,
          name: "Paraguay".to_string(),
          population: 7044639,
          geo_id: "PY".to_string(),
          country_code: "PRY".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 165,
          name: "Peru".to_string(),
          population: 32510462,
          geo_id: "PE".to_string(),
          country_code: "PER".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 166,
          name: "Philippines".to_string(),
          population: 108116622,
          geo_id: "PH".to_string(),
          country_code: "PHL".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 167,
          name: "Pitcairn Islands".to_string(),
          population: 40,
          geo_id: "PN".to_string(),
          country_code: "PCN".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 168,
          name: "Poland".to_string(),
          population: 37972812,
          geo_id: "PL".to_string(),
          country_code: "POL".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 169,
          name: "Portugal".to_string(),
          population: 10276617,
          geo_id: "PT".to_string(),
          country_code: "PRT".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 170,
          name: "Puerto Rico".to_string(),
          population: 2933404,
          geo_id: "PR".to_string(),
          country_code: "PRI".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 171,
          name: "Qatar".to_string(),
          population: 2832071,
          geo_id: "QA".to_string(),
          country_code: "QAT".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 172,
          name: "Republic of Korea".to_string(),
          population: 51225321,
          geo_id: "KR".to_string(),
          country_code: "KOR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 173,
          name: "Republic of Moldova".to_string(),
          population: 4043258,
          geo_id: "MD".to_string(),
          country_code: "MDA".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 174,
          name: "Réunion".to_string(),
          population: 863083,
          geo_id: "RE".to_string(),
          country_code: "REU".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 175,
          name: "Romania".to_string(),
          population: 19414458,
          geo_id: "RO".to_string(),
          country_code: "ROU".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 176,
          name: "Russia".to_string(),
          population: 145872260,
          geo_id: "RU".to_string(),
          country_code: "RUS".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 177,
          name: "Rwanda".to_string(),
          population: 12626938,
          geo_id: "RW".to_string(),
          country_code: "RWA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 178,
          name: "Saba".to_string(),
          population: 1918,
          geo_id: "XC".to_string(),
          country_code: "BES".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 179,
          name: "Saint Barthélemy".to_string(),
          population: 10457,
          geo_id: "BL".to_string(),
          country_code: "BLM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 180,
          name: "Saint Helena, Ascension and Tristan da Cunha".to_string(),
          population: 5633,
          geo_id: "SH".to_string(),
          country_code: "SHN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 181,
          name: "Saint Kitts and Nevis".to_string(),
          population: 52834,
          geo_id: "KN".to_string(),
          country_code: "KNA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 182,
          name: "Saint Lucia".to_string(),
          population: 182795,
          geo_id: "LC".to_string(),
          country_code: "LCA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 183,
          name: "Saint Martin".to_string(),
          population: 31801,
          geo_id: "MF".to_string(),
          country_code: "MAF".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 184,
          name: "Saint Pierre and Miquelon".to_string(),
          population: 5925,
          geo_id: "PM".to_string(),
          country_code: "SPM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 185,
          name: "Saint Vincent and the Grenadines".to_string(),
          population: 110593,
          geo_id: "VC".to_string(),
          country_code: "VCT".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 186,
          name: "Samoa".to_string(),
          population: 198410,
          geo_id: "WS".to_string(),
          country_code: "WSM".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 187,
          name: "San Marino".to_string(),
          population: 34453,
          geo_id: "SM".to_string(),
          country_code: "SMR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 188,
          name: "Sao Tome and Principe".to_string(),
          population: 215048,
          geo_id: "ST".to_string(),
          country_code: "STP".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 189,
          name: "Saudi Arabia".to_string(),
          population: 34268529,
          geo_id: "SA".to_string(),
          country_code: "SAU".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 190,
          name: "Senegal".to_string(),
          population: 16296362,
          geo_id: "SN".to_string(),
          country_code: "SEN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 191,
          name: "Serbia".to_string(),
          population: 6963764,
          geo_id: "RS".to_string(),
          country_code: "SRB".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 192,
          name: "Seychelles".to_string(),
          population: 97741,
          geo_id: "SC".to_string(),
          country_code: "SYC".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 193,
          name: "Sierra Leone".to_string(),
          population: 7813207,
          geo_id: "SL".to_string(),
          country_code: "SLE".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 194,
          name: "Singapore".to_string(),
          population: 5804343,
          geo_id: "SG".to_string(),
          country_code: "SGP".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 195,
          name: "Sint Eustatius".to_string(),
          population: 3142,
          geo_id: "XB".to_string(),
          country_code: "BES".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 196,
          name: "Sint Maarten".to_string(),
          population: 42389,
          geo_id: "SX".to_string(),
          country_code: "SXM".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 197,
          name: "Slovakia".to_string(),
          population: 5450421,
          geo_id: "SK".to_string(),
          country_code: "SVK".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 198,
          name: "Slovenia".to_string(),
          population: 2080908,
          geo_id: "SI".to_string(),
          country_code: "SVN".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 199,
          name: "Solomon Islands".to_string(),
          population: 669821,
          geo_id: "SB".to_string(),
          country_code: "SLB".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 200,
          name: "Somalia".to_string(),
          population: 15442906,
          geo_id: "SO".to_string(),
          country_code: "SOM".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 201,
          name: "South Africa".to_string(),
          population: 58558267,
          geo_id: "ZA".to_string(),
          country_code: "ZAF".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 172,
          name: "South Korea".to_string(),
          population: 51225321,
          geo_id: "KR".to_string(),
          country_code: "KOR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 202,
          name: "South Sudan".to_string(),
          population: 11062114,
          geo_id: "SS".to_string(),
          country_code: "SSD".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 203,
          name: "Spain".to_string(),
          population: 46937060,
          geo_id: "ES".to_string(),
          country_code: "ESP".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 204,
          name: "Sri Lanka".to_string(),
          population: 21323734,
          geo_id: "LK".to_string(),
          country_code: "LKA".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 205,
          name: "Sudan".to_string(),
          population: 42813237,
          geo_id: "SD".to_string(),
          country_code: "SDN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 206,
          name: "Suriname".to_string(),
          population: 581363,
          geo_id: "SR".to_string(),
          country_code: "SUR".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 207,
          name: "Sweden".to_string(),
          population: 10230185,
          geo_id: "SE".to_string(),
          country_code: "SWE".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 208,
          name: "Switzerland".to_string(),
          population: 8544527,
          geo_id: "CH".to_string(),
          country_code: "CHE".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 209,
          name: "Syria".to_string(),
          population: 17070132,
          geo_id: "SY".to_string(),
          country_code: "SYR".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 189,
          name: "Taiwan".to_string(),
          population: 23773881,
          geo_id: "TW".to_string(),
          country_code: "CNG1925".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 210,
          name: "Tajikistan".to_string(),
          population: 9321023,
          geo_id: "TJ".to_string(),
          country_code: "TJK".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 211,
          name: "Thailand".to_string(),
          population: 69625581,
          geo_id: "TH".to_string(),
          country_code: "THA".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 212,
          name: "The United Kingdom".to_string(),
          population: 66647112,
          geo_id: "GB".to_string(),
          country_code: "GBR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 213,
          name: "Timor Leste".to_string(),
          population: 1293120,
          geo_id: "TL".to_string(),
          country_code: "TLS".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 214,
          name: "Togo".to_string(),
          population: 8082359,
          geo_id: "TG".to_string(),
          country_code: "TGO".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 215,
          name: "Tokelau".to_string(),
          population: 1499,
          geo_id: "TK".to_string(),
          country_code: "TKL".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 216,
          name: "Tonga".to_string(),
          population: 105697,
          geo_id: "TO".to_string(),
          country_code: "TON".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 217,
          name: "Trinidad and Tobago".to_string(),
          population: 1394969,
          geo_id: "TT".to_string(),
          country_code: "TTO".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 218,
          name: "Tunisia".to_string(),
          population: 11694721,
          geo_id: "TN".to_string(),
          country_code: "TUN".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 219,
          name: "Turkey".to_string(),
          population: 82003882,
          geo_id: "TR".to_string(),
          country_code: "TUR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 220,
          name: "Turkmenistan".to_string(),
          population: 6430770,
          geo_id: "TM".to_string(),
          country_code: "TKM".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 221,
          name: "Turks and Caicos islands".to_string(),
          population: 38194,
          geo_id: "TC".to_string(),
          country_code: "TCA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 222,
          name: "Tuvalu".to_string(),
          population: 10507,
          geo_id: "TV".to_string(),
          country_code: "TUV".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 223,
          name: "Uganda".to_string(),
          population: 44269587,
          geo_id: "UG".to_string(),
          country_code: "UGA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 224,
          name: "Ukraine".to_string(),
          population: 43993643,
          geo_id: "UA".to_string(),
          country_code: "UKR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 225,
          name: "United Arab Emirates".to_string(),
          population: 9770526,
          geo_id: "AE".to_string(),
          country_code: "ARE".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 212,
          name: "United Kingdom".to_string(),
          population: 66647112,
          geo_id: "UK".to_string(),
          country_code: "GBR".to_string(),
          continent: "Europe".to_string()
        },
        Country {
          country_id: 226,
          name: "United Republic of Tanzania".to_string(),
          population: 58005461,
          geo_id: "TZ".to_string(),
          country_code: "TZA".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 227,
          name: "United States of America".to_string(),
          population: 329064917,
          geo_id: "US".to_string(),
          country_code: "USA".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 228,
          name: "United States Virgin Islands".to_string(),
          population: 104579,
          geo_id: "VI".to_string(),
          country_code: "VIR".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 229,
          name: "Uruguay".to_string(),
          population: 3461731,
          geo_id: "UY".to_string(),
          country_code: "URY".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 230,
          name: "Uzbekistan".to_string(),
          population: 32981715,
          geo_id: "UZ".to_string(),
          country_code: "UZB".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 231,
          name: "Vanuatu".to_string(),
          population: 299882,
          geo_id: "VU".to_string(),
          country_code: "VUT".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 232,
          name: "Venezuela".to_string(),
          population: 28515829,
          geo_id: "VE".to_string(),
          country_code: "VEN".to_string(),
          continent: "America".to_string()
        },
        Country {
          country_id: 233,
          name: "Vietnam".to_string(),
          population: 96462108,
          geo_id: "VN".to_string(),
          country_code: "VNM".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 234,
          name: "Wallis and Futuna".to_string(),
          population: -1,
          geo_id: "WF".to_string(),
          country_code: "WLF".to_string(),
          continent: "Oceania".to_string()
        },
        Country {
          country_id: 211,
          name: "Western Sahara".to_string(),
          population: 582458,
          geo_id: "EH".to_string(),
          country_code: "ESH".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 235,
          name: "Yemen".to_string(),
          population: 29161922,
          geo_id: "YE".to_string(),
          country_code: "YEM".to_string(),
          continent: "Asia".to_string()
        },
        Country {
          country_id: 236,
          name: "Zambia".to_string(),
          population: 17861034,
          geo_id: "ZM".to_string(),
          country_code: "ZMB".to_string(),
          continent: "Africa".to_string()
        },
        Country {
          country_id: 237,
          name: "Zimbabwe".to_string(),
          population: 14645473,
          geo_id: "ZW".to_string(),
          country_code: "ZWE".to_string(),
          continent: "Africa".to_string()
        }
      ]
    }
  }

  /**
   * Finds a country by its geo id (i. e. ISO 3166 two-letter code).
   *
   * Note that the search is case-sensitive, i. e. codes must be all upper case.
   * @return Returns the Country, if a match was found.
   *         Returns None, if no match was found.
   */
  pub fn find_by_geo_id(&self, geo_id: &str) -> Option<&Country>
  {
    self.all_countries.iter().find(|c| c.geo_id == geo_id)
  }
}
