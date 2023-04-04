/// Give a high level interface for users
mod general;
/// Parse mmCIF/PDBx files
mod mmcif;
/// Parse PDB files
mod pdb;
use super::{check_extension, check_extension2};

pub use general::{open, open_raw};
pub use mmcif::{open_mmcif, open_mmcif_gz, open_mmcif_raw};
pub use pdb::{open_pdb, open_pdb_gz, open_pdb_raw};
