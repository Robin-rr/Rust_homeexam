# Lab assignment

The file f401_minimal.rs under /examples contains changes to complete the two warm-up assignments.
To run said file, from the root of the project run "openocd -f openocd.cfg" in a seperate terminal and then run:
"cargo run --example f401_minimal --features f4 --target thumbv7em-none-eabihf"

# Home exam

The files generate.rs and common.rs under /runner/src and /runner/src/bin contains the implementation to reach grade 3 of the home exam.
To run the home exam, from the /runner folder of the project run "cargo run --bin generate"

Note: To run different Task sets paste the new taskset instead of the old one,
To change preemtion calculation between approximate and exact, two lines must be changed.
Line 216 and 238 should both contain the string "exact" or "approx" to switch between the two calculations.

---

### Disclaimer

This is a home-exam at LTU (Lulea university of technology), there will not be any further development of this repository.

### Original Disclaimer

This project is in early development, thus expect bugs and shortcomings, no API stability offered or guaranteed. It is however well suited for experimentation and all implemented features have been successfully tested.

---

## Licencse

Copyright Per Lindgren.

All rights reserved, use restricted for non-commercial purpose.
