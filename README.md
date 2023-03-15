**KN Sim Parser**

This is a simple Rust utilty that parses the `.dat` spectrum files from [Wollaeger et al 2021](https://ui.adsabs.harvard.edu/abs/2021ApJ...918...10W/abstract) into individual HDF5 files.

The data files are obtained from [https://zenodo.org/record/7335961](https://zenodo.org/record/7335961).

The parser does no transformations or unit conversions on the file data. It only parses the UTF-encoded numerical data in the input files into single precision floating point (f32) values, and dumps these into multidimensional arrays in the HDF5 files.

*** Build ***

Build the project with `cargo` as normal for Rust packages.

This package requires a local installation of an HDF5 library for the Rust `hdf5` crate to compile against.  If the automatic build process cannot locate the library, it can be directed by setting the `HDF5_DIR` environment variable.  See the `hdf5` Rust crate documenation for more details [https://docs.rs/hdf5/latest/hdf5/](https://docs.rs/hdf5/latest/hdf5/).

*** Output Data Layout ***

The output HDF5 file has 10 fields, all Datasets:
- `topo`: Integer, i32, 0 ("S") or 1 ("P"), denoting the topology of the simulation.
- `wind`: Integer, i32, 1 or 2, denoting the type of lanthanide wind.
- `md_Msolar`: float, f64, mass of dynamic ejecta in Solar Masses.
- `vd_c`: float, f64, speed of dynamic ejecta in c.
- `mw_Msolar`: float, f64, mass of wind ejecta in Solar Masses.
- `vw_c`: float, f64, speed of wind ejecta in c.
- `t_days`: array, size `[Nt]`, f32. Time of fluxes in days.
- `lambda_cm`: array, size `[Nl, 2]`, f32.  Wavelength bins (lower then upper).
- `theta_rad`: array, size `[Nv, 2]`, f32.  Viewing angle bins (lower then upper).
- `fla_cgs_per_angstrom`: array, size `[Nt, Nl, Nv]`, spectral flux density f_lambda in (erg / (s cm^2 A))



