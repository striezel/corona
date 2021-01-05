/*
 -------------------------------------------------------------------------------
    This file is part of the Corona numbers website generator.
    Copyright (C) 2021  Dirk Stolle
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

mod australia;
mod fiji;
mod french_polynesia;
// TODO: Guam: Cannot find separate numbers for Guam in the API.
mod marshall_islands;
mod new_caledonia;
mod new_zealand;
// TODO: Northern Mariana Islands: Cannot find separate numbers for them in the API.
mod papua_new_guinea;
mod solomon_islands;
mod vanuatu;
mod wallis_and_futuna;

pub use australia::Australia;
pub use fiji::Fiji;
pub use french_polynesia::FrenchPolynesia;
pub use marshall_islands::MarshallIslands;
pub use new_caledonia::NewCaledonia;
pub use new_zealand::NewZealand;
pub use papua_new_guinea::PapuaNewGuinea;
pub use solomon_islands::SolomonIslands;
pub use vanuatu::Vanuatu;
pub use wallis_and_futuna::WallisAndFutuna;
